use crate::app::{aws::parameter_store::SelectedPsMetadata, App, SelectedTab};

use color_eyre::owo_colors::OwoColorize;
use layout::Flex;
use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::Text,
    widgets::{
        Block, BorderType, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Wrap,
    },
};

const LEFT_HELP_TEXT: &[HelpBlock] = &[
    HelpBlock {
        title: "General",
        bindings: &[
            KeyBindings {
                keys: &["0~6"],
                desc: "switch tab",
            },
            KeyBindings {
                keys: &["Enter"],
                desc: "select",
            },
            KeyBindings {
                keys: &["c"],
                desc: "change context",
            },
            KeyBindings {
                keys: &["n"],
                desc: "select namespace",
            },
            KeyBindings {
                keys: &["N"],
                desc: "select namespaces",
            },
            KeyBindings {
                keys: &["Tab"],
                desc: "change focus",
            },
            KeyBindings {
                keys: &["y"],
                desc: "open yaml dialog",
            },
            KeyBindings {
                keys: &["q", "Esc"],
                desc: "quit",
            },
            KeyBindings {
                keys: &["q", "Esc"],
                desc: "close dialog",
            },
            KeyBindings {
                keys: &["h", "?"],
                desc: "Show this help",
            },
        ],
    },
    HelpBlock {
        title: "View Control",
        bindings: &[
            KeyBindings {
                keys: &["j", "k", "Down", "Up"],
                desc: "goto next/previous line",
            },
            KeyBindings {
                keys: &["PgDn", "PgUp"],
                desc: "scroll upward/downward",
            },
            KeyBindings {
                keys: &["Left", "Right"],
                desc: "scroll horizontal",
            },
            KeyBindings {
                keys: &["g"],
                desc: "goto first line",
            },
            KeyBindings {
                keys: &["G"],
                desc: "goto last line",
            },
        ],
    },
    HelpBlock {
        title: "Remap Keys",
        bindings: &[
            KeyBindings {
                keys: &["Ctrl-p"],
                desc: "↑",
            },
            KeyBindings {
                keys: &["Ctrl-n"],
                desc: "↓",
            },
            KeyBindings {
                keys: &["Ctrl-f"],
                desc: "→",
            },
            KeyBindings {
                keys: &["Ctrl-b"],
                desc: "←",
            },
            KeyBindings {
                keys: &["Ctrl-u"],
                desc: "PgUp",
            },
            KeyBindings {
                keys: &["Ctrl-d"],
                desc: "PgDn",
            },
            KeyBindings {
                keys: &["Ctrl-h", "BS"],
                desc: "Del",
            },
            KeyBindings {
                keys: &["Ctrl-a"],
                desc: "Home",
            },
            KeyBindings {
                keys: &["Ctrl-e"],
                desc: "End",
            },
            KeyBindings {
                keys: &["Ctrl-["],
                desc: "Esc",
            },
        ],
    },
];

