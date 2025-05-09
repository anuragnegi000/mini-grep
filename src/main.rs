use std::env;
use std::process;

use cli_search::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e)=cli_search::run(config){
        println!("Application error: {}",e);
        process::exit(1);
    }

}
