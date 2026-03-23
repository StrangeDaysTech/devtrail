use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::theme;

/// Convert markdown text to styled ratatui Lines
pub fn markdown_to_lines(markdown: &str, available_width: usize) -> Vec<Line<'static>> {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(markdown, options);

    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut current_spans: Vec<Span<'static>> = Vec::new();
    let mut style_stack: Vec<Style> = vec![Style::default()];
    let mut in_code_block = false;
    let mut list_depth: usize = 0;
    let mut ordered_index: Option<u64> = None;
    let mut in_heading = false;
    let mut heading_level = HeadingLevel::H1;
    let mut table_row: Vec<String> = Vec::new();
    let mut table_alignments: Vec<pulldown_cmark::Alignment> = Vec::new();
    let mut table_header_done = false;
    let mut in_table_cell = false;
    let mut cell_text = String::new();
    let mut table_header_row: Vec<String> = Vec::new();
    let mut table_body_rows: Vec<Vec<String>> = Vec::new();
    let mut code_block_lines: Vec<String> = Vec::new();
    // Current indent level based on last heading seen (2 spaces per level, max 3)
    let mut content_indent: usize = 0;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    in_heading = true;
                    heading_level = level;
                }
                Tag::Paragraph => {}
                Tag::Emphasis => {
                    let current = current_style(&style_stack);
                    style_stack.push(current.add_modifier(Modifier::ITALIC));
                }
                Tag::Strong => {
                    let current = current_style(&style_stack);
                    style_stack.push(current.add_modifier(Modifier::BOLD));
                }
                Tag::Strikethrough => {
                    let current = current_style(&style_stack);
                    style_stack.push(current.add_modifier(Modifier::CROSSED_OUT));
                }
                Tag::CodeBlock(_) => {
                    in_code_block = true;
                    code_block_lines.clear();
                    flush_spans(&mut current_spans, &mut lines, content_indent);
                }
                Tag::List(start) => {
                    list_depth += 1;
                    ordered_index = start;
                }
                Tag::Item => {
                    flush_spans(&mut current_spans, &mut lines, content_indent);
                    let list_indent = "  ".repeat(list_depth.saturating_sub(1));
                    let bullet = if let Some(ref mut idx) = ordered_index {
                        let s = format!("{list_indent}{idx}. ");
                        *idx += 1;
                        s
                    } else {
                        format!("{list_indent}• ")
                    };
                    // Prepend content indent + bullet
                    if content_indent > 0 {
                        current_spans.push(Span::raw(" ".repeat(content_indent)));
                    }
                    current_spans.push(Span::styled(
                        bullet,
                        Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
                    ));
                }
                Tag::Link { dest_url, .. } => {
                    let current = current_style(&style_stack);
                    style_stack.push(
                        current
                            .fg(Color::Blue)
                            .add_modifier(Modifier::UNDERLINED),
                    );
                    let _ = dest_url;
                }
                Tag::Table(alignments) => {
                    table_alignments = alignments;
                    table_header_done = false;
                    flush_spans(&mut current_spans, &mut lines, content_indent);
                }
                Tag::TableHead => {
                    table_row.clear();
                }
                Tag::TableRow => {
                    table_row.clear();
                }
                Tag::TableCell => {
                    in_table_cell = true;
                    cell_text.clear();
                }
                Tag::BlockQuote(_) => {
                    let current = current_style(&style_stack);
                    style_stack.push(current.fg(theme::TEXT_DIM).add_modifier(Modifier::ITALIC));
                }
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) => {
                    in_heading = false;
                    let style = heading_style(heading_level);
                    let text: String = current_spans
                        .drain(..)
                        .map(|s| s.content.to_string())
                        .collect();

                    // Heading indent: H1=0, H2=2, H3+=4 (capped)
                    let heading_indent = heading_indent_level(heading_level);
                    // Update content indent for subsequent content
                    content_indent = heading_indent + 2;
                    // Cap content indent at 6
                    if content_indent > 6 {
                        content_indent = 6;
                    }

                    if heading_level == HeadingLevel::H1 {
                        lines.push(
                            Line::from(Span::styled(text, style))
                                .alignment(ratatui::layout::Alignment::Center),
                        );
                    } else if heading_indent > 0 {
                        lines.push(Line::from(vec![
                            Span::raw(" ".repeat(heading_indent)),
                            Span::styled(text, style),
                        ]));
                    } else {
                        lines.push(Line::from(Span::styled(text, style)));
                    }
                    lines.push(Line::from(""));
                }
                TagEnd::Paragraph => {
                    flush_spans(&mut current_spans, &mut lines, content_indent);
                    lines.push(Line::from(""));
                }
                TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough => {
                    if style_stack.len() > 1 {
                        style_stack.pop();
                    }
                }
                TagEnd::CodeBlock => {
                    in_code_block = false;
                    // Calculate uniform width: max line length + padding
                    let max_len = code_block_lines
                        .iter()
                        .map(|l| l.len())
                        .max()
                        .unwrap_or(0);
                    let code_bg = Style::default()
                        .fg(Color::Rgb(210, 215, 235))
                        .bg(Color::Rgb(45, 45, 60));

                    for code_line in &code_block_lines {
                        let padded = format!("  {:<width$}  ", code_line, width = max_len);
                        let mut spans: Vec<Span<'static>> = Vec::new();
                        if content_indent > 0 {
                            spans.push(Span::raw(" ".repeat(content_indent)));
                        }
                        spans.push(Span::styled(padded, code_bg));
                        lines.push(Line::from(spans));
                    }
                    code_block_lines.clear();
                    lines.push(Line::from(""));
                }
                TagEnd::List(_) => {
                    list_depth = list_depth.saturating_sub(1);
                    if list_depth == 0 {
                        ordered_index = None;
                        lines.push(Line::from(""));
                    }
                }
                TagEnd::Item => {
                    flush_spans(&mut current_spans, &mut lines, content_indent);
                }
                TagEnd::Link => {
                    if style_stack.len() > 1 {
                        style_stack.pop();
                    }
                }
                TagEnd::TableCell => {
                    in_table_cell = false;
                    table_row.push(cell_text.clone());
                    cell_text.clear();
                }
                TagEnd::TableHead => {
                    table_header_row = table_row.clone();
                    table_header_done = true;
                    table_row.clear();
                }
                TagEnd::TableRow => {
                    if table_header_done {
                        table_body_rows.push(table_row.clone());
                    }
                    table_row.clear();
                }
                TagEnd::Table => {
                    // Calculate column widths, fitting to available width
                    let col_widths = compute_column_widths(
                        &table_header_row,
                        &table_body_rows,
                        available_width.saturating_sub(content_indent),
                    );
                    // Render header (multiline cells)
                    render_table_row(&table_header_row, &col_widths, true, &mut lines, content_indent);
                    render_table_separator(&col_widths, &mut lines, content_indent);
                    // Render body rows (multiline cells)
                    for row in &table_body_rows {
                        render_table_row(row, &col_widths, false, &mut lines, content_indent);
                    }
                    table_alignments.clear();
                    table_header_row.clear();
                    table_body_rows.clear();
                    table_header_done = false;
                    lines.push(Line::from(""));
                }
                TagEnd::BlockQuote(_) => {
                    if style_stack.len() > 1 {
                        style_stack.pop();
                    }
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_table_cell {
                    cell_text.push_str(&text);
                } else if in_code_block {
                    for code_line in text.lines() {
                        code_block_lines.push(code_line.to_string());
                    }
                } else if in_heading {
                    current_spans.push(Span::raw(text.to_string()));
                } else {
                    let style = current_style(&style_stack);
                    current_spans.push(Span::styled(text.to_string(), style));
                }
            }
            Event::Code(code) => {
                if in_table_cell {
                    cell_text.push_str(&format!("`{code}`"));
                } else {
                    current_spans.push(Span::styled(
                        format!(" {code} "),
                        Style::default().fg(theme::TEXT).bg(Color::Rgb(60, 60, 60)),
                    ));
                }
            }
            Event::SoftBreak => {
                if in_table_cell {
                    cell_text.push(' ');
                } else {
                    current_spans.push(Span::raw(" "));
                }
            }
            Event::HardBreak => {
                flush_spans(&mut current_spans, &mut lines, content_indent);
            }
            Event::Rule => {
                flush_spans(&mut current_spans, &mut lines, content_indent);
                push_indented(&mut lines, content_indent, Span::styled(
                    "────────────────────────────────────────────────────",
                    Style::default().fg(theme::SUBTLE),
                ));
                lines.push(Line::from(""));
            }
            Event::TaskListMarker(checked) => {
                let marker = if checked {
                    Span::styled(" ✓ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled(" □ ", Style::default().fg(theme::SUBTLE))
                };
                current_spans.push(marker);
            }
            _ => {}
        }
    }

    flush_spans(&mut current_spans, &mut lines, content_indent);
    lines
}

