use crate::{chart_impl::RuntimeChart, NoteColor, NoteDirection, RuntimeNote};
use std::ops::ControlFlow;
use swf::{
    avm1::{
        self,
        types::{ConstantPool, Value},
    },
    read::Reader,
    SwfBuf, UTF_8,
};
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
    mp3: Vec<u8>,
    chart: RuntimeChart,
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
                swf::Tag::DefineSound(_) => log::info!("DefineSound"),
                swf::Tag::DoAction(action) => {
                    let res = SwfParser::parse_action(action, swf_reader.version());
                    if let Ok(chart) = res {
                        self.state.chart = chart;
                    }
                }
                swf::Tag::SoundStreamBlock(sound) => {
                    self.state.mp3.extend_from_slice(sound);
                }
                swf::Tag::SoundStreamHead(ssh) => {
                    log::info!("SoundStreamHead");
                    log::info!("latency seek: {}", ssh.latency_seek);
                    log::info!("playback format: {:?}", ssh.playback_format);
                    log::info!("num samples per block: {}", ssh.num_samples_per_block);
                    log::info!("stream format: {:?}", ssh.stream_format);
                }
                swf::Tag::SoundStreamHead2(_) => log::info!("SoundStreamHead2"),
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
        let mut is_chart_data = false;
        let mut constant_pool: Option<ConstantPool<'_>> = None;
        let mut value_stack: Vec<Value<'_>> = Vec::with_capacity(4);
        let mut beat_box: Vec<RuntimeNote> = Vec::new();

        let mut done = false;
        while !done {
            if let Ok(action) = action_reader.read_action() {
                match action {
                    avm1::types::Action::ConstantPool(cp) => {
                        constant_pool.replace(cp);
                    }

                    avm1::types::Action::Push(mut push_object) => {
                        if let ControlFlow::Break(_) =
                            parse_push_action(is_chart_data, &mut push_object, &mut value_stack)
                        {
                            continue;
                        }
                    }

                    avm1::types::Action::End | avm1::types::Action::Stop => {
                        done = true;
                    }

                    avm1::types::Action::GetVariable => {
                        is_chart_data = true;
                    }

                    avm1::types::Action::InitArray => {
                        // Ignore the first `InitArray`, data at this point is garbage.
                        if value_stack.is_empty() {
                            continue;
                        }

                        let beat_position = parse_beat_position(&mut value_stack);
                        let direction = parse_direction(&mut value_stack, &constant_pool);
                        let color = parse_color(&mut value_stack, &constant_pool);
                        let timestamp = parse_timestamp(&mut value_stack);

                        if let (Ok(bp), Ok(dir), Ok(col), Ok(ts)) =
                            (beat_position, direction, color, timestamp)
                        {
                            beat_box.push(RuntimeNote {
                                beat_position: bp,
                                direction: dir,
                                color: col,
                                timestamp: ts,
                            });
                        } else {
                            anyhow::bail!(ChartParseError::Timestamp);
                        }
                    }

                    avm1::types::Action::SetMember => {
                        is_chart_data = false;
                    }

                    _ => {
                        log::error!("Unexpectedly unhandled action: {:?}", action);
                    }
                }
            }
        }

        if beat_box.is_empty() {
            if is_chart_data {
                anyhow::bail!(ChartParseError::BeatPosition);
            }

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
            _ => anyhow::bail!(ChartParseError::NoteColor),
        }
    } else {
        anyhow::bail!(ChartParseError::NoteColor);
    }
}

fn parse_direction(
    value_stack: &mut Vec<Value<'_>>,
    constant_pool: &Option<ConstantPool<'_>>,
) -> anyhow::Result<NoteDirection> {
    if let Some(Value::ConstantPool(dir)) = value_stack.pop() {
        match constant_pool.clone().unwrap().strings[dir as usize]
            .to_str_lossy(UTF_8)
            .to_string()
            .as_str()
        {
            "L" => Ok(NoteDirection::Left),
            "U" => Ok(NoteDirection::Up),
            "D" => Ok(NoteDirection::Down),
            "R" => Ok(NoteDirection::Right),
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
    is_chart_data: bool,
    push_object: &mut avm1::types::Push<'a>,
    value_stack: &mut Vec<Value<'a>>,
) -> ControlFlow<()> {
    if !is_chart_data {
        return ControlFlow::Break(());
    }

    if push_object.values.len() < 2 {
        return ControlFlow::Break(());
    }

    let real_size = push_object.values.pop();
    let total_size = push_object.values.len();
    let garbage = if let Some(Value::Int(len)) = real_size {
        total_size.checked_sub(len.try_into().unwrap()).unwrap()
    } else {
        0
    };

    for i in garbage..total_size {
        value_stack.push(push_object.values.get(i).unwrap().clone());
    }

    ControlFlow::Continue(())
}
