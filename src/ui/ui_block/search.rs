use ratatui::{
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap}
};
use crate::app::App;

pub fn render_search(app: &mut App, f: &mut Frame, layout: Rect){
    let first_text_color = Style::default().fg(Color::Rgb((255), (126), (0)));
    
    let text = vec![
        Line::from(vec![
            Span::styled("🐩> ", first_text_color),
            app.ps_filter_data.input.clone().into()
        ]),
    ];

    f.render_widget(
        Paragraph::new(text).wrap(Wrap { trim: true })
        .block(
            Block::default()
                // .title("Search")
                // .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        )
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Left)
        ,
        layout,
    );
}