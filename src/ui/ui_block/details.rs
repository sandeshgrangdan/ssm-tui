use crate::app::{aws::parameter_store::SelectedPsMetadata, App, SelectedTab};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::Text,
    widgets::{
        Block, BorderType, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Wrap,
    },
};

pub fn render_details(app: &mut App, f: &mut Frame, layout: Rect) {
    let mut lines = vec![];
    let mut ps_name: &String = &String::from("No data found");

    let border_style = match app.selected_tab {
        SelectedTab::Details => Style::default().fg(Color::LightRed),
        _ => Style::default().fg(Color::Gray),
    };

    let scroll = app.vertical_scroll;
    let mut version: i64 = 1;

    match app.get_selected_ps_data() {
        SelectedPsMetadata::Data(metadata, value, name) => {
            lines = value;
            ps_name = name;
            version = metadata.version;
        }
        SelectedPsMetadata::None => {}
    }

    let text = Text::from(lines);

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title("Configuration")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        // .wrap(Wrap { trim: false })
        .scroll((scroll as u16, 0));

    f.render_widget(
        paragraph
            .block(
                Block::default()
                    .title(format!(" v{} - [{}] ", version, ps_name.clone()))
                    .title_alignment(Alignment::Left)
                    .title_style(Style::default().fg(Color::Yellow))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(border_style)
                    .padding(Padding::new(1, 1, 0, 1)),
            )
            .style(Style::default().fg(Color::LightCyan)),
        // .alignment(Alignment::Center),
        layout,
    );

    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        layout,
        &mut app.vertical_scroll_state,
    );
}
