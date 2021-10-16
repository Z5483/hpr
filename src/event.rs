use crossbeam_channel::Sender;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app;
use crate::cleanup_terminal;

pub fn handle_key_bindings(
    key_event: KeyEvent,
    mut app: &mut app::App,
    request_redraw: &Sender<()>,
) {
    match (key_event.modifiers, key_event.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
            cleanup_terminal();
            std::process::exit(0);
        }
        (KeyModifiers::NONE, KeyCode::Char('q')) => {
            cleanup_terminal();
            std::process::exit(0);
        }
        (KeyModifiers::NONE, KeyCode::Char('k')) => {
            if app.can_scroll {
                if app.scroll_offset > 0 {
                    app.scroll_offset -= 1;
                }
            }
        }
        (KeyModifiers::NONE, KeyCode::Char('j')) => {
            if app.can_scroll {
                if app.scroll_offset < app.scrolloff_limit {
                    app.scroll_offset += 1;
                }
            }
        }
        (KeyModifiers::NONE, KeyCode::Char('n')) => {
            if app.current_page < app.num_of_page {
                app.current_page += 1;
            }
            app.scroll_offset = 0;
        }
        (KeyModifiers::NONE, KeyCode::Char('p')) => {
            if app.current_page > 1 {
                app.current_page -= 1;
            }
            app.scroll_offset = 0;
        }
        _ => {}
    }

    let _ = request_redraw.try_send(());
}
