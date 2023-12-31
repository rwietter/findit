use helper::strings::format;
use uri::engine;

pub mod config;
pub mod helper;
pub mod system;
pub mod uri;

use crate::config::{args::CliArgs, search::Search};

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
    let exact = format::format_exact_search(Self::process_option(args.exact, "", "%"));
    let inurl = Self::process_option(
      args.inurl,
      "inurl:",
      format!(" {} inurl:", operator).as_str(),
    );

    Self {
      search,
      intitle,
      site,
      filetype,
      exact,
      operator,
      inurl,
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
  let engine = args.engine.clone();
  let uri_args = Search::new(args);
  let search_uri = uri::make_uri(uri_args);

  if search_uri.is_empty() {
    return Err("No search terms provided. See `findit --help` for more information.");
  }

  Ok(engine::search_engine(engine, search_uri))
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
      inurl: "graphql".into(),
    };

    let expect = "query+graphql+query+graphql+medium.com+pdf+graphql";

    assert_eq!(uri::make_uri(s), expect);
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
