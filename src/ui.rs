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
            Constraint::Percentage(30),
            Constraint::Percentage(50),
            Constraint::Percentage(40),
        ])
        .split(f.size());

    let split_right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(layout[2]);

    ui_block::ps_list::render_ps_list(app, f, layout[0]);
    ui_block::details::render_details(app, f, layout[1]);
    ui_block::metadata::render_metadata(app, f, split_right_layout[0]);
    ui_block::user_info::render_user_info(app, f, split_right_layout[1]);
}
