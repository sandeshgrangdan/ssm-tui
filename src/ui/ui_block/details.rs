use ratatui::{
    text::Text,
    prelude::*, 
    style::{Color, Style}, 
    widgets::{
        Block, 
        BorderType, 
        Borders, 
        Paragraph, 
        Wrap,
        Padding
    }
};
use crate::app::App;
use crate::app::PsMetadata;

pub fn render_details(app: &mut App, f: &mut Frame, layout: Rect){
    let mut ps_value : String = String::from("No data found.");
    let mut ps_name : &String = &String::from("No data found");
    
    match app.get_selected_ps_data(){
        PsMetadata::Data(_, value,name ) => {
            ps_value = value;
            ps_name = name;
        }
        PsMetadata::None => {}
    }

    let text = Text::from(format!("{}",ps_value));

    let paragraph = Paragraph::new(text)
            .block(Block::default().title("Configuration").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .scroll((app.scroll,0));

    f.render_widget(
        paragraph
        .block(
            Block::default()
                .title(format!("Value({})",ps_name))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .padding(Padding::new(1, 1, 0, 1)),
        )
        .style(Style::default().fg(Color::Rgb(83, 178, 226))),
        // .alignment(Alignment::Center),
        layout,
    );
}
