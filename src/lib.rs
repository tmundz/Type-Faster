use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::{io, thread};

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::read;
use crate::app::ui;
use crate::app::App;
pub mod app;

pub fn start_ui(_app: Rc<RefCell<App>>) -> Result<(), io::Error> {
    let stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;
    terminal.draw(|rect| ui::render(rect))?;

    thread::sleep(Duration::from_millis(5000));
    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}

pub fn get_phrase() {
    let phrase_vec: Vec<String> = read::get_file();
    println!("{:#?}", phrase_vec);
}
