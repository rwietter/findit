use clap::Parser;
use findit::CliArgs;
use std::process;

mod helper;
use helper::middlewares;

fn main() {
  middlewares::human_readable_panic();

  let args = CliArgs::parse();

  match findit::run(args) {
    Ok(search_uri) => {
      println!("{}", search_uri);
      findit::open_uri(search_uri);
      process::exit(0)
    }
    Err(error) => eprintln!("{}", error),
  }
}
