use layout::Flex;
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::app::{aws::parameter_store::SelectedPsMetadata, input::user_input::InputMode, App};

fn center(area: Rect, percent_x: usize) -> Rect {
    let [area] = Layout::horizontal([Constraint::Percentage(percent_x as u16)])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([Constraint::Length(3)])
        .flex(Flex::Center)
        .areas(area);
    area
}

pub fn render(app: &mut App, f: &mut Frame) {
    let mut ps_name = String::new();
    let area = f.area();

    let area = center(area, 60);

    f.render_widget(ratatui::widgets::Clear, area);
    let first_text_color = Style::default().fg(Color::White);

    let name = vec![Line::from(vec![
        Span::styled("> ", first_text_color),
        app.delete_ps.1.input.clone().into(),
    ])];

    let selected_color = Color::LightGreen;
    let normal_color = Color::LightRed;

    match app.get_selected_ps_data() {
        SelectedPsMetadata::Data(_metadata, _value, name) => {
            ps_name = name.to_string();
        }
        SelectedPsMetadata::None => {}
    }

    let get_name_color = if app.delete_ps.1.input == ps_name {
        selected_color
    } else {
        normal_color
    };

    f.render_widget(
        Paragraph::new(name)
            // .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(format!(
                        " - To confirm deletion, type `{}` in the text input field.",
                        ps_name
                    ))
                    .title_style(Style::default().bold().fg(get_name_color))
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(get_name_color))
                    .padding(Padding::new(1, 0, 0, 0)),
            )
            .alignment(Alignment::Left),
        area,
    );

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing if app.delete_ps.0 => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            f.set_cursor_position(Position::new(
                area.x + app.delete_ps.1.character_index as u16 + 4,
                area.y + 1,
            ));
        }

        InputMode::Editing => {}
    }
    // f.render_widget(block, area);
}
