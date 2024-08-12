use crossterm::event::{KeyCode, KeyEvent, KeyModifiers,KeyEventKind};

use crate::app::App;
use crate::tui::Tui;
use crate::app::ps_list_filter::user_input::InputMode;

pub async fn update(app: &mut App, key_event: KeyEvent, tui : &mut Tui) {
    match app.ps_filter_data.input_mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit()
                }
            }
            KeyCode::Right | KeyCode::Char('j') => app.increment_scrol(),
            KeyCode::Left | KeyCode::Char('k') => app.decrement_scrol(),
            KeyCode::Down => {
                app.clear_scrol();
                app.parameter_stores.next()
            },
            KeyCode::Up => {
                app.clear_scrol();
                app.parameter_stores.previous()
            },
            KeyCode::Char('/')  => {
                app.toggle_search();
            },
            KeyCode::Char('e') | KeyCode::Enter => {
                    let _ = tui.init_vim();

                    if let Err(e) = app.launch_vim().await {
                        eprintln!("Error launching Vim: {}", e);
                    }

                    let _ = tui.exit_vim();
            },
            _ => {}
        },
        InputMode::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Enter => app.toggle_search(),
            KeyCode::Char(to_insert) => {
                app.parameter_stores.state.select(Some(0));
                app.ps_filter_data.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                app.parameter_stores.state.select(Some(0));
                app.ps_filter_data.delete_char();
            }
            KeyCode::Left => {
                app.parameter_stores.state.select(Some(0));
                app.ps_filter_data.move_cursor_left();
            }
            KeyCode::Right => {
                app.parameter_stores.state.select(Some(0));
                app.ps_filter_data.move_cursor_right();
            }
            KeyCode::Down => {
                app.clear_scrol();
                app.parameter_stores.next()
            },
            KeyCode::Up => {
                app.clear_scrol();
                app.parameter_stores.previous()
            },
            KeyCode::Esc => {
                app.toggle_search();
            }
            _ => {}
        },
        InputMode::Editing => {}
    }
}
