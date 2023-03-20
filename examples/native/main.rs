use anyhow::Result;
use rrr_config::Config;
use rrr_fetch::{platform::Fetcher, FetchProgress};
use rrr_game::{
    builder::RustRustRevolutionBuilder,
    prelude::{rrr_render, Play, Turntable},
};
use rrr_record::RecordPressBuilder;
use rrr_window::{prelude::EventLoopBuilder, Window};

pub fn main() -> Result<()> {
    if simple_logger::init().is_err() {
        return Err(anyhow::anyhow!(
            "Could not initialize simple_logger, quitting."
        ));
    }

    let url = format!(
            "https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&mode=2&type=ChartFFR_music",
            "a054ce01d88f3cba3bc98f70d71b3278"
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
    let renderer = futures::executor::block_on(
        rrr_render::RendererBuilder::new(config.width, config.height, &window.window).build(),
    )?;

    let turntable = Turntable::load(record.unwrap());
    let play = Play::new(turntable);

    let mut rrr = RustRustRevolutionBuilder::with_play(play)
        .with_renderer(renderer)
        .build();
    window.run_once(&mut rrr);
    Ok(())
}