fn current_style(stack: &[Style]) -> Style {
    stack.last().copied().unwrap_or_default()
}

fn flush_spans(spans: &mut Vec<Span<'static>>, lines: &mut Vec<Line<'static>>, indent: usize) {
    if !spans.is_empty() {
        if indent > 0 {
            spans.insert(0, Span::raw(" ".repeat(indent)));
        }
        lines.push(Line::from(spans.drain(..).collect::<Vec<_>>()));
    }
}

fn push_indented(lines: &mut Vec<Line<'static>>, indent: usize, span: Span<'static>) {
    if indent > 0 {
        lines.push(Line::from(vec![Span::raw(" ".repeat(indent)), span]));
    } else {
        lines.push(Line::from(span));
    }
}

/// Indent for heading itself: H1=0, H2=2, H3+=4
fn heading_indent_level(level: HeadingLevel) -> usize {
    match level {
        HeadingLevel::H1 => 0,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 4,
        _ => 4,
    }
}

fn heading_style(_level: HeadingLevel) -> Style {
    Style::default()
        .fg(Color::Rgb(250, 179, 135))
        .add_modifier(Modifier::BOLD)
}

fn compute_column_widths(
    header: &[String],
    body: &[Vec<String>],
    available_width: usize,
) -> Vec<usize> {
    let num_cols = header
        .len()
        .max(body.iter().map(|r| r.len()).max().unwrap_or(0));
    if num_cols == 0 {
        return Vec::new();
    }

    // Calculate natural (max content) width per column
    let mut natural = vec![0usize; num_cols];
    for (i, cell) in header.iter().enumerate() {
        natural[i] = natural[i].max(cell.len());
    }
    for row in body {
        for (i, cell) in row.iter().enumerate() {
            if i < num_cols {
                natural[i] = natural[i].max(cell.len());
            }
        }
    }
    for w in &mut natural {
        *w = (*w).max(3);
    }

    // Overhead: indent is handled outside; here we account for borders
    // "│ " + (" │ " between cols) + " │" = 2 + (num_cols - 1) * 3 + 2
    let border_overhead = 2 + (num_cols.saturating_sub(1)) * 3 + 2;
    let content_budget = available_width.saturating_sub(border_overhead);

    let total_natural: usize = natural.iter().sum();
    if total_natural <= content_budget {
        return natural;
    }

    // Distribute available width proportionally
    let mut widths = vec![0usize; num_cols];
    for (i, &nat) in natural.iter().enumerate() {
        widths[i] = ((nat as f64 / total_natural as f64) * content_budget as f64).floor() as usize;
        widths[i] = widths[i].max(3);
    }

    // Distribute any remaining space to the largest columns
    let assigned: usize = widths.iter().sum();
    let mut remaining = content_budget.saturating_sub(assigned);
    while remaining > 0 {
        // Find column with largest deficit
        let mut best = 0;
        let mut best_deficit = 0usize;
        for (i, (&nat, &w)) in natural.iter().zip(widths.iter()).enumerate() {
            let deficit = nat.saturating_sub(w);
            if deficit > best_deficit {
                best_deficit = deficit;
                best = i;
            }
        }
        if best_deficit == 0 {
            break;
        }
        widths[best] += 1;
        remaining -= 1;
    }

    widths
}

