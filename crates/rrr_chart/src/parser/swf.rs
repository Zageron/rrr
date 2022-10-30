use crate::{chart_impl::RuntimeChart, NoteColor, RuntimeNote};
use ::swf::{
    avm1::{
        self,
        types::{ConstantPool, Value},
    },
    read::Reader,
    SwfBuf, UTF_8,
};
use rrr_types::Direction;
use std::ops::ControlFlow;
use swf::SwfStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum ChartParseError {
    #[error("Invalid beat position in chart.")]
    BeatPosition,

    #[error("Invalid direction in chart.")]
    NoteDirection,

    #[error("Invalid note color in chart.")]
    NoteColor,

    #[error("Invalid timestamp in chart.")]
    Timestamp,
}

pub struct SwfParser<S: SwfParserState> {
    state: S,
}

pub struct Compressed {
    raw_swf: Vec<u8>,
}

pub struct ReadyToParse {
    stream: SwfBuf,
}

pub struct Parsing {
    stream: SwfBuf,
    mp3: Vec<u8>,
    chart: Vec<RuntimeNote>,
}

#[allow(unused)]
pub struct Parsed {
    pub mp3: Vec<u8>,
    pub chart: RuntimeChart,
}

pub enum ParsingState {
    Parsing,
    Finished,
}

pub trait SwfParserState {}
impl SwfParserState for Compressed {}
impl SwfParserState for ReadyToParse {}
impl SwfParserState for Parsing {}
impl SwfParserState for Parsed {}

impl SwfParser<Compressed> {
    #[must_use]
    pub fn new(swf_file: Vec<u8>) -> Self {
        Self {
            state: Compressed { raw_swf: swf_file },
        }
    }

    /// # Errors
    ///
    /// Will return `swf::error::Error` if `swf_file` is not a valid swf binary slice.
    pub fn decompress(self) -> anyhow::Result<SwfParser<ReadyToParse>, swf::error::Error> {
        let stream = swf::decompress_swf(self.state.raw_swf.as_slice())?;
        Ok(SwfParser {
            state: ReadyToParse { stream },
        })
    }
}

impl SwfParser<ReadyToParse> {
    #[must_use]
    pub fn parse(self) -> SwfParser<Parsing> {
        SwfParser {
            state: Parsing {
                stream: self.state.stream,
                mp3: Vec::new(),
                chart: Vec::new(),
            },
        }
    }
}

// TODO: Make this parse function async.
impl SwfParser<Parsing> {
    #[must_use]
    pub fn tick(&mut self) -> ParsingState {
        let mut swf_reader = Reader::new(
            &self.state.stream.data[..],
            self.state.stream.header.version(),
        );
        while let Ok(tag) = swf_reader.read_tag() {
            match tag {
                // This is for files that do not have a block of audio at the front.
                swf::Tag::DefineSound(sound) => {
                    //println!("DefineSound: {:?}", sound)
                    self.state.mp3.extend_from_slice(sound.data);
                }

                swf::Tag::DoAction(action) => {
                    let res = SwfParser::parse_action(action, swf_reader.version());
                    match res {
                        Ok(chart) => self.state.chart = chart,
                        Err(e) => println!("Error when parsing the swf: {}", e),
                    }
                }

                // One Shot of all of the audio
                // TODO: Bug: If the length of the tag - 4 is 0, it is invalid.
                swf::Tag::SoundStreamBlock(sound) => {
                    if sound.len() - 4 == 0 {
                        log::error!("No song data, this is invalid!")
                    }
                    self.state.mp3.extend_from_slice(sound);
                }

                // This is audio metadata.
                swf::Tag::SoundStreamHead(ssh) => {
                    log::info!("SoundStreamHead");
                    log::info!("latency seek: {}", ssh.latency_seek);
                    log::info!("playback format: {:?}", ssh.playback_format);
                    log::info!("num samples per block: {}", ssh.num_samples_per_block);
                    log::info!("stream format: {:?}", ssh.stream_format);
                }
                swf::Tag::SoundStreamHead2(ssh) => {
                    log::info!("SoundStreamHead");
                    log::info!("latency seek: {}", ssh.latency_seek);
                    log::info!("playback format: {:?}", ssh.playback_format);
                    log::info!("num samples per block: {}", ssh.num_samples_per_block);
                    log::info!("stream format: {:?}", ssh.stream_format);
                }
                _ => {}
            }
        }

        ParsingState::Finished
    }

