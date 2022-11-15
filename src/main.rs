use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use type_fast::app::App;
use type_fast::start_ui;
fn main() -> Result<(), io::Error> {
    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
