use anyhow::Result;
use clap::Parser;
use rrr_config::Config;
use rrr_fetch::{platform::Fetcher, FetchProgress};
use rrr_game::{
    builder::RustRustRevolutionBuilder,
    prelude::{rrr_render, Play, Turntable},
};
use rrr_record::RecordPressBuilder;
use rrr_window::{prelude::EventLoopBuilder, Window};

#[derive(Debug, Parser)]
pub struct Args {
    /// ID of song to play
    song_id: u16,
}

impl Args {
    pub fn run(&self) -> Result<()> {
        if simple_logger::init().is_err() {
            return Err(anyhow::anyhow!(
                "Could not initialize simple_logger, quitting."
            ));
        }

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

        let data = if let Ok(fetcher) = fetcher {
            fetcher.consume()
        } else {
            return Err(anyhow::anyhow!("Failed to fetch."));
        };

        let record_press = RecordPressBuilder::from_swf(data);
        let record = record_press.press();

        let config = Config::default();
        let mut event_loop = EventLoopBuilder::new().build();
        let mut window = Window::new(config, &mut event_loop)?;
        let renderer = futures::executor::block_on(async {
            rrr_render::RendererBuilder::new(config.width, config.height, &window.window)
                .build()
                .await
        })?;

        let turntable = Turntable::load(record.unwrap());
        let play = Play::new(turntable);

        let mut rrr = RustRustRevolutionBuilder::with_play(play)
            .with_renderer(renderer)
            .build();
        window.run_once(&mut rrr);
        Ok(())
    }
}
