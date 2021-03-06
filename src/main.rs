use std::env;
use std::process;

use dinigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Проблемка: {}", err);
        process::exit(1);
    });

    println!("Ищем {} в файле {}", config.query, config.filename);

    if let Err(e) = dinigrep::run(config) {
        eprintln!("Шо-то пошло на хуй таким способом: {}", e);

        process::exit(1);
    };
}
