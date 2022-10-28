pub mod record;

use anyhow::Result;
use record::Record;
#[cfg(feature = "swf")]
use rrr_chart::swf::{Compressed, SwfParser};
use std::fmt::Debug;

pub enum SourceType {
    SWF,
    BIN,
    SM,
}

#[cfg(feature = "swf")]
pub struct SwfChart {
    pub parser: SwfParser<Compressed>,
}

#[cfg(feature = "sm")]
#[derive(Debug, Clone)]
struct SmChart();
#[cfg(feature = "bin")]
#[derive(Debug, Clone)]
struct BinChart();

pub struct Press<S: PressType> {
    pub s: S,
}
pub trait PressType {}
#[cfg(feature = "swf")]
impl PressType for SwfChart {}
#[cfg(feature = "sm")]
impl PressType for SmChart {}
#[cfg(feature = "bin")]
impl PressType for BinChart {}

#[derive(Debug, Clone)]
pub struct RecordPressBuilder {}

impl RecordPressBuilder {
    #[cfg(feature = "swf")]
    #[must_use]
    pub fn from_swf(swf: Vec<u8>) -> Press<SwfChart> {
        let parser = SwfParser::new(swf);
        Press {
            s: SwfChart { parser },
        }
    }

    #[cfg(feature = "bin")]
    #[must_use]
    pub fn from_bin() {}

    #[cfg(feature = "sm")]
    #[must_use]
    pub fn from_sm() {}
}

impl Press<SwfChart> {
    pub fn press(self) -> Result<Record> {
        let parser_result = self.s.parser.decompress();

        let parser = match parser_result {
            Ok(parser) => parser,
            Err(e) => return Err(anyhow::anyhow!("Decompression Error: {}", e)),
        };

        let mut parsing = parser.parse();

        // Availalable so we don't tank frame performance.
        let _state = parsing.tick();
        let parsed = parsing.finish();

        let chart = parsed.consume();
        Record::new(chart.mp3, chart.chart)
    }
}
