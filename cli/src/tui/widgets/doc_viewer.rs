use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget, Wrap};

use crate::tui::app::{ActivePanel, App};
use crate::tui::markdown::markdown_to_lines;

pub struct DocViewer<'a> {
    app: &'a mut App,
}

impl<'a> DocViewer<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }

    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let is_active = self.app.active_panel == ActivePanel::Document;
        let border_style = if is_active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let title = match &self.app.current_doc {
            Some(doc) => format!(" {} ", doc.filename),
            None => " Document ".to_string(),
        };

        let block = Block::default()
            .title(title)
            .title_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        if self.app.current_doc.is_none() {
            let welcome = render_welcome(self.app.index.total_docs);
            let paragraph = Paragraph::new(welcome);
            paragraph.render(inner, buf);
            return;
        }

        let doc = self.app.current_doc.as_ref().unwrap();

        // Render markdown body only (metadata is in separate panel)
        let mut all_lines = vec![Line::from(""); 1];
        all_lines.extend(markdown_to_lines(&doc.body));

        // Estimate total lines accounting for wrapping
        let width = inner.width.max(1) as usize;
        let wrapped_count: usize = all_lines
            .iter()
            .map(|line| {
                let line_width: usize = line.spans.iter().map(|s| s.content.len()).sum();
                if line_width == 0 {
                    1
                } else {
                    (line_width + width - 1) / width
                }
            })
            .sum();
        self.app.doc_total_lines = wrapped_count;

        let text = Text::from(all_lines);
        let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .scroll((self.app.doc_scroll, 0));
        paragraph.render(inner, buf);

        // Render scrollbar
        if self.app.doc_total_lines > inner.height as usize {
            let mut scrollbar_state = ScrollbarState::new(self.app.doc_total_lines)
                .position(self.app.doc_scroll as usize);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));
            scrollbar.render(inner, buf, &mut scrollbar_state);
        }
    }
}

fn render_welcome(total_docs: usize) -> Vec<Line<'static>> {
    let title = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let dim = Style::default().fg(Color::DarkGray);
    let key = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let text = Style::default().fg(Color::White);

    vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled("  DevTrail Explorer", title)),
        Line::from(Span::styled(
            "  Documentation Governance for AI-Assisted Development",
            dim,
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Total documents: ", dim),
            Span::styled(
                total_docs.to_string(),
                text.add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled("  Quick start", title)),
        Line::from(""),
        Line::from(vec![
            Span::styled("    ↑↓ ", key),
            Span::styled("Navigate groups in the left panel", text),
        ]),
        Line::from(vec![
            Span::styled("  Enter ", key),
            Span::styled("Expand a group and open a document", text),
        ]),
        Line::from(vec![
            Span::styled("   Tab  ", key),
            Span::styled("Cycle panels: Navigation → Metadata → Document", text),
        ]),
        Line::from(vec![
            Span::styled("     /  ", key),
            Span::styled("Search by filename, title, tags, or date", text),
        ]),
        Line::from(vec![
            Span::styled("     f  ", key),
            Span::styled("Toggle document fullscreen", text),
        ]),
        Line::from(vec![
            Span::styled("     ?  ", key),
            Span::styled("Show all keyboard shortcuts", text),
        ]),
        Line::from(vec![
            Span::styled("     q  ", key),
            Span::styled("Quit", text),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  ─────────────────────────────────────────────",
            dim,
        )),
        Line::from(vec![
            Span::styled("  Developed by ", dim),
            Span::styled("Strange Days Tech, S.A.S.", text),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "https://strangedays.tech",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ]),
    ]
}
