use clap::Parser;

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
}

struct Search {
  search: String,
  intitle: String,
  filetype: String,
  site: String,
}

impl Search {
  fn new(args: CliArgs) -> Self {
    let search = Self::process_option(args.search, "", " ");
    let intitle = Self::process_option(args.intitle, "intitle:", ",");
    let site = Self::process_option(args.site, "site:", " OR site:");
    let filetype = Self::process_option(args.filetype, "filetype:", ",");

    Self {
      search,
      intitle,
      site,
      filetype,
    }
  }

  fn process_option(option: Option<Vec<String>>, prefix: &str, separator: &str) -> String {
    option
      .map(|s| {
        if !s.is_empty() {
          format!("{}{}", prefix, s.join(separator))
        } else {
          String::from("")
        }
      })
      .unwrap_or_else(String::new)
  }
}

pub fn run(args: CliArgs) -> Result<String, &'static str> {
  let s = Search::new(args);
  let search_string = format!("{} {} {} {}", s.search, s.intitle, s.site, s.filetype)
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
  fn it_works() {
    let result = run(CliArgs::parse_from(&[
      "findit",
      "-k",
      "rust",
      "python",
      "-i",
      "rust",
      "python",
      "-s",
      "github.com",
      "gitlab.com",
      "-f",
      "pdf",
      "epub",
    ]));

    match result {
      Ok(search_uri) => {
        assert_eq!(
            search_uri,
            "https://www.google.com/search?q=rust+python+intitle:rust,python+site:github.com+OR+site:gitlab.com+filetype:pdf,epub"
          );
      }
      Err(e) => panic!("Error: {}", e),
    }
  }
}
