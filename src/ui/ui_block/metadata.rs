use ratatui::{
    layout::Layout, 
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Wrap}
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

    f.render_widget(
        Paragraph::new(format!(
            "\nType: `{}`\n\
            Tier: `{}`\n\
            Data type: `{}`\n\
            Description: `{}`",
            ps_type.to_uppercase(),
            ps_tire.to_uppercase(),
            ps_data_type.to_uppercase(),
            ps_desc
        )).wrap(Wrap {trim: true})
        .block(
            Block::default()
                .title("Metadata")
                // .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow)),
        // .alignment(Alignment::Center),
        layout,
    );
}

