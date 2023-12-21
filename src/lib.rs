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
  #[arg(short, long)]
  operator: Option<String>,
}

#[derive(Debug)]
pub struct Search {
  search: String,
  intitle: String,
  filetype: String,
  site: String,
  exact: String,
  operator: String,
}

impl Search {
  fn new(args: CliArgs) -> Self {
    let operator = args.operator.unwrap_or("OR".to_string());
    let search = Self::process_option(args.search, "", " ");
    let intitle = Self::process_option(
      args.intitle,
      "intitle:",
      format!(" {} intitle:", operator).as_str(),
    );
    let site = Self::process_option(args.site, "site:", format!(" {} site:", operator).as_str());
    let filetype = Self::process_option(
      args.filetype,
      "filetype:",
      format!(" {} filetype:", operator).as_str(),
    );
    let exact = strings::format::format_exact_search(Self::process_option(args.exact, "", "%"));

    Self {
      search,
      intitle,
      site,
      filetype,
      exact,
      operator,
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

pub fn make_uri(s: Search) -> String {
  format!(
    "{} {} {} {} {}",
    s.exact, s.search, s.intitle, s.site, s.filetype
  )
  .trim()
  .replace(" ", "+")
}

pub fn run(args: CliArgs) -> Result<String, &'static str> {
  let s = Search::new(args);
  let search_uri = make_uri(s);

  if search_uri.is_empty() {
    return Err("No search terms provided. See `findit --help` for more information.");
  }

  let search_url: String = format!("https://www.google.com/search?q={}", search_uri);
  Ok(search_url)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_make_uri() {
    let s = Search {
      exact: "query".into(),
      search: "graphql query".into(),
      intitle: "graphql".into(),
      site: "medium.com".into(),
      filetype: "pdf".into(),
      operator: "OR".into(),
    };

    let expect = "query+graphql+query+graphql+medium.com+pdf";

    assert_eq!(make_uri(s), expect);
  }

  #[test]
  fn test_process_option() {
    let option: Option<Vec<String>> =
      Some(vec!["graphql".into(), "query".into(), "mutation".into()]);
    let s = Search::process_option(option, "intitle:", " OR intitle:");
    let expect = "intitle:graphql OR intitle:query OR intitle:mutation";
    assert_eq!(s, expect);
  }
}
