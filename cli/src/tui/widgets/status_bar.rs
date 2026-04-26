use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::tui::app::App;
use crate::tui::i18n_strings::t;
use crate::tui::theme;
use crate::utils::visual_width;

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
        // Fill status bar background
        for x in area.x..area.x + area.width {
            buf[(x, area.y)].set_bg(theme::SURFACE);
        }

        let key_style = Style::default()
            .fg(theme::ACCENT)
            .add_modifier(Modifier::BOLD);
        let desc_style = Style::default().fg(theme::TEXT_DIM);
        let info_style = Style::default().fg(theme::ACCENT);

        let lang = self.app.language.as_str();

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
                    format!("  {}", t("(press any key to dismiss)", lang)),
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
                    Style::default().fg(theme::TEXT),
                ),
                Span::styled("█", Style::default().fg(theme::TEXT)),
                Span::styled("  Enter", key_style),
                Span::styled(format!(" {}  ", t("apply", lang)), desc_style),
                Span::styled("Esc", key_style),
                Span::styled(format!(" {}", t("cancel", lang)), desc_style),
            ]);
            buf.set_line(area.x, area.y, &line, area.width);
            return;
        }

        let mut spans: Vec<Span> = vec![
            Span::styled(" q ", key_style),
            Span::styled(format!("{} ", t("quit", lang)), desc_style),
            Span::styled(" / ", key_style),
            Span::styled(format!("{} ", t("search", lang)), desc_style),
            Span::styled(" Tab ", key_style),
            Span::styled(format!("{} ", t("panel", lang)), desc_style),
            Span::styled(" Enter ", key_style),
            Span::styled(format!("{} ", t("open", lang)), desc_style),
            Span::styled(" f ", key_style),
            Span::styled(format!("{} ", t("fullscreen", lang)), desc_style),
            Span::styled(" Esc ", key_style),
            Span::styled(format!("{} ", t("back", lang)), desc_style),
            Span::styled(" ? ", key_style),
            Span::styled(format!("{} ", t("help", lang)), desc_style),
        ];

        // Right-aligned: path + doc count
        let path_display = self.app.project_root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?");
        let right_str = format!(
            " {}  │  {} {} ",
            path_display,
            self.app.index.total_docs,
            t("docs", lang)
        );
        let used_width: usize = spans.iter().map(|s| visual_width(s.content.as_ref())).sum();
        let remaining = (area.width as usize).saturating_sub(used_width);
        let right_cols = visual_width(&right_str);
        if remaining > right_cols {
            let padding = remaining - right_cols;
            spans.push(Span::styled(" ".repeat(padding), Style::default()));
            spans.push(Span::styled(
                format!(" {} ", path_display),
                desc_style,
            ));
            spans.push(Span::styled(" │ ", Style::default().fg(Color::DarkGray)));
            spans.push(Span::styled(
                format!("{} {} ", self.app.index.total_docs, t("docs", lang)),
                info_style,
            ));
        }

        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
