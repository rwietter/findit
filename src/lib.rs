use clap::Parser;
use helper::strings;

mod helper;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(author = "Maurício W. <mauriciobw17@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Advanced web search", long_about = None)]
#[command(ignore_errors = true)]
pub struct CliArgs {
  #[arg(short = 'k', long)]
  #[arg(num_args(0..))]
  search: Option<Vec<String>>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  intitle: Option<Vec<String>>,
  #[arg(num_args(0..))]
  #[arg(short, long)]
  site: Option<Vec<String>>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  filetype: Option<Vec<String>>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  exact: Option<Vec<String>>,
}

#[derive(Debug)]
struct Search {
  search: String,
  intitle: String,
  filetype: String,
  site: String,
  exact: String,
}

impl Search {
  fn new(args: CliArgs) -> Self {
    let search = Self::process_option(args.search, "", " ");
    let intitle = Self::process_option(args.intitle, "intitle:", " OR intitle:");
    let site = Self::process_option(args.site, "site:", " OR site:");
    let filetype = Self::process_option(args.filetype, "filetype:", " OR filetype:");
    let exact = strings::format::format_exact_search(Self::process_option(args.exact, "", "%"));

    Self {
      search,
      intitle,
      site,
      filetype,
      exact,
    }
  }

  fn process_option(option: Option<Vec<String>>, prefix: &str, separator: &str) -> String {
    option
      .map(|s| {
        if !s.is_empty() {
          return format!("{}{}", prefix, s.join(separator));
        }
        String::new()
      })
      .unwrap_or_else(String::new)
  }
}

pub fn run(args: CliArgs) -> Result<String, &'static str> {
  let s = Search::new(args);
  let search_string = format!(
    "{} {} {} {} {}",
    s.exact, s.search, s.intitle, s.site, s.filetype
  )
  .trim()
  .replace(" ", "+");

  if search_string.is_empty() {
    return Err("No search terms provided. See `findit --help` for more information.");
  }

  let search_url: String = format!("https://www.google.com/search?q={}", search_string);
  Ok(search_url)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intitle() {
    let result = run(CliArgs::parse_from(&[
      "findit", // first parameter is name of the program
      "-k",
      "Lambda lifting",
      "-i",
      "lambda",
      "lifting",
      "church",
    ]));

    let expect = "https://www.google.com/search?q=Lambda+lifting+intitle:lambda+OR+intitle:lifting+OR+intitle:church";

    match result {
      Ok(search_uri) => assert_eq!(search_uri, expect),
      Err(error) => panic!("Error: {}", error),
    }
  }

  #[test]
  fn exact_search() {
    let result = run(CliArgs::parse_from(&[
      "findit", "-k", "rust", "python", "-e", "while", "for loop",
    ]));

    let expect = "https://www.google.com/search?q=\"while\"+\"for+loop\"+rust+python";

    match result {
      Ok(search_uri) => assert_eq!(search_uri, expect),
      Err(error) => panic!("{}", error),
    }
  }
}