const RIGHT_HELP_TEXT: &[HelpBlock] = &[
    HelpBlock {
        title: "Input Form",
        bindings: &[
            KeyBindings {
                keys: &["Ctrl-a", "Home"],
                desc: "move the cursor to the first",
            },
            KeyBindings {
                keys: &["Ctrl-e", "End"],
                desc: "move the cursor to the end",
            },
            KeyBindings {
                keys: &["Ctrl-f", "Right"],
                desc: "move the cursor to the right",
            },
            KeyBindings {
                keys: &["Ctrl-b", "Left"],
                desc: "move the cursor to the left",
            },
            KeyBindings {
                keys: &["Ctrl-w"],
                desc: "delete the text from the cursor position to the first",
            },
            KeyBindings {
                keys: &["Ctrl-k"],
                desc: "delete the text from the cursor position to the end",
            },
        ],
    },
    HelpBlock {
        title: "API / Yaml Tab",
        bindings: &[KeyBindings {
            keys: &["f"],
            desc: "open select dialog",
        }],
    },
    HelpBlock {
        title: "Search (Only text view)",
        bindings: &[
            KeyBindings {
                keys: &["/"],
                desc: "enable search mode",
            },
            KeyBindings {
                keys: &["q", "Esc"],
                desc: "disable search mode",
            },
            KeyBindings {
                keys: &["Enter"],
                desc: "confirm search word",
            },
            KeyBindings {
                keys: &["n", "N"],
                desc: "goto next/prev word",
            },
        ],
    },
    HelpBlock {
        title: "Filter (Only table view)",
        bindings: &[
            KeyBindings {
                keys: &["/"],
                desc: "open filter form",
            },
            KeyBindings {
                keys: &["q", "Esc"],
                desc: "clear filter form",
            },
            KeyBindings {
                keys: &["Enter"],
                desc: "confirm filter word",
            },
        ],
    },
    HelpBlock {
        title: "Log",
        bindings: &[
            KeyBindings {
                keys: &["Enter"],
                desc: "insert blank line",
            },
            KeyBindings {
                keys: &["f", "p"],
                desc: "toggle json pretty print",
            },
        ],
    },
];

#[derive(Clone)]
struct HelpBlock {
    title: &'static str,
    bindings: &'static [KeyBindings],
}

struct KeyBindings {
    keys: &'static [&'static str],
    desc: &'static str,
}

impl KeyBindings {
    fn keys(&self) -> String {
        self.keys.join(" ")
    }

    fn desc(&self) -> String {
        self.desc.to_string()
    }
}

pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(layout::Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn render(_app: &mut App, f: &mut Frame) {
    let area = f.area();

    let area = popup_area(area, 70, 70);

    f.render_widget(ratatui::widgets::Clear, area); //this clears out the background

    let title_style = Style::default().bold().fg(Color::White);
    let key_style = Style::default().fg(Color::LightCyan);
    let desc_style = Style::default().fg(Color::Gray);

    let text = vec![
        Line::from(vec![Span::styled("[ General ]", title_style)]),
        Line::from(vec![
            Span::styled("        Ctrl-c: ", key_style),
            Span::styled("quit", desc_style),
        ]),
        Line::from(vec![
            Span::styled("   j k Down Up: ", key_style),
            Span::styled("scroll upward/downward", desc_style),
        ]),
        Line::from(vec![
            Span::styled("h l Left Right: ", key_style),
            Span::styled("select Left/Right panel", desc_style),
        ]),
        Line::from(vec![
            Span::styled("             /: ", key_style),
            Span::styled("enable search mode", desc_style),
        ]),
        Line::from(vec![
            Span::styled("             e: ", key_style),
            Span::styled("edit selected parameter store", desc_style),
        ]),
        "\n\n".into(),
        Line::from(vec![Span::styled(
            "[ Search (only text view) ]",
            title_style,
        )]),
        Line::from(vec![
            Span::styled("        Ctrl-c: ", key_style),
            Span::styled("quit", desc_style),
        ]),
        Line::from(vec![
            Span::styled("   j k Down Up: ", key_style),
            Span::styled("scroll upward/downward", desc_style),
        ]),
        Line::from(vec![
            Span::styled("h l Left Right: ", key_style),
            Span::styled("select Left/Right panel", desc_style),
        ]),
        Line::from(vec![
            Span::styled("             /: ", key_style),
            Span::styled("enable search mode", desc_style),
        ]),
        Line::from(vec![
            Span::styled("             e: ", key_style),
            Span::styled("edit selected parameter store", desc_style),
        ]),
    ];

    f.render_widget(
        Paragraph::new(text)
            // .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(" + Help ")
                    .title_style(Style::default().bold().fg(Color::LightGreen))
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::LightGreen))
                    .padding(Padding::new(1, 1, 0, 1)),
            )
            .alignment(Alignment::Left),
        area,
    );

    // f.render_widget(block, area);
}
