use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
#[command(author = "Maurício W. <mauriciobw17@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "Findit - A simple tool to advanced search in your browser", long_about = None)]
#[command(ignore_errors = true)]
pub struct CliArgs {
  #[arg(short = 'g', long)]
  #[arg(default_value = "google")]
  /// Your search engine (google, duckduckgo, bing)
  pub engine: Option<String>,
  #[arg(short = 'k', long)]
  #[arg(num_args(0..))]
  /// A simple search terms
  pub search: Option<Vec<String>>,
  #[arg(short = 't', long)]
  #[arg(num_args(0..))]
  /// Search for a keyword in the title
  pub intitle: Option<Vec<String>>,
  #[arg(num_args(0..))]
  #[arg(short, long)]
  /// Display search results restricted to a domain
  pub site: Option<Vec<String>>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  /// Display search results based on the file type
  pub filetype: Option<Vec<String>>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  /// Match the exact keyword in the text
  pub exact: Option<Vec<String>>,
  #[arg(short, long)]
  /// Search for one (OR|AND) another keyword
  pub operator: Option<String>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  /// Search for a keyword in the URL
  pub inurl: Option<Vec<String>>,
}
