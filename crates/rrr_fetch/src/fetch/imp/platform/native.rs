#![allow(dead_code)]

use crate::FetchProgress;
use anyhow::Result;
use std::fmt::Debug;
use std::io::Read;

pub struct Fetcher {
    url: String,
    reader: Box<dyn Read + Send + Sync + 'static>,
    bytes: Vec<u8>,
}

impl Debug for Fetcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fetcher")
            .field("url", &self.url)
            .field("bytes", &self.bytes.capacity())
            .finish()
    }
}

impl Fetcher {
    pub fn new(url: String) -> Result<Self> {
        let response = ureq::get(&url).call()?;
        let len = if let Some(len) = response.header("Content-Length") {
            len.parse()?
        } else {
            return Err(anyhow::anyhow!("No content length."));
        };

        Ok(Self {
            url,
            reader: response.into_reader(),
            bytes: Vec::with_capacity(len),
        })
    }

    pub fn fetch(&mut self) -> Result<FetchProgress> {
        let reader = self.reader.as_mut();
        reader.take(1000).read_to_end(&mut self.bytes)?;

        if self.bytes.len() >= self.bytes.capacity() {
            Ok(FetchProgress::Finished)
        } else {
            Ok(FetchProgress::Fetching(
                (self.bytes.len() as f32 / self.bytes.capacity() as f32) * 100.,
            ))
        }
    }

    pub fn consume(self) -> Vec<u8> {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetch::PlaylistPayload;

    #[test]
    fn test_fetch() {
        let url = format!(
            "https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&type=ChartFFR_music",
            "8e058c682f875b36fac6fa065c33fd88"
        );

        let mut fetcher = Fetcher::new(url);

        assert!(fetcher.is_ok(), "{:?}", fetcher.err());

        if let Ok(fetcher) = fetcher.as_mut() {
            loop {
                let progress = fetcher.fetch();
                if let Ok(progress) = progress {
                    match progress {
                        FetchProgress::Fetching(percent) => println!("%{:?} complete", percent),
                        FetchProgress::Finished => break,
                        FetchProgress::Error(_) => todo!(),
                    }
                }
            }
        }
    }

    #[test]
    fn test_fetch_stream() {
        let mut fetcher = Fetcher::new(
            "https://www.flashflashrevolution.com/game/r3/r3-playlist.v2.php".to_string(),
        );

        assert!(fetcher.is_ok(), "{:?}", fetcher.err());

        if let Ok(fetcher) = fetcher.as_mut() {
            loop {
                let progress = fetcher.fetch();
                if let Ok(progress) = progress {
                    match progress {
                        FetchProgress::Fetching(percent) => println!("%{:?} complete", percent),
                        FetchProgress::Finished => break,
                        FetchProgress::Error(_) => todo!(),
                    }
                }
            }
        }

        if let Ok(fetcher) = fetcher {
            let data = fetcher.consume();
            let _playlist: PlaylistPayload = serde_json::from_slice(&data).unwrap();
        }
    }
}
