use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use crate::tui::theme;

pub struct HelpPopup;

impl Widget for HelpPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = centered_rect(65, 80, area);

        Clear.render(popup, buf);

        let block = Block::default()
            .title(" Keyboard Shortcuts ")
            .title_style(
                Style::default()
                    .fg(theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(theme::BORDER_TYPE)
            .border_style(Style::default().fg(theme::ACCENT))
            .style(Style::default().bg(theme::SURFACE));

        let key_style = Style::default()
            .fg(theme::ACCENT)
            .add_modifier(Modifier::BOLD);
        let desc_style = Style::default().fg(theme::TEXT);
        let section_style = Style::default()
            .fg(Color::Rgb(250, 179, 135))
            .add_modifier(Modifier::BOLD);
        let dim = Style::default().fg(theme::TEXT_DIM);

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled("  Navigation panel", section_style)),
            help_line("  j / ↓       ", "Move selection down", key_style, desc_style),
            help_line("  k / ↑       ", "Move selection up", key_style, desc_style),
            help_line("  Enter       ", "Expand group / Open document", key_style, desc_style),
            help_line("  Esc         ", "Collapse group / Clear search", key_style, desc_style),
            help_line("  1-8         ", "Jump to group by number", key_style, desc_style),
            Line::from(""),
            Line::from(Span::styled("  Metadata panel", section_style)),
            help_line("  j / ↓       ", "Move between related links", key_style, desc_style),
            help_line("  k / ↑       ", "Move between related links", key_style, desc_style),
            help_line("  Enter       ", "Follow selected related link", key_style, desc_style),
            help_line("  Esc         ", "Back to Navigation", key_style, desc_style),
            Line::from(""),
            Line::from(Span::styled("  Document panel", section_style)),
            help_line("  j / ↓       ", "Scroll down", key_style, desc_style),
            help_line("  k / ↑       ", "Scroll up", key_style, desc_style),
            help_line("  g / G       ", "Top / Bottom of document", key_style, desc_style),
            help_line("  Ctrl+d / u  ", "Half page down / up", key_style, desc_style),
            help_line("  PgDn / PgUp ", "Page down / up", key_style, desc_style),
            help_line("  n / N       ", "Next / Previous document", key_style, desc_style),
            help_line("  f           ", "Toggle fullscreen", key_style, desc_style),
            help_line("  Esc         ", "Exit fullscreen / Back to Nav", key_style, desc_style),
            Line::from(""),
            Line::from(Span::styled("  General", section_style)),
            help_line("  Tab         ", "Next panel: Nav → Meta → Doc", key_style, desc_style),
            help_line("  Shift+Tab   ", "Prev panel: Doc → Meta → Nav", key_style, desc_style),
            help_line("  /           ", "Search by name, title, tags, date", key_style, desc_style),
            help_line("  s           ", "Cycle sort order", key_style, desc_style),
            help_line("  r           ", "Refresh document index", key_style, desc_style),
            help_line("  q           ", "Quit", key_style, desc_style),
            help_line("  Ctrl+C      ", "Force quit", key_style, desc_style),
            Line::from(""),
            Line::from(Span::styled(
                "  Press any key to close",
                dim,
            )),
        ];

        let paragraph = Paragraph::new(lines).block(block);
        paragraph.render(popup, buf);
    }
}

fn help_line<'a>(key: &'a str, desc: &'a str, key_style: Style, desc_style: Style) -> Line<'a> {
    Line::from(vec![
        Span::styled(key, key_style),
        Span::styled(desc, desc_style),
    ])
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1])[1]
}
