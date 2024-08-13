use ratatui::{
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap}
};
use crate::app::App;

pub fn render_user_info(_app: &mut App, f: &mut Frame, layout: Rect){
    let text = vec![
        "Use `Esc`, `Ctrl-C` or `q` to stop running.".into(),
        "Use `▲ ▼` to select list.".into(),
        "Use `◄ ►` to scrol value.".into(),
        Line::from(vec![
            Span::styled("Use `/` to filter list.",
            Style::default().fg(Color::Rgb(255, 51, 221)))
        ]),
        Line::from(vec![
            Span::styled("Use `e` or `Enter` to edit parameter store.",
            Style::default().fg(Color::Rgb(255, 51, 221)))
        ]),
    ];

    f.render_widget(
        Paragraph::new(text).wrap(Wrap { trim: true })
        .block(
            Block::default()
                // .title("User Info")
                // .title_alignment(Alignment::Center)
                .borders(Borders::RIGHT)
                .border_type(BorderType::QuadrantInside)
                .padding(Padding::new(1, 1, 1, 1)),
        )
        .style(Style::default().fg(Color::Rgb(239, 184, 135)))
        .alignment(Alignment::Left)
        ,
        layout,
    );
}