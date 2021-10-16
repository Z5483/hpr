use std::ffi::OsStr;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{io, panic, thread};
use std::path::Path;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::Event;
use crossterm::{cursor, execute, terminal};

use lazy_static::lazy_static;

mod app;
mod parser;
mod render;
mod event;
mod draw;

lazy_static! {
    pub static ref REDRAW_REQUEST: (Sender<()>, Receiver<()>) = bounded(1);
}

fn setup_terminal() {
    let mut stdout = io::stdout();

    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    terminal::enable_raw_mode().unwrap();
}

fn cleanup_terminal() {
    let mut stdout = io::stdout();

    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();

    terminal::disable_raw_mode().unwrap();
}

fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        cleanup_terminal();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));
}

fn setup_ui_events() -> Receiver<Event> {
    let (sender, receiver) = unbounded();
    thread::spawn(move || loop {
        sender.send(crossterm::event::read().unwrap()).unwrap();
    });

    receiver
}

fn main() {
    better_panic::install();

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    setup_panic_hook();
    setup_terminal();

    let request_redraw = REDRAW_REQUEST.0.clone();
    let ui_events = setup_ui_events();

    let path = &std::env::args().nth(1).expect("error: no path given");

    if Path::new(path).extension().and_then(OsStr::to_str) != Some("md") {
        eprintln!("error: filetype is not markdown");
        std::process::exit(1);
    }

    let file = File::open(path).expect("error: cannot read file");
    let buf = BufReader::new(file);
    let mut lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("error: cannot parse line"))
        .collect();

    render::render(&mut lines);

    let buffer = parser::Buffer::new(lines.clone());

    let app = Arc::new(Mutex::new(app::App {
        contents: buffer.contents,
        num_of_page: buffer.num_of_page,
        num_of_line: buffer.num_of_line,
        current_page: 1,
        scroll_offset: 0,
        scrolloff_limit: Default::default(),
        can_scroll: false,
    }));

    let move_app = app.clone();

    thread::spawn(move || {
        let app = move_app;

        let redraw_requested = REDRAW_REQUEST.1.clone();

        loop {
            select! {
                recv(redraw_requested) -> _ => {
                    let mut app = app.lock().unwrap();
                    draw::draw(&mut terminal, &mut app);
                }
                default(Duration::from_millis(500)) => {
                    let mut app = app.lock().unwrap();
                    draw::draw(&mut terminal, &mut app);
                }
            }
        }
    });

    loop {
        select! {
            recv(ui_events) -> message => {
                let mut app = app.lock().unwrap();

                match message.unwrap() {
                    Event::Key(key_event) => {
                        event::handle_key_bindings(key_event, &mut app, &request_redraw);
                    }
                    Event::Resize(..) => {
                        let _ = request_redraw.try_send(());
                    }
                    _ => {}
                }
            }
        }
    }
}
