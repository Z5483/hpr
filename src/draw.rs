use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;
use tui::Terminal;

use ansi4tui::bytes_to_text;

use crate::app::App;

#[allow(dead_code)]
pub enum PaddingDirection {
    Top,
    Bottom,
    Left,
    Right,
    All,
}

pub fn add_padding(mut rect: Rect, n: u16, direction: PaddingDirection) -> Rect {
    match direction {
        PaddingDirection::Top => {
            rect.y += n;
            rect.height = rect.height.saturating_sub(n);
            rect
        }
        PaddingDirection::Bottom => {
            rect.height = rect.height.saturating_sub(n);
            rect
        }
        PaddingDirection::Left => {
            rect.x += n;
            rect.width = rect.width.saturating_sub(n);
            rect
        }
        PaddingDirection::Right => {
            rect.width = rect.width.saturating_sub(n);
            rect
        }
        PaddingDirection::All => {
            rect.y += n;
            rect.height = rect.height.saturating_sub(n * 2);

            rect.x += n;
            rect.width = rect.width.saturating_sub(n * 2);

            rect
        }
    }
}

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    terminal
        .draw(|frame| {
            let mut layout = Layout::default()
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(frame.size().height - 3),
                        Constraint::Min(0),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            layout[1] = add_padding(layout[1], 10, PaddingDirection::Left);
            layout[1] = add_padding(layout[1], 10, PaddingDirection::Right);
            layout[1] = add_padding(layout[1], 2, PaddingDirection::Top);

            let text = bytes_to_text(app.contents[app.current_page - 1].clone());
            let content = Paragraph::new(text);

            let line_count: u16 = app.num_of_line[app.current_page - 1] as u16;

            if layout[1].height < line_count {
                app.scrolloff_limit = line_count - layout[1].height;
                app.can_scroll = true;
            } else {
                app.can_scroll = false;
            }

            frame.render_widget(content.scroll((app.scroll_offset, 0)), layout[1]);

            let bottom_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Min(0),
                        Constraint::Length(app.num_of_page.to_string().len() as u16 * 2 + 3),
                    ]
                    .as_ref(),
                )
                .split(layout[2]);

            frame.render_widget(
                Paragraph::new(Spans::from(Span::styled(
                    format!("{}/{}", app.current_page, app.num_of_page),
                    Style::default(),
                ))),
                bottom_layout[1],
            );
        })
        .unwrap();
}
