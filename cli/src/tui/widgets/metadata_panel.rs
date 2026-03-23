use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::tui::app::{ActivePanel, App};
use crate::tui::document::{ConfidenceLevel, DocStatus, RiskLevel};
use crate::tui::theme;

pub struct MetadataPanel<'a> {
    app: &'a App,
}

impl<'a> MetadataPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for MetadataPanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let is_active = self.app.active_panel == ActivePanel::Metadata;
        let border_style = if is_active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(theme::SUBTLE)
        };

        let block = Block::default()
            .title(" Metadata ")
            .title_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(theme::BORDER_TYPE)
            .border_style(border_style)
            .style(Style::default().bg(theme::SURFACE));

        let inner = block.inner(area);
        block.render(area, buf);

        let doc = match &self.app.current_doc {
            Some(d) => d,
            None => {
                let line = Line::from(Span::styled(
                    " No document selected",
                    Style::default().fg(theme::TEXT_DIM),
                ));
                Paragraph::new(vec![line]).render(inner, buf);
                return;
            }
        };

        let fm = match &doc.frontmatter {
            Some(fm) => fm,
            None => {
                let lines = vec![
                    Line::from(vec![
                        Span::styled(" File:  ", Style::default().fg(theme::TEXT_DIM)),
                        Span::styled(doc.filename.clone(), Style::default().fg(Color::White)),
                    ]),
                    Line::from(Span::styled(
                        " No frontmatter",
                        Style::default().fg(theme::TEXT_DIM),
                    )),
                ];
                Paragraph::new(lines)
                    .wrap(Wrap { trim: false })
                    .render(inner, buf);
                return;
            }
        };

        let l = Style::default().fg(theme::TEXT_DIM);
        let v = Style::default().fg(Color::White);
        let mut lines: Vec<Line<'static>> = Vec::new();

        // Status
        if let Some(ref status) = fm.status {
            let (indicator, color) = status_style(status);
            lines.push(Line::from(vec![
                Span::styled(" Status:      ", l),
                Span::styled(
                    format!("{indicator} {status}"),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        // Created
        if let Some(ref created) = fm.created {
            lines.push(Line::from(vec![
                Span::styled(" Created:     ", l),
                Span::styled(created.clone(), v),
            ]));
        }

        // Agent
        if let Some(ref agent) = fm.agent {
            lines.push(Line::from(vec![
                Span::styled(" Agent:       ", l),
                Span::styled(agent.clone(), v),
            ]));
        }

        // Confidence
        if let Some(ref confidence) = fm.confidence {
            let (filled, total, color, label) = confidence_bar(confidence);
            lines.push(Line::from(vec![
                Span::styled(" Confidence:  ", l),
                Span::styled(
                    format!("{}{}", "█".repeat(filled), "░".repeat(total - filled)),
                    Style::default().fg(color),
                ),
                Span::styled(format!("  {label}"), Style::default().fg(color)),
            ]));
        }

        // Risk
        if let Some(ref risk) = fm.risk_level {
            let (filled, total, color, label) = risk_bar(risk);
            lines.push(Line::from(vec![
                Span::styled(" Risk:        ", l),
                Span::styled(
                    format!("{}{}", "█".repeat(filled), "░".repeat(total - filled)),
                    Style::default().fg(color),
                ),
                Span::styled(format!("  {label}"), Style::default().fg(color)),
            ]));
        }

        // Review required
        if let Some(true) = fm.review_required {
            lines.push(Line::from(vec![
                Span::styled(" Review:      ", l),
                Span::styled(
                    "⚠ REQUIRED",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        // Tags
        if !fm.tags.is_empty() {
            let mut spans: Vec<Span<'static>> = vec![Span::styled(" Tags:        ", l)];
            let colors = [
                Color::Cyan,
                Color::Magenta,
                Color::Green,
                Color::Yellow,
                Color::Blue,
            ];
            for (i, tag) in fm.tags.iter().enumerate() {
                let color = colors[i % colors.len()];
                spans.push(Span::styled(format!("[{tag}]"), Style::default().fg(color)));
            }
            lines.push(Line::from(spans));
        }

        // Separator before related
        if !fm.related.is_empty() {
            lines.push(Line::from(Span::styled(
                " ─────────────────────────────",
                Style::default().fg(theme::TEXT_DIM),
            )));

            let hint = if self.app.selected_related.is_some() {
                "Enter: follow  Tab: next"
            } else {
                "Tab: select"
            };
            lines.push(Line::from(vec![
                Span::styled(" Related      ", l),
                Span::styled(hint, Style::default().fg(theme::TEXT_DIM)),
            ]));

            let max_link_width = inner.width.saturating_sub(4) as usize; // 3 marker + 1 padding
            for (i, rel) in fm.related.iter().enumerate() {
                let is_selected = self.app.selected_related == Some(i);
                let marker = if is_selected { " ▸ " } else { "   " };
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::UNDERLINED)
                };
                let display = truncate_str(rel, max_link_width);
                lines.push(Line::from(vec![
                    Span::styled(marker, l),
                    Span::styled(display, style),
                ]));
            }
        }

        // Calculate scroll to ensure selected related is visible
        let total_lines = lines.len() as u16;
        let scroll = if total_lines > inner.height {
            if let Some(sel_idx) = self.app.selected_related {
                // The selected related link's line position in the list
                let related_start = total_lines.saturating_sub(fm.related.len() as u16);
                let selected_line = related_start + sel_idx as u16;
                // Scroll so the selected line is visible
                selected_line.saturating_sub(inner.height.saturating_sub(1))
            } else {
                0
            }
        } else {
            0
        };

        Paragraph::new(lines)
            .scroll((scroll, 0))
            .render(inner, buf);
    }
}

fn status_style(status: &DocStatus) -> (&'static str, Color) {
    match status {
        DocStatus::Draft => ("○", Color::Yellow),
        DocStatus::Accepted => ("■", Color::Green),
        DocStatus::Deprecated => ("✗", Color::Red),
        DocStatus::Superseded => ("◌", theme::TEXT_DIM),
        DocStatus::Unknown => ("?", theme::TEXT_DIM),
    }
}

fn confidence_bar(level: &ConfidenceLevel) -> (usize, usize, Color, &'static str) {
    match level {
        ConfidenceLevel::High => (8, 10, Color::Green, "high"),
        ConfidenceLevel::Medium => (5, 10, Color::Yellow, "medium"),
        ConfidenceLevel::Low => (2, 10, Color::Red, "low"),
        ConfidenceLevel::Unknown => (0, 10, theme::TEXT_DIM, "unknown"),
    }
}

fn risk_bar(level: &RiskLevel) -> (usize, usize, Color, &'static str) {
    match level {
        RiskLevel::Low => (2, 10, Color::Green, "low"),
        RiskLevel::Medium => (5, 10, Color::Yellow, "medium"),
        RiskLevel::High => (7, 10, Color::Red, "high"),
        RiskLevel::Critical => (10, 10, Color::Red, "critical"),
        RiskLevel::Unknown => (0, 10, theme::TEXT_DIM, "unknown"),
    }
}

fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else if max > 3 {
        format!("{}...", &s[..max - 3])
    } else {
        s[..max].to_string()
    }
}