/// Wrap text into lines of at most `width` characters, breaking at word boundaries
fn wrap_cell_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    if text.len() <= width {
        return vec![text.to_string()];
    }

    let mut result = Vec::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        if remaining.len() <= width {
            result.push(remaining.to_string());
            break;
        }
        // Find last space within width
        let chunk = &remaining[..width];
        let break_at = chunk.rfind(' ').unwrap_or(width);
        let break_at = if break_at == 0 { width } else { break_at };
        result.push(remaining[..break_at].to_string());
        remaining = remaining[break_at..].trim_start();
    }

    if result.is_empty() {
        result.push(String::new());
    }
    result
}

fn render_table_row(
    cells: &[String],
    col_widths: &[usize],
    is_header: bool,
    lines: &mut Vec<Line<'static>>,
    indent: usize,
) {
    let style = if is_header {
        Style::default()
            .fg(theme::ACCENT)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let border = Style::default().fg(theme::SUBTLE);

    // Wrap each cell's content and determine how many visual lines this row needs
    let wrapped: Vec<Vec<String>> = col_widths
        .iter()
        .enumerate()
        .map(|(i, &w)| {
            let text = cells.get(i).map(|s| s.as_str()).unwrap_or("");
            wrap_cell_text(text, w)
        })
        .collect();

    let max_lines = wrapped.iter().map(|w| w.len()).max().unwrap_or(1);

    // Render each visual line of the row
    for line_idx in 0..max_lines {
        let mut spans: Vec<Span<'static>> = Vec::new();
        if indent > 0 {
            spans.push(Span::raw(" ".repeat(indent)));
        }
        spans.push(Span::styled("│ ", border));
        for (col, width) in col_widths.iter().enumerate() {
            if col > 0 {
                spans.push(Span::styled(" │ ", border));
            }
            let text = wrapped
                .get(col)
                .and_then(|w| w.get(line_idx))
                .map(|s| s.as_str())
                .unwrap_or("");
            spans.push(Span::styled(
                format!("{:<width$}", text, width = width),
                style,
            ));
        }
        spans.push(Span::styled(" │", border));
        lines.push(Line::from(spans));
    }
}

fn render_table_separator(
    col_widths: &[usize],
    lines: &mut Vec<Line<'static>>,
    indent: usize,
) {
    let sep_style = Style::default().fg(theme::SUBTLE);
    let mut s = String::new();
    if indent > 0 {
        s.push_str(&" ".repeat(indent));
    }
    s.push_str("├─");
    for (i, width) in col_widths.iter().enumerate() {
        if i > 0 {
            s.push_str("─┼─");
        }
        s.push_str(&"─".repeat(*width));
    }
    s.push_str("─┤");
    lines.push(Line::from(Span::styled(s, sep_style)));
}