    pub fn finish(self) -> SwfParser<Parsed> {
        SwfParser {
            state: Parsed {
                mp3: self.state.mp3,
                chart: RuntimeChart::new(&self.state.chart),
            },
        }
    }

    fn parse_action(action_raw: &[u8], version: u8) -> anyhow::Result<Vec<RuntimeNote>> {
        let mut action_reader = avm1::read::Reader::new(action_raw, version);
        let mut constant_pool: Option<ConstantPool<'_>> = None;
        let mut value_stack: Vec<Value<'_>> = Vec::with_capacity(6);
        let mut beat_box: Vec<RuntimeNote> = Vec::new();

        let mut done = false;
        while !done {
            if let Ok(action) = action_reader.read_action() {
                match action {
                    avm1::types::Action::ConstantPool(cp) => {
                        let is_chart_data = cp.strings.contains(&SwfStr::from_utf8_str("beatBox"));
                        if is_chart_data {
                            constant_pool.replace(cp);
                        } else {
                            break;
                        }
                    }

                    avm1::types::Action::Push(mut push_object) => {
                        if let ControlFlow::Break(_) =
                            parse_push_action(&mut push_object, &mut value_stack)
                        {
                            continue;
                        }
                    }

                    avm1::types::Action::End => {
                        done = true;
                    }

                    avm1::types::Action::GetVariable => {
                        // Pop the last item on the stack and do something with it.
                    }

                    avm1::types::Action::InitArray => {
                        // Ignore the first `InitArray`, data at this point is garbage.
                        if value_stack.is_empty() {
                            continue;
                        }

                        let items_to_pop = if let Some(Value::Int(length)) = value_stack.pop() {
                            log::info!("Number of items to create init for {}", length);
                            length
                        } else {
                            anyhow::bail!("There was not an int on the end so you broke it.")
                        };

                        if value_stack.len() < items_to_pop as usize {
                            log::info!("We are done.");
                            // We're done!
                            break; // Probably
                        }

                        let beat_position = parse_beat_position(&mut value_stack)?;
                        let direction = parse_direction(&mut value_stack, &constant_pool)?;

                        let color = match items_to_pop > 2 {
                            true => parse_color(&mut value_stack, &constant_pool)?,
                            false => NoteColor::Blue,
                        };

                        let timestamp = match items_to_pop > 3 {
                            true => parse_timestamp(&mut value_stack)?,
                            false => beat_position / 30 * 1000,
                        };

                        beat_box.push(RuntimeNote {
                            beat_position,
                            direction,
                            color,
                            timestamp,
                        });
                    }

                    avm1::types::Action::SetMember => {}

                    _ => {
                        log::error!("Unexpectedly unhandled action: {:?}", action);
                    }
                }
            }
        }

        if beat_box.is_empty() {
            anyhow::bail!("Not chart data.");
        }

        Ok(beat_box)
    }
}

impl SwfParser<Parsed> {
    #[must_use]
    pub fn consume(self) -> Parsed {
        self.state
    }
}

fn parse_timestamp(value_stack: &mut Vec<Value<'_>>) -> anyhow::Result<u32> {
    if let Some(Value::Int(ms)) = value_stack.pop() {
        Ok(ms.unsigned_abs())
    } else {
        anyhow::bail!(ChartParseError::Timestamp);
    }
}

