use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::tui::app::App;

pub struct StatusBar<'a> {
    app: &'a App,
}

impl<'a> StatusBar<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let key_style = Style::default()
            .fg(Color::Black)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD);
        let desc_style = Style::default().fg(Color::DarkGray);
        let info_style = Style::default().fg(Color::Cyan);

        // Show notification if present
        if let Some(ref msg) = self.app.notification {
            let line = Line::from(vec![
                Span::styled(
                    " ⚠ ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {msg} "),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(
                    "  (press any key to dismiss)",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            buf.set_line(area.x, area.y, &line, area.width);
            return;
        }

        if self.app.is_searching {
            let line = Line::from(vec![
                Span::styled(" / ", key_style),
                Span::styled(
                    format!(" {} ", self.app.search_input),
                    Style::default().fg(Color::White),
                ),
                Span::styled("█", Style::default().fg(Color::White)),
                Span::styled("  Enter", key_style),
                Span::styled(" apply  ", desc_style),
                Span::styled("Esc", key_style),
                Span::styled(" cancel", desc_style),
            ]);
            buf.set_line(area.x, area.y, &line, area.width);
            return;
        }

        let mut spans: Vec<Span> = vec![
            Span::styled(" q ", key_style),
            Span::styled("quit ", desc_style),
            Span::styled(" / ", key_style),
            Span::styled("search ", desc_style),
            Span::styled(" Tab ", key_style),
            Span::styled("panel ", desc_style),
            Span::styled(" Enter ", key_style),
            Span::styled("open ", desc_style),
            Span::styled(" f ", key_style),
            Span::styled("fullscreen ", desc_style),
            Span::styled(" Esc ", key_style),
            Span::styled("back ", desc_style),
            Span::styled(" ? ", key_style),
            Span::styled("help ", desc_style),
        ];

        // Right-aligned doc count
        let count_str = format!(" {} docs ", self.app.index.total_docs);
        let used_width: usize = spans.iter().map(|s| s.content.len()).sum();
        let remaining = area.width as usize - used_width.min(area.width as usize);
        if remaining > count_str.len() {
            let padding = remaining - count_str.len();
            spans.push(Span::styled(
                " ".repeat(padding),
                Style::default(),
            ));
            spans.push(Span::styled(count_str, info_style));
        }

        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
