use crate::app::App;
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

pub fn render_user_info(app: &mut App, f: &mut Frame, layout: Rect) {
    let text = vec![
        "Use `Ctrl-C` to stop running.".into(),
        "Use `▲ ▼` to scroll.".into(),
        "Use `◄ ►` to move.".into(),
        Line::from(vec![Span::styled(
            "Use `/` to filter list.",
            Style::default().fg(Color::LightRed),
        )]),
        Line::from(vec![Span::styled(
            "Use `e` or `Enter` to edit parameter store.",
            Style::default().fg(Color::Magenta),
        )]),
        Line::from(vec![
            Span::styled("Profile: ", Style::default().fg(Color::LightYellow)),
            app.args.profile.clone().light_green().bold(),
            Span::styled(" Region: ", Style::default().fg(Color::LightYellow)),
            app.args.region.clone().light_green().bold(),
        ]),
    ];

    f.render_widget(
        Paragraph::new(text)
            // .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    // .title("User Info")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::RIGHT)
                    .border_type(BorderType::QuadrantInside)
                    .padding(Padding::new(1, 1, 1, 1)),
            )
            .style(Style::default().fg(Color::LightBlue))
            .alignment(Alignment::Left),
        layout,
    );
}