fn parse_color(
    value_stack: &mut Vec<Value<'_>>,
    constant_pool: &Option<ConstantPool<'_>>,
) -> anyhow::Result<NoteColor> {
    if let Some(Value::ConstantPool(color)) = value_stack.pop() {
        match constant_pool.clone().unwrap().strings[color as usize]
            .to_str_lossy(UTF_8)
            .to_string()
            .as_str()
        {
            "red" => Ok(NoteColor::Red),
            "yellow" => Ok(NoteColor::Yellow),
            "blue" => Ok(NoteColor::Blue),
            "orange" => Ok(NoteColor::Orange),
            "green" => Ok(NoteColor::Green),
            "pink" => Ok(NoteColor::Pink),
            "purple" => Ok(NoteColor::Purple),
            "cyan" => Ok(NoteColor::Cyan),
            "white" => Ok(NoteColor::White),
            _ => anyhow::bail!(ChartParseError::NoteColor),
        }
    } else {
        anyhow::bail!(ChartParseError::NoteColor);
    }
}

fn parse_direction(
    value_stack: &mut Vec<Value<'_>>,
    constant_pool: &Option<ConstantPool<'_>>,
) -> anyhow::Result<Direction> {
    if let Some(Value::ConstantPool(dir)) = value_stack.pop() {
        match constant_pool.clone().unwrap().strings[dir as usize]
            .to_str_lossy(UTF_8)
            .to_string()
            .as_str()
        {
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "R" => Ok(Direction::Right),
            _ => anyhow::bail!(ChartParseError::NoteDirection),
        }
    } else {
        anyhow::bail!(ChartParseError::NoteDirection);
    }
}

fn parse_beat_position(value_stack: &mut Vec<Value<'_>>) -> anyhow::Result<u32> {
    if let Some(Value::Int(ms)) = value_stack.pop() {
        Ok(ms.unsigned_abs())
    } else {
        log::error!("No beat position found");
        anyhow::bail!(ChartParseError::BeatPosition);
    }
}

fn parse_push_action<'a>(
    pushed_objects: &mut avm1::types::Push<'a>,
    value_stack: &mut Vec<Value<'a>>,
) -> ControlFlow<()> {
    for object in pushed_objects.values.clone() {
        value_stack.push(object);
    }

    ControlFlow::Continue(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NoteColor, RuntimeChart};

    use super::SwfParser;
    use anyhow::{self, Result};

    fn parse_chart(raw_swf: &[u8]) -> Result<Parsed, swf::error::Error> {
        if simple_logger::init().is_err() {
            println!("error");
            assert!(false)
        }

        let mut vec = Vec::<u8>::new();
        vec.extend_from_slice(raw_swf);

        let parser = SwfParser::new(vec);
        let parser_decomp_result = parser.decompress()?;

        let mut parsing = parser_decomp_result.parse();

        let _state = parsing.tick();
        let parsed = parsing.finish();

        let chart = parsed.consume();
        Ok(chart)
    }

    #[test]
    pub fn test_parse_2_cell_chart() -> Result<(), swf::error::Error> {
        let swf = include_bytes!("./test_assets/test_2.swf");
        let chart = parse_chart(swf)?;
        assert!(chart.chart.notes[0].color == NoteColor::Blue);
        Ok(())
    }

    #[test]
    pub fn test_parse_3_cell_chart() -> Result<(), swf::error::Error> {
        let swf = include_bytes!("./test_assets/test_3.swf");
        let chart = parse_chart(swf)?;
        assert!(chart.chart.notes[0].color != NoteColor::Blue);
        Ok(())
    }

    #[test]
    pub fn test_parse_4_cell_chart() -> Result<(), swf::error::Error> {
        let swf = include_bytes!("./test_assets/test_4.swf");
        let chart = parse_chart(swf)?;
        assert!(chart.chart.notes[0].color != NoteColor::Blue);
        Ok(())
    }

    #[test]
    pub fn test_parse_block_audio() -> Result<(), swf::error::Error> {
        let swf = include_bytes!("./test_assets/test_4_block.swf");
        let chart = parse_chart(swf)?;
        assert!(chart.mp3.len() > 0);
        Ok(())
    }
}
