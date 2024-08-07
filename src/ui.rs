use ratatui::{
    layout::Layout, prelude::*, style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph}
};

mod ui_block;
use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(55),
            Constraint::Percentage(40),
        ])
        .split(f.size());

    let split_left_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(80),
        Constraint::Max(10),
    ])
    .split(layout[0]);

    let split_right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(layout[2]);

    ui_block::ps_list::render_ps_list(app, f, split_left_layout[0]);
    ui_block::modified_user::render_modified_user(app, f, split_left_layout[1]);
    ui_block::details::render_details(app, f, layout[1]);
    ui_block::metadata::render_metadata(app, f, split_right_layout[0]);
    ui_block::user_info::render_user_info(app, f, split_right_layout[1]);
}
