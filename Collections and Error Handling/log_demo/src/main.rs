// use log::{debug, error, info, warn};


// fn main() {
//     env_logger::init();

//     info!("This is an info message");
//     warn!("This is a warning");
//     error!("This is an error");
//     debug!("This is a debug message");
// }

// ----------------------------------------------
use slog::{info, o, Drain, Logger};
use slog_async;
use slog_term;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());

    info!(log, "Application started"; "version" => "1.0.0");
    info!(log, "This is an info message"; "user" => "JohnDoe");
}