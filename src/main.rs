use std::{env, process};

use task_app::app_module::app::App;

fn main() {
    if let Err(e) = App::run(env::args()) {
        eprintln!("{e}");
        process::exit(1)
    };
}
