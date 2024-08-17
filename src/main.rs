// ANCHOR: imports_main
// ANCHOR: declare_mods
/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;
// ANCHOR_END: declare_mods
use app::App;
use clap::Parser;
use color_eyre::Result;
// use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;
use crossterm::event::{self as my_event};

// use tokio::task;

// ANCHOR_END: imports_main


// ANCHOR: main
#[tokio::main]
async fn main() -> Result<()> {

    // Create an application.
    let mut app = App::new(app::Args::parse()).await;

    // let mut app_clone = app.clone(); // Clone the app for the task
    // task::spawn(async move {
        app.fetch_ps_data().await;
    // });

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;

    // let events = EventHandler::new(10);
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    // Start the main loop.
    while !&app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;

        // Handle events.

        if let Ok(event) = my_event::read() {
            match event {
                my_event::Event::Key(key_event) => update(&mut app, key_event, &mut tui).await,
                _ => {}
            }
        }
        // if !app.is_vim_open {
        //     match events.next()? {
        //         Event::Tick => {}
        //         Event::Key(key_event) => update(&mut app, key_event, &mut tui),
        //         Event::Mouse(_) => {}
        //         Event::Resize(_, _) => {}
        //     };
        // }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
// ANCHOR_END: main
