use aws_sdk_ssm::types::{ParameterTier, ParameterType};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::input::input::{InputMode, UserInput};
use crate::app::{App, SelectedTab};
use crate::tui::Tui;

async fn open_editor(app: &mut App, tui: &mut Tui) {
    let _ = tui.init_vim();

    if let Err(e) = app.launch_vim().await {
        eprintln!("Error launching Vim: {}", e);
    }

    let _ = tui.exit_vim();
}

pub async fn update(app: &mut App, key_event: KeyEvent, tui: &mut Tui) {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Char('d') | KeyCode::Char('D')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                app.delete_ps.0 = !app.delete_ps.0;
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                app.quit();
            }
            KeyCode::Esc | KeyCode::Char('q') if app.show_help => app.show_help = !app.show_help,
            KeyCode::Char('r') | KeyCode::Char('R') => {
                app.fetch_ps_data().await;
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                app.add_ps.0 = !app.add_ps.0;
                app.add_ps_desc.0 = false;
                if app.add_ps.1.input.is_empty() {
                    app.add_ps.1.input = String::from("/");
                }
                app.add_ps.1.character_index = app.add_ps.1.input.len();
                app.input_mode = InputMode::Editing
            }
            KeyCode::Down | KeyCode::Char('j') => match app.selected_tab {
                SelectedTab::Details => {
                    app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                    app.vertical_scroll_state =
                        app.vertical_scroll_state.position(app.vertical_scroll);
                    app.increment_scrol();
                }
                SelectedTab::List => {
                    app.clear_scrol();
                    app.vertical_scroll = 0;
                    app.vertical_scroll_state = app.vertical_scroll_state.position(0);
                    app.parameter_stores.next()
                }
            },
            KeyCode::Up | KeyCode::Char('k') => match app.selected_tab {
                SelectedTab::Details => {
                    app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                    app.vertical_scroll_state =
                        app.vertical_scroll_state.position(app.vertical_scroll);
                    app.decrement_scrol();
                }
                SelectedTab::List => {
                    app.clear_scrol();
                    app.vertical_scroll = 0;
                    app.vertical_scroll_state = app.vertical_scroll_state.position(0);
                    app.parameter_stores.previous()
                }
            },
            KeyCode::Right | KeyCode::Char('l') => {
                app.selected_tab = SelectedTab::Details;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.selected_tab = SelectedTab::List;
            }
            KeyCode::Char('/') => {
                app.toggle_search();
            }
            KeyCode::Char('?') => {
                app.show_help = !app.show_help;
            }
            KeyCode::Char('e') | KeyCode::Enter => open_editor(app, tui).await,
            _ => {}
        },
        InputMode::Editing if key_event.kind == KeyEventKind::Press && app.search.0 => {
            match key_event.code {
                KeyCode::Enter => app.toggle_search(),
                KeyCode::Char(to_insert) => {
                    app.parameter_stores.state.select(Some(0));
                    app.search.1.enter_char(to_insert);
                }
                KeyCode::Backspace => {
                    app.parameter_stores.state.select(Some(0));
                    app.search.1.delete_char();
                }
                KeyCode::Left => {
                    app.parameter_stores.state.select(Some(0));
                    app.search.1.move_cursor_left();
                }
                KeyCode::Right => {
                    app.parameter_stores.state.select(Some(0));
                    app.search.1.move_cursor_right();
                }
                KeyCode::Down => {
                    app.clear_scrol();
                    app.parameter_stores.next()
                }
                KeyCode::Up => {
                    app.clear_scrol();
                    app.parameter_stores.previous()
                }
                KeyCode::Esc => {
                    app.toggle_search();
                }
                _ => {}
            }
        }
        InputMode::Editing
            if key_event.kind == KeyEventKind::Press && (app.add_ps.0 || app.add_ps_desc.0) =>
        {
            match key_event.code {
                KeyCode::Char('s') | KeyCode::Char('S')
                    if key_event.modifiers == KeyModifiers::CONTROL =>
                {
                    app.ps_type = match &app.ps_type {
                        ParameterType::String => ParameterType::SecureString,
                        ParameterType::SecureString => ParameterType::StringList,
                        ParameterType::StringList => ParameterType::String,
                        _ => ParameterType::String,
                    };
                }
                KeyCode::Char('t') | KeyCode::Char('T')
                    if key_event.modifiers == KeyModifiers::CONTROL =>
                {
                    app.ps_tier = match &app.ps_tier {
                        ParameterTier::Standard => ParameterTier::Advanced,
                        ParameterTier::Advanced => ParameterTier::IntelligentTiering,
                        ParameterTier::IntelligentTiering => ParameterTier::Standard,
                        _ => ParameterTier::Standard,
                    };
                }
                KeyCode::Enter => {
                    app.add().await;
                    open_editor(app, tui).await;
                }
                KeyCode::Char(to_insert) if app.add_ps.0 => {
                    app.add_ps.1.enter_char(to_insert);
                }
                KeyCode::Char(to_insert) if app.add_ps_desc.0 => {
                    app.add_ps_desc.1.enter_char(to_insert);
                }
                KeyCode::Backspace if app.add_ps.0 => {
                    if app.add_ps.1.input != "/" {
                        app.add_ps.1.delete_char();
                    }
                }
                KeyCode::Backspace if app.add_ps_desc.0 => {
                    app.add_ps_desc.1.delete_char();
                }
                KeyCode::Left if app.add_ps.0 => {
                    app.add_ps.1.move_cursor_left();
                }
                KeyCode::Left if app.add_ps_desc.0 => {
                    app.add_ps_desc.1.move_cursor_left();
                }
                KeyCode::Right if app.add_ps.0 => {
                    app.add_ps.1.move_cursor_right();
                }
                KeyCode::Right if app.add_ps_desc.0 => {
                    app.add_ps_desc.1.move_cursor_right();
                }
                KeyCode::Down => {
                    app.add_ps.0 = false;
                    app.add_ps_desc.0 = true;
                }
                KeyCode::Up => {
                    app.add_ps.0 = true;
                    app.add_ps_desc.0 = false;
                }
                KeyCode::Esc => {
                    app.add_ps.0 = false;
                    app.add_ps_desc.0 = false;
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            }
        }
        InputMode::Editing if key_event.kind == KeyEventKind::Press && app.delete_ps.0 => {
            match key_event.code {
                KeyCode::Enter => {
                    let selected_ps_index =
                        &app.parameter_stores.state.selected().unwrap_or_default();
                    let ps_name = &app.parameter_stores.display_items[*selected_ps_index].clone();

                    if *ps_name == app.delete_ps.1.input {
                        app.delete_ps().await;
                        app.delete_ps.0 = false;
                        app.delete_ps.1 = UserInput::default();
                        app.input_mode = InputMode::Normal;
                        if *selected_ps_index > 0 {
                            app.parameter_stores
                                .state
                                .select(Some(selected_ps_index - 1));
                        }
                    }
                }
                KeyCode::Char(to_insert) => {
                    app.delete_ps.1.enter_char(to_insert);
                }
                KeyCode::Backspace => {
                    app.delete_ps.1.delete_char();
                }
                KeyCode::Left => {
                    app.delete_ps.1.move_cursor_left();
                }
                KeyCode::Right => {
                    app.delete_ps.1.move_cursor_right();
                }
                KeyCode::Esc => {
                    app.delete_ps.0 = false;
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            }
        }
        InputMode::Editing => {}
    }
}
