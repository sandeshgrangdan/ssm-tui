use crate::app::{aws::parameter_store::SelectedPsMetadata, App, SelectedTab};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::Text,
    widgets::{
        Block, BorderType, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Wrap,
    },
};

pub const BANNER: &str = r"
                              __        _ 
   ______________ ___        / /___  __(_)
  / ___/ ___/ __ `__ \______/ __/ / / / / 
 (__  |__  ) / / / / /_____/ /_/ /_/ / /  
/____/____/_/ /_/ /_/      \__/\__,_/_/   
";

pub fn render_details(app: &mut App, f: &mut Frame, layout: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
        .margin(2)
        .split(layout);

    let welcome = Block::default()
        .title(Span::styled(
            " </Welcome> ",
            Style::default().fg(Color::LightCyan),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightCyan))
        .border_type(BorderType::Rounded);

    f.render_widget(welcome, layout);

    let changelog = include_str!("../../../README.md").to_string();

    let clean_changelog = if cfg!(debug_assertions) {
        changelog
    } else {
        changelog.replace("\n## [Unreleased]\n", "")
    };

    let top_text_banner = Text::from(BANNER);

    let bottom_text_raw = format!(
        "{}{}",
        "\nPlease report any bugs or missing features to https://github.com/sandeshgrangdan/ssm-tui\n\n",
        clean_changelog
      );
    let bottom_text = Text::from(bottom_text_raw.as_str());

    let top_text = Paragraph::new(top_text_banner)
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default());
    f.render_widget(top_text, chunks[0]);

    let bottom_text = Paragraph::new(bottom_text)
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default())
        .wrap(Wrap { trim: false })
        .scroll((0, 0));
    f.render_widget(bottom_text, chunks[1]);
}
