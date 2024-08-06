use ratatui::{
    layout::Layout, prelude::*, style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph}
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(50),
            Constraint::Percentage(40),
        ])
        .split(f.size());

    let split_right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(layout[2]);

    render_ps_list(app, f, layout[0]);

    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {:?}
      ",
            app.parameter_store_names.state
        ))
        .block(
            Block::default()
                .title("Details")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        layout[1],
    );

    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Metadata")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        split_right_layout[0],
    );

    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {:?}
      ",
            app.parameter_store_names.last_selected
        ))
        .block(
            Block::default()
                .title("User Info")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        split_right_layout[1],
    );
}

fn render_ps_list(app: &mut App, f: &mut Frame, layout: Rect){
    let list = List::new(app.parameter_store_names.items.clone())
            .block(
                Block::default()
                .title("PS (List)")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            )
            .highlight_style(
                Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
            )
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        

    f.render_stateful_widget(
        list,
        layout,
        &mut app.parameter_store_names.state
    );
}
