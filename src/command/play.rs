use anyhow::Result;
use clap::Parser;
use rrr_config::Config;
use rrr_game::{prelude::rrr_render, prelude::SongID, RustRustRevolutionBuilder};
use rrr_window::{prelude::EventLoopBuilder, Window};

#[derive(Debug, Parser)]
pub struct Args {
    /// ID of song to play
    song_id: u16,
}

impl Args {
    pub fn run(&self) -> Result<()> {
        let config = Config::default();
        let mut event_loop = EventLoopBuilder::new().build();
        let mut window = Window::new(config, &mut event_loop)?;
        let renderer = futures::executor::block_on(async {
            rrr_render::RendererBuilder::new(config.width, config.height, &window.window)
                .build()
                .await
        })?;
        let mut rrr =
            RustRustRevolutionBuilder::with_renderer(renderer).build(SongID(self.song_id));
        window.run_once(&mut rrr);
        Ok(())
    }
}
