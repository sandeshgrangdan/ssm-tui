use ratatui::{
    layout::Layout, 
    prelude::*, 
};

mod ui_block;
use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(8),
            Constraint::Percentage(100),
        ])
        .split(f.size());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(70),
            Constraint::Min(30),
        ])
        .split(main_layout[0]);
    
    let main_app_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            if app.filter_ps_list {
                 vec![
                    Constraint::Min(20),
                    Constraint::Length(3),
                ]
            }else {
                vec![
                    Constraint::Min(20)
                ]
            }
        )
        .split(main_layout[1]);


    let layout: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(main_app_layout[0]);


    ui_block::metadata::render_metadata(app, f, top_layout[0]);
    ui_block::user_info::render_user_info(app, f, top_layout[1]);
    ui_block::ps_list::render_ps_list(app, f, layout[0]);
    // ui_block::modified_user::render_modified_user(app, f, split_left_layout[1]);
    ui_block::details::render_details(app, f, layout[1]);
    if app.filter_ps_list {
        ui_block::search::render_search(app, f, main_app_layout[1]);
    }
    
}
