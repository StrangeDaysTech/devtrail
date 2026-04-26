use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::tui::app::{ActivePanel, App, MetaSelection};
use crate::tui::document::{ConfidenceLevel, DocStatus, RiskLevel};
use crate::tui::i18n_strings::t;
use crate::tui::theme;
use crate::utils::{pad_right_visual, truncate_visual};

/// Visual columns reserved for the field label + colon. Sized to fit the
/// widest English label ("Confidence:" = 11 cols) plus a small gutter.
/// Spanish ("Confianza:") and zh-CN labels (≤ 6 cols) all fit comfortably.
const LABEL_WIDTH: usize = 13;

/// Render a metadata field label padded to LABEL_WIDTH visual columns so
/// values stay aligned across all locales.
fn label_block(label: &str, style: ratatui::style::Style) -> ratatui::text::Span<'static> {
    ratatui::text::Span::styled(format!(" {}", pad_right_visual(label, LABEL_WIDTH)), style)
}

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
        let lang = self.app.language.as_str();
        let border_style = if is_active {
            Style::default().fg(theme::BORDER_ACTIVE)
        } else {
            Style::default().fg(theme::SUBTLE)
        };

        let block = Block::default()
            .title(format!(" {} ", t("Metadata", lang)))
            .title_style(if is_active {
                Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::SUBTLE)
            })
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
                    format!(" {}", t("No document selected", lang)),
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
                        Span::styled(
                            format!(" {}  ", t("File:", lang)),
                            Style::default().fg(theme::TEXT_DIM),
                        ),
                        Span::styled(doc.filename.clone(), Style::default().fg(theme::TEXT)),
                    ]),
                    Line::from(Span::styled(
                        format!(" {}", t("No frontmatter", lang)),
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
        let v = Style::default().fg(theme::TEXT);
        let mut lines: Vec<Line<'static>> = vec![Line::from("")];

        // Status
        if let Some(ref status) = fm.status {
            let (indicator, color) = status_style(status);
            lines.push(Line::from(vec![
                label_block(t("Status:", lang), l),
                Span::styled(
                    format!("{indicator} {status}"),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        // Created
        if let Some(ref created) = fm.created {
            lines.push(Line::from(vec![
                label_block(t("Created:", lang), l),
                Span::styled(created.clone(), v),
            ]));
        }

        // Agent
        if let Some(ref agent) = fm.agent {
            lines.push(Line::from(vec![
                label_block(t("Agent:", lang), l),
                Span::styled(agent.clone(), v),
            ]));
        }

        // Confidence
        if let Some(ref confidence) = fm.confidence {
            let (filled, total, color, label) = confidence_bar(confidence);
            lines.push(Line::from(vec![
                label_block(t("Confidence:", lang), l),
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
                label_block(t("Risk:", lang), l),
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
                label_block(t("Review:", lang), l),
                Span::styled(
                    t("⚠ REQUIRED", lang).to_string(),
                    Style::default()
                        .fg(theme::YELLOW)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        // Tags
        if !fm.tags.is_empty() {
            let tag_hint = match self.app.meta_selection {
                Some(MetaSelection::Tag(_)) => t(" (Enter: search)", lang),
                _ => "",
            };
            lines.push(Line::from(vec![
                Span::styled(format!(" {}", t("Tags:", lang)), l),
                Span::styled(tag_hint.to_string(), Style::default().fg(theme::TEXT_DIM)),
            ]));
            let tag_style = Style::default()
                .fg(theme::TEXT_DIM)
                .bg(Color::Rgb(45, 45, 60));
            let tag_selected_style = Style::default()
                .fg(Color::Rgb(220, 224, 242))
                .bg(Color::Rgb(45, 50, 80))
                .add_modifier(Modifier::BOLD);
            for (i, tag) in fm.tags.iter().enumerate() {
                let is_sel = self.app.meta_selection == Some(MetaSelection::Tag(i));
                let marker = if is_sel { " ▸ " } else { "   " };
                let st = if is_sel { tag_selected_style } else { tag_style };
                lines.push(Line::from(vec![
                    Span::styled(marker, l),
                    Span::styled(format!(" {tag} "), st),
                ]));
            }
        }

        // Separator before related
        if !fm.related.is_empty() {
            let sep_width = inner.width.saturating_sub(2) as usize;
            lines.push(Line::from(Span::styled(
                format!(" {}", "─".repeat(sep_width)),
                Style::default().fg(theme::SUBTLE),
            )));

            let hint = match self.app.meta_selection {
                Some(MetaSelection::Related(_)) => t(" (Enter: follow)", lang),
                _ => "",
            };
            lines.push(Line::from(vec![
                Span::styled(format!(" {}", t("Related:", lang)), l),
                Span::styled(hint.to_string(), Style::default().fg(theme::TEXT_DIM)),
            ]));

            let max_link_width = inner.width.saturating_sub(4) as usize;
            for (i, rel) in fm.related.iter().enumerate() {
                let is_selected = self.app.meta_selection == Some(MetaSelection::Related(i));
                let marker = if is_selected { " ▸ " } else { "   " };
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Rgb(220, 224, 242))
                        .bg(Color::Rgb(45, 50, 80))
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                } else {
                    Style::default()
                        .fg(theme::TEXT)
                        .add_modifier(Modifier::UNDERLINED)
                };
                let display = truncate_visual(rel, max_link_width);
                lines.push(Line::from(vec![
                    Span::styled(marker, l),
                    Span::styled(display, style),
                ]));
            }
        }

        // Calculate scroll: find the line with ▸ marker to keep it visible
        let total_lines = lines.len() as u16;
        let scroll = if total_lines > inner.height {
            if let Some(selected_pos) = lines.iter().position(|line| {
                line.spans
                    .first()
                    .map(|s| s.content.contains('▸'))
                    .unwrap_or(false)
            }) {
                let sel = selected_pos as u16;
                if sel >= inner.height {
                    sel.saturating_sub(inner.height.saturating_sub(3))
                        .min(total_lines.saturating_sub(inner.height))
                } else {
                    0
                }
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
        DocStatus::Draft => ("○", theme::YELLOW),
        DocStatus::Accepted => ("■", theme::GREEN),
        DocStatus::Deprecated => ("✗", theme::RED),
        DocStatus::Superseded => ("◌", theme::TEXT_DIM),
        DocStatus::Unknown => ("?", theme::TEXT_DIM),
    }
}

fn confidence_bar(level: &ConfidenceLevel) -> (usize, usize, Color, &'static str) {
    match level {
        ConfidenceLevel::High => (8, 10, theme::GREEN, "high"),
        ConfidenceLevel::Medium => (5, 10, theme::YELLOW, "medium"),
        ConfidenceLevel::Low => (2, 10, theme::RED, "low"),
        ConfidenceLevel::Unknown => (0, 10, theme::TEXT_DIM, "unknown"),
    }
}

fn risk_bar(level: &RiskLevel) -> (usize, usize, Color, &'static str) {
    match level {
        RiskLevel::Low => (2, 10, theme::GREEN, "low"),
        RiskLevel::Medium => (5, 10, theme::YELLOW, "medium"),
        RiskLevel::High => (7, 10, theme::RED, "high"),
        RiskLevel::Critical => (10, 10, theme::RED, "critical"),
        RiskLevel::Unknown => (0, 10, theme::TEXT_DIM, "unknown"),
    }
}

