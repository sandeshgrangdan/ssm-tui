use ratatui::{
    layout::Layout, 
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Wrap}
};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::app::App;

pub fn render_modified_user(app: &mut App, f: &mut Frame, layout: Rect){
    let ps_metadata = &app.get_selected_metadata();

    let modified_user = match &ps_metadata.last_modified_user{
        Some(name) => name,
        None => ""
    };


    let modified_date = match &ps_metadata.last_modified_date {
        Some(date) => date,
        None => panic!("Error for Datetime")
    };


    // Create a DateTime<Utc> from the timestamp and subsecond nanoseconds
    let datetime: DateTime<Utc> = DateTime::from_timestamp(modified_date.secs(), modified_date.subsec_nanos()).unwrap();

    // Format the DateTime to the desired format
    let formatted_date = datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string();



    f.render_widget(
        Paragraph::new(format!(
            "User: {}.\n
            Date: {}.\n
            ",modified_user,
            formatted_date
        )).wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("Last modified")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        layout,
    );
}