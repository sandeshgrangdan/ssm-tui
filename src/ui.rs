use ratatui::{layout::Layout, prelude::*};

mod ui_block;
pub mod widgets;
use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let area = f.area();
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(9), Constraint::Percentage(100)])
        .split(area);

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(60), Constraint::Min(40)])
        .split(main_layout[0]);

    let main_app_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if app.search.0 {
            vec![Constraint::Min(20), Constraint::Length(3)]
        } else {
            vec![Constraint::Min(20)]
        })
        .split(main_layout[1]);

    let layout: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_app_layout[0]);

    ui_block::metadata::render_metadata(app, f, top_layout[0]);
    ui_block::user_info::render_user_info(app, f, top_layout[1]);
    ui_block::ps_list::render_ps_list(app, f, layout[0]);
    // ui_block::modified_user::render_modified_user(app, f, split_left_layout[1]);
    match app.parameter_stores.state.selected() {
        Some(_) => ui_block::details::render_details(app, f, layout[1]),
        None => ui_block::welcome::render_details(app, f, layout[1]),
    }
    // ui_block::welcome::render_details(app, f, layout[1]);
    // ui_block::details::render_details(app, f, layout[1]);
    if app.search.0 {
        ui_block::search::render_search(app, f, main_app_layout[1]);
    }

    if app.add_ps.0 || app.add_ps_desc.0 {
        ui_block::add_ps::render(app, f);
    }

    if app.delete_ps.0 {
        ui_block::delete_ps::render(app, f);
    }

    if app.show_help {
        ui_block::help::render(app, f);
    }
}
