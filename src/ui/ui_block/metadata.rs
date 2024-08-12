use ratatui::{
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap}
};
use aws_sdk_ssm::types::{
    ParameterType::{
        SecureString,
        String as PsString,
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
use crate::app::PsMetadata;


pub fn render_metadata(app: &mut App, f: &mut Frame, layout: Rect){

    let mut ps_type = "None";
    let mut ps_tire = "None";
    let mut ps_data_type = "None";
    let mut ps_desc = "None";
    let mut modified_user = "None";
    let mut formatted_date = String::from("None");
    let mut ps_name: &str = "No data found";
    
    match app.get_selected_ps_data() {
        PsMetadata::Data(ps_metadata, _,name ) => {

            ps_name = name;

            ps_type = match &ps_metadata.r#type {
                Some(my_ps_type) => match my_ps_type {
                        SecureString => "SecureString",
                        PsString => "String",
                        StringList => "StringList",
                        _ => ""
                    }
                None => ""
            };

            ps_tire = match &ps_metadata.tier {
                Some(my_ps_tire) => match my_ps_tire {
                        Advanced => "Advanced",
                        IntelligentTiering => "IntelligentTiering",
                        Standard => "Standard",
                        _ => ""
                    }
                None => ""
            };

            ps_data_type = match &ps_metadata.data_type {
                Some(my_ps_data_type) => my_ps_data_type,
                None => ""
            };

            ps_desc = match &ps_metadata.description {
                Some(my_ps_description) => my_ps_description,
                None => "No description for this parameter store."
            };

            modified_user = match &ps_metadata.last_modified_user{
                Some(name) => name,
                None => ""
            };

            let modified_date = match &ps_metadata.last_modified_date {
                Some(date) => date,
                None => panic!("Error for Datetime")
            };

            let datetime: DateTime<Utc> = DateTime::from_timestamp(modified_date.secs(), modified_date.subsec_nanos()).unwrap();

            formatted_date = datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

        }
        PsMetadata::None => {}
    }

    let first_text_color = Style::default().fg(Color::Rgb((255), (126), (0)));

    let text = vec![
        Line::from(vec![
            Span::styled("Name:        ",first_text_color),
            ps_name.gray().bold()
        ]),
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
            ps_data_type.to_uppercase().gray().bold()
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
        ])
    ];

    f.render_widget(
        Paragraph::new(text).wrap(Wrap {trim: true})
        .block(
            Block::default()
                // .title("Metadata")
                // .title_alignment(Alignment::Center)
                .borders(Borders::LEFT)
                .border_type(BorderType::QuadrantInside)
                .padding(Padding::new(1, 1, 1, 1)),
        )
        .style(Style::default().fg(Color::White)),
        // .alignment(Alignment::Center),
        layout,
    );
}

