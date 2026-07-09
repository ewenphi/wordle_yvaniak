use std::process;
use wordle_yvaniak::config::Config;

fn main() {
    let config: Config = Config::cmd().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = wordle_yvaniak::launch(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

//TODO: Test main
