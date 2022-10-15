use anyhow::Result;
use rrr_config::Config;
use rrr_game::{
    prelude::{rrr_render, SongID},
    RustRustRevolutionBuilder,
};
use rrr_window::{prelude::EventLoopBuilder, Window};

pub fn main() -> Result<()> {
    let song_id = SongID(100);
    let config = Config::default();
    let mut event_loop = EventLoopBuilder::new().build();
    let mut window = Window::new(config, &mut event_loop)?;
    let renderer = futures::executor::block_on(async {
        rrr_render::RendererBuilder::new(config.width, config.height, &window.window)
            .build()
            .await
    })?;
    let mut rrr = RustRustRevolutionBuilder::with_renderer(renderer).build(song_id);
    window.run_once(&mut rrr);
    Ok(())
}
