#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::note::{NoteRow, RuntimeNote};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// A runtime efficient representation of a chart used by an [RRR](crate::RRR) instance.
pub struct RuntimeChart {
    pub notes: Vec<RuntimeNote>,
}

impl RuntimeChart {
    #[must_use]
    pub fn new(notes: &[RuntimeNote]) -> Self {
        Self {
            notes: notes.to_vec(),
        }
    }

    /// # Errors
    ///
    /// Will return `anyhow::Error` if there is not at least 1 note in the chart.
    pub fn get_duration(&self) -> Result<u32, anyhow::Error> {
        if let Some(last_note) = self.notes.first() {
            Ok(last_note.timestamp)
        } else {
            Err(anyhow::anyhow!("No notes in chart"))
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
/// Stores all of the [note rows](NoteRow) that represent a beat.
pub struct Beat {
    note_rows: Vec<NoteRow>,
    subdivisions: u32,
}

impl Beat {
    #[must_use]
    pub fn new(note_rows: &[NoteRow], subdivisions: u32) -> Self {
        Self {
            note_rows: note_rows.into(),
            subdivisions,
        }
    }
}
