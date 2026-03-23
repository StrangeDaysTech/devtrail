use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

use super::app::{ActivePanel, App, ViewMode};

/// Process events and update app state. Returns true if a redraw is needed.
pub fn handle_events(app: &mut App) -> std::io::Result<bool> {
    if !event::poll(Duration::from_millis(50))? {
        return Ok(false);
    }

    let event = event::read()?;

    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            handle_key(app, key);
            Ok(true)
        }
        Event::Resize(_, _) => Ok(true),
        _ => Ok(false),
    }
}

fn handle_key(app: &mut App, key: KeyEvent) {
    // Search mode: capture typed characters
    if app.is_searching {
        handle_search_key(app, key);
        return;
    }

    // Notification: any key dismisses it
    if app.notification.is_some() {
        app.notification = None;
        return;
    }

    // Help mode: any key closes it
    if app.view_mode == ViewMode::Help {
        app.view_mode = ViewMode::Normal;
        return;
    }

    // Ctrl+C / Ctrl+Q: always quit regardless of mode
    if key.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(key.code, KeyCode::Char('c') | KeyCode::Char('q'))
    {
        app.should_quit = true;
        return;
    }

    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('?') => app.toggle_help(),
        KeyCode::Tab => app.toggle_panel(),
        KeyCode::Char('f') => app.toggle_fullscreen(),
        KeyCode::Char('/') => app.start_search(),
        KeyCode::Char('s') => app.cycle_sort(),

        // Group jumping with number keys
        KeyCode::Char(c @ '1'..='8') => {
            app.jump_to_group(c.to_digit(10).unwrap() as usize);
        }

        // Navigation or document scrolling depending on active panel
        KeyCode::Char('j') | KeyCode::Down => match app.active_panel {
            ActivePanel::Document => app.scroll_down(1),
            ActivePanel::Navigation => app.nav_down(),
            ActivePanel::Metadata => {}
        },
        KeyCode::Char('k') | KeyCode::Up => match app.active_panel {
            ActivePanel::Document => app.scroll_up(1),
            ActivePanel::Navigation => app.nav_up(),
            ActivePanel::Metadata => {}
        },

        // Enter: open/expand or follow selected related link
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
            if app.active_panel == ActivePanel::Navigation {
                app.nav_enter();
            } else if app.active_panel == ActivePanel::Metadata
                && app.selected_related.is_some()
            {
                app.follow_selected_related();
            }
        }

        // Back: collapse/go back / clear search
        KeyCode::Esc | KeyCode::Char('h') | KeyCode::Left => {
            match app.active_panel {
                ActivePanel::Document if key.code == KeyCode::Esc => {
                    if app.view_mode == ViewMode::Fullscreen {
                        app.view_mode = ViewMode::Normal;
                    } else {
                        app.active_panel = ActivePanel::Navigation;
                    }
                }
                ActivePanel::Metadata if key.code == KeyCode::Esc => {
                    app.selected_related = None;
                    app.active_panel = ActivePanel::Navigation;
                }
                ActivePanel::Navigation => {
                    if key.code == KeyCode::Esc && app.search_query.is_some() {
                        app.cancel_search();
                    } else {
                        app.nav_back();
                    }
                }
                _ => {}
            }
        }

        // Document scroll shortcuts
        KeyCode::Char('g') => app.scroll_to_top(),
        KeyCode::Char('G') => app.scroll_to_bottom(),
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.scroll_down(15);
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.scroll_up(15);
        }
        KeyCode::PageDown => app.scroll_down(20),
        KeyCode::PageUp => app.scroll_up(20),

        // Next/Previous document
        KeyCode::Char('n') => app.next_document(),
        KeyCode::Char('N') => app.prev_document(),

        // Refresh
        KeyCode::Char('r') => {
            let root = app.project_root.clone();
            *app = App::new(&root);
        }

        _ => {}
    }
}

fn handle_search_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.cancel_search(),
        KeyCode::Enter => app.apply_search(),
        KeyCode::Backspace => {
            app.search_input.pop();
        }
        KeyCode::Char(c) => {
            app.search_input.push(c);
        }
        _ => {}
    }
}
