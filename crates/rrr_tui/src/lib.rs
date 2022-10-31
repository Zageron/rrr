use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rrr_config::Config;
use rrr_fetch::{platform::Fetcher, FetchProgress};
use rrr_game::{
    builder::RustRustRevolutionBuilder,
    prelude::{Play, Turntable},
};
use rrr_record::RecordPressBuilder;
use rrr_window::{
    prelude::{EventLoopBuilder, EventLoopExtRunReturn},
    Window,
};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
    sender: Sender<u16>,
}

impl<'a> App<'a> {
    fn new(sender: Sender<u16>) -> App<'a> {
        App {
            items: StatefulList::with_items(vec![("Song 1", 1), ("Song 2", 2)]),
            sender,
        }
    }

    /// Rotate through the event list.
    /// This only exists to simulate some kind of "progress"
    fn on_tick(&mut self) {}
}

pub fn init() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let terminal_join = thread::spawn(move || {
        // setup terminal
        let _res = enable_raw_mode();
        let mut stdout = io::stdout();
        let _res = execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let app = App::new(tx);
        let res = run_app(&mut terminal, app, tick_rate);

        // restore terminal
        let _res = disable_raw_mode();
        let _res = execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _res = terminal.show_cursor();

        if let Err(err) = res {
            println!("{:?}", err)
        }
    });

    let mut event_loop = EventLoopBuilder::new().build();
    loop {
        if let Ok(_song_id) = rx.try_recv() {
            {
                let url = format!(
                    "https://www.flashflashrevolution.com/game/r3/r3-songLoad.php?id={}&mode=2&type=ChartFFR_music",
                    "f9b50c8a00667e711ff63ed2cd944f54"
                );

                let mut fetcher = Fetcher::new(url);

                assert!(fetcher.is_ok(), "{:?}", fetcher.err());

                if let Ok(fetcher) = fetcher.as_mut() {
                    loop {
                        let progress = fetcher.fetch();
                        if let Ok(progress) = progress {
                            match progress {
                                FetchProgress::Fetching(percent) => {
                                    println!("%{:?} complete", percent)
                                }
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
            }

            // Make sure window is dropped by running run_return again.
            event_loop.run_return(move |_event, _, control_flow| {
                control_flow.set_exit();
            });
        }

        if terminal_join.is_finished() {
            break;
        }
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    app.items.next();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    KeyCode::Enter => {
                        let song_id = app.items.state.selected().unwrap().try_into().unwrap();
                        let _res = app.sender.send(song_id);
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(i.0)];
            for _ in 0..i.1 {
                lines.push(Spans::from(Span::styled(
                    "Song Name",
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
            }
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    let text = Spans::from("Select a song and press enter to play.");
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Instruction"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[1]);
}
