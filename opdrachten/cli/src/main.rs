use std::env;
use std::process;

use cli::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumetns: {}", err);
        process::exit(1);
    });

    eprintln!("Searing for {}", config.query);
    eprintln!("In file {}", config.filename);

   if let Err(e) = cli::run(config) {
    eprintln!("Application eror: {}", e);
    process::exit(1);
   }
}



