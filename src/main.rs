use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(author = "Maurício W. <mauriciobw17@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Advanced web search", long_about = None)]
#[command(ignore_errors = true)]
struct Args {
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
  fn new(args: Args) -> Self {
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

fn make_search_url(args: Args) -> String {
  let s = Search::new(args);
  // TODO: formatar key se o um valor para ela for passado
  //   let search_string = format!("{} {} {} {}", s.search, s.intitle, s.filetype, s.site);
  let search_string = format!("{} {} {} {}", s.search, s.intitle, s.site, s.filetype)
    .trim()
    .replace(" ", "+");
  let search_url: String = format!("https://www.google.com/search?q={}", search_string);
  search_url
}

fn main() {
  let args = Args::parse();
  let search_url = make_search_url(args);
  println!("{:?}", search_url);
}
