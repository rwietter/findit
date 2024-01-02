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
  pub search: Option<Vec<String>>,
  #[arg(short = 't', long)]
  #[arg(num_args(0..))]
  pub intitle: Option<Vec<String>>,
  #[arg(num_args(0..))]
  #[arg(short, long)]
  pub site: Option<Vec<String>>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  pub filetype: Option<Vec<String>>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  pub exact: Option<Vec<String>>,
  #[arg(short, long)]
  pub operator: Option<String>,
  #[arg(short, long)]
  #[arg(num_args(0..))]
  pub inurl: Option<Vec<String>>,
}
