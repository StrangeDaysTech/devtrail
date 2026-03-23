use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

use crate::tui::app::{ActivePanel, App, NavSelection};
use crate::tui::index::DocEntry;

pub struct NavTree<'a> {
    app: &'a App,
}

impl<'a> NavTree<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for NavTree<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let is_active = self.app.active_panel == ActivePanel::Navigation;
        let border_style = if is_active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(" Navigation ")
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines: Vec<Line<'static>> = Vec::new();
        let search = self.app.search_query.as_deref();

        let has_search = search.is_some();

        for (gi, group) in self.app.index.groups.iter().enumerate() {
            let is_expanded = self.app.expanded_groups[gi];
            let is_selected = self.app.selection == NavSelection::Group(gi);

            // When searching, auto-expand groups that have matches
            let show_children = if has_search {
                group_has_matches(group, search)
            } else {
                is_expanded
            };

            // When searching, skip groups with no matches
            if has_search && !show_children {
                continue;
            }

            let arrow = if show_children { "▾" } else { "▸" };
            let doc_count = count_group_docs(group);

            let count_str = if doc_count > 0 {
                format!(" ({doc_count})")
            } else {
                String::new()
            };

            let style = if is_selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            lines.push(Line::from(vec![
                Span::styled(format!(" {arrow} "), Style::default().fg(Color::Cyan)),
                Span::styled(group.label.clone(), style),
                Span::styled(count_str, Style::default().fg(Color::DarkGray)),
            ]));

            if show_children {
                // Direct files in group
                for (fi, entry) in group.files.iter().enumerate() {
                    if !matches_search(entry, search) {
                        continue;
                    }
                    let is_sel = self.app.selection == NavSelection::GroupFile(gi, fi);
                    let style = file_style(is_sel);
                    lines.push(Line::from(vec![
                        Span::raw("     "),
                        Span::styled(truncate_filename(&entry.filename, inner.width as usize - 6), style),
                    ]));
                }

                // Subgroups
                for (si, sg) in group.subgroups.iter().enumerate() {
                    // When searching, skip subgroups with no matches
                    let sg_has_matches = sg.files.iter().any(|e| matches_search(e, search));
                    if has_search && !sg_has_matches {
                        continue;
                    }

                    let is_sel = self.app.selection == NavSelection::Subgroup(gi, si);
                    let sg_count = sg.files.len();
                    let sg_style = if is_sel {
                        Style::default()
                            .bg(Color::DarkGray)
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Yellow)
                    };

                    lines.push(Line::from(vec![
                        Span::raw("   "),
                        Span::styled("▸ ", Style::default().fg(Color::Yellow)),
                        Span::styled(format!("{}/", sg.label), sg_style),
                        Span::styled(
                            format!(" ({sg_count})"),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]));

                    // Files in subgroup
                    for (fi, entry) in sg.files.iter().enumerate() {
                        if !matches_search(entry, search) {
                            continue;
                        }
                        let is_sel =
                            self.app.selection == NavSelection::SubgroupFile(gi, si, fi);
                        let style = file_style(is_sel);
                        lines.push(Line::from(vec![
                            Span::raw("       "),
                            Span::styled(
                                truncate_filename(&entry.filename, inner.width as usize - 8),
                                style,
                            ),
                        ]));
                    }
                }
            }
        }

        let paragraph = Paragraph::new(lines);
        paragraph.render(inner, buf);
    }
}

fn file_style(selected: bool) -> Style {
    if selected {
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    }
}

fn count_group_docs(group: &crate::tui::index::DocGroup) -> usize {
    let direct = group.files.len();
    let sub: usize = group.subgroups.iter().map(|sg| sg.files.len()).sum();
    direct + sub
}

fn truncate_filename(name: &str, max_width: usize) -> String {
    if name.len() <= max_width {
        name.to_string()
    } else if max_width > 3 {
        format!("{}...", &name[..max_width - 3])
    } else {
        name[..max_width].to_string()
    }
}

fn group_has_matches(group: &crate::tui::index::DocGroup, search: Option<&str>) -> bool {
    if group.files.iter().any(|e| matches_search(e, search)) {
        return true;
    }
    group
        .subgroups
        .iter()
        .any(|sg| sg.files.iter().any(|e| matches_search(e, search)))
}

fn matches_search(entry: &DocEntry, search: Option<&str>) -> bool {
    let Some(q) = search else {
        return true;
    };
    let query = q.to_lowercase();

    // Search in filename
    if entry.filename.to_lowercase().contains(&query) {
        return true;
    }
    // Search in title
    if entry.title.to_lowercase().contains(&query) {
        return true;
    }
    // Search in tags
    if entry.tags.iter().any(|t| t.to_lowercase().contains(&query)) {
        return true;
    }
    // Search in created date
    if !entry.created.is_empty() && entry.created.contains(&query) {
        return true;
    }
    // Search in id
    if entry.id.to_lowercase().contains(&query) {
        return true;
    }

    false
}
