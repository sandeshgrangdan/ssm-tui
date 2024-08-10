use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::Layout, 
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap}
};
use aws_sdk_ssm::types::{
    ParameterType::{
        SecureString,
        String,
        StringList
    },
    ParameterTier::{
        Advanced,
        IntelligentTiering,
        Standard,
    }
};
use chrono::{DateTime, Utc};

use crate::app::App;


pub fn render_metadata(app: &mut App, f: &mut Frame, layout: Rect){
    let ps_metadata = app.get_selected_metadata();

    let ps_type = match &ps_metadata.r#type {
        Some(my_ps_type) => match my_ps_type {
                SecureString => "SecureString",
                String => "String",
                StringList => "StringList",
                _ => ""
            }
        
        None => ""
    };

    let ps_tire = match &ps_metadata.tier {
        Some(my_ps_tire) => match my_ps_tire {
                Advanced => "Advanced",
                IntelligentTiering => "IntelligentTiering",
                Standard => "Standard",
                _ => ""
            }
        
        None => ""
    };

    let ps_data_type = match &ps_metadata.data_type {
        Some(my_ps_data_type) => my_ps_data_type,
        None => ""
    };

    let ps_desc = match &ps_metadata.description {
        Some(my_ps_description) => my_ps_description,
        None => "No description for this parameter store."
    };

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

    let first_text_color = Style::default().fg(Color::Rgb((255), (126), (0)));

    let text = vec![
        Line::from(vec![
            Span::styled("Type:        ",first_text_color),
            ps_type.gray().bold()
        ]),
        Line::from(vec![
            Span::styled("Tier:        ",first_text_color),
            ps_tire.gray().bold()
        ]),
        Line::from(vec![
            Span::styled("Data type:   ", first_text_color),
            ps_data_type.gray().bold()
        ]),
        Line::from(vec![
            Span::styled("Description: ", first_text_color),
            ps_desc.gray().bold()
        ]),
        Line::from(vec![
            Span::styled("User :       ", first_text_color),
            modified_user.gray().bold()
        ]),
        Line::from(vec![
            Span::styled("Date :       ", first_text_color),
            formatted_date.gray().bold()
        ]),
    ];

    f.render_widget(
        Paragraph::new(text).wrap(Wrap {trim: true})
        .block(
            Block::default()
                // .title("Metadata")
                // .title_alignment(Alignment::Center)
                .borders(Borders::NONE)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 1)),
        )
        .style(Style::default().fg(Color::White)),
        // .alignment(Alignment::Center),
        layout,
    );
}

