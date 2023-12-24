use clap::Parser;
use findit::config::args;
use findit::helper::middlewares;
use findit::uri;
use std::process;

fn main() {
  middlewares::human_readable_panic();

  let args = args::CliArgs::parse();

  match findit::run(args) {
    Ok(search_uri) => {
      println!("{}", search_uri);
      uri::open_uri(search_uri);
      process::exit(0)
    }
    Err(error) => eprintln!("{}", error),
  }
}
