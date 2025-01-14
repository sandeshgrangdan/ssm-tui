// ANCHOR: imports_main
// ANCHOR: declare_mods
/// Application.
pub mod app;
/// Terminal events handler.
pub mod event;
/// Terminal user interface.
pub mod tui;
/// Widget renderer.
pub mod ui;
pub mod update;

/// Application updater.
// ANCHOR_END: declare_mods
use app::App;
use clap::Parser;
use color_eyre::Result;
// use event::{Event, EventHandler};
use crossterm::event::{self as my_event};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;
// use tokio::task;
// use tokio::sync::mpsc;
// ANCHOR_END: imports_main

// ANCHOR: main
#[tokio::main]
async fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new(app::Args::parse());
    app.set_ssm_client().await;

    app.fetch_ps_data().await;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;

    // let events = EventHandler::new(10);
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    // Start the main loop.
    while !&app.should_quit {
        app.set_ps_list();
        // Render the user interface.
        tui.draw(&mut app)?;

        // Handle events.

        if let Ok(event) = my_event::read() {
            if let my_event::Event::Key(key_event) = event { update(&mut app, key_event, &mut tui).await }
        }
    }
    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
// ANCHOR_END: main
