use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use type_fast::app::App;
use type_fast::{get_phrase, start_ui};
fn main() -> Result<(), io::Error> {
    let app = Rc::new(RefCell::new(App::new()));
    //start_ui(app)?;
    get_phrase();
    Ok(())
}
