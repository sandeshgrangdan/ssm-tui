use crate::app::{input::user_input::InputMode, App};

use aws_sdk_ssm::types::{ParameterTier, ParameterType};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

use super::help::popup_area;

pub fn render(app: &mut App, f: &mut Frame) {
    let area = f.area();

    let area = popup_area(area, 70, 30);

    f.render_widget(ratatui::widgets::Clear, area);
    let first_text_color = Style::default().fg(Color::White);

    let name = vec![Line::from(vec![
        Span::styled("> ", first_text_color),
        app.add_ps.1.input.clone().into(),
    ])];

    let desc = vec![Line::from(vec![
        Span::styled("> ", first_text_color),
        app.add_ps_desc.1.input.clone().into(),
    ])];

    let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);

    let [name_area, desc_area] = vertical.areas(area);

    let selected_color = Color::LightRed;
    let normal_color = Color::Gray;

    let get_name_color = if app.add_ps.0 {
        selected_color
    } else {
        normal_color
    };

    let get_desc_color = if app.add_ps_desc.0 {
        selected_color
    } else {
        normal_color
    };

    let get_ps_type = match app.ps_type {
        ParameterType::SecureString => "SecureString",
        ParameterType::String => "String",
        ParameterType::StringList => "StringList",
        _ => "",
    };

    let get_ps_tire = match app.ps_tier {
        ParameterTier::Advanced => "Advanced",
        ParameterTier::IntelligentTiering => "IntelligentTiering",
        ParameterTier::Standard => "Standard",
        _ => "",
    };

    f.render_widget(
        Paragraph::new(name)
            // .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(" + Name ")
                    .title_style(Style::default().bold().fg(get_name_color))
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(get_name_color))
                    .border_style(Style::default().fg(get_name_color))
                    .padding(Padding::new(1, 0, 0, 0)),
            )
            .alignment(Alignment::Left),
        name_area,
    );

    f.render_widget(
        Paragraph::new(desc)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(
                        Line::from(" + Description ")
                            .left_aligned()
                            .fg(get_desc_color),
                    )
                    .title(
                        Line::from(format!(" Type: **{} ", get_ps_type))
                            .left_aligned()
                            .fg(Color::LightMagenta)
                            .bold(),
                    )
                    .title(
                        Line::from(format!(" Tire: **{} ", get_ps_tire))
                            .left_aligned()
                            .fg(Color::LightBlue)
                            .bold(),
                    )
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(get_desc_color))
                    .padding(Padding::new(1, 1, 0, 1)),
            )
            .alignment(Alignment::Left),
        desc_area,
    );

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing if app.add_ps.0 => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            f.set_cursor_position(Position::new(
                name_area.x + app.add_ps.1.character_index as u16 + 4,
                name_area.y + 1,
            ));
        }

        InputMode::Editing if app.add_ps_desc.0 => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            f.set_cursor_position(Position::new(
                desc_area.x + app.add_ps_desc.1.character_index as u16 + 4,
                desc_area.y + 1,
            ));
        }

        InputMode::Editing => {}
    }

    // f.render_widget(block, area);
}
