use crossterm::event::{KeyCode, KeyEvent, KeyModifiers,KeyEventKind};

use crate::app::App;
use crate::app::ps_list_filter::user_input::InputMode;

pub fn update(app: &mut App, key_event: KeyEvent) {
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
                app.parameter_store_names.next()
            },
            KeyCode::Up => {
                app.clear_scrol();
                app.parameter_store_names.previous()
            },
            KeyCode::Char('/')  => {
                app.toggle_search();
            },
            _ => {}
        },
        InputMode::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Enter => app.ps_filter_data.submit_message(),
            KeyCode::Char(to_insert) => {
                app.ps_filter_data.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                app.ps_filter_data.delete_char();
            }
            KeyCode::Left => {
                app.ps_filter_data.move_cursor_left();
            }
            KeyCode::Right => {
                app.ps_filter_data.move_cursor_right();
            }
            KeyCode::Esc => {
                app.toggle_search();
            }
            _ => {}
        },
        InputMode::Editing => {}
    }
}
