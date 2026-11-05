use std::env::args;
use wotoi::cli::Args;
use wotoi::app;

fn main() {
    match Args::from_args(args()) {
        Ok(args) => app::run_app(args),
        Err(parse_error) => app::print_parse_error(parse_error),
    }
}
