use clap::Parser;
use findit::run;
use findit::CliArgs;

mod common;

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

#[test]
fn test_and_operator() {
  let result = run(CliArgs::parse_from(&[
    "findit",
    "-k",
    "javascript gc",
    "-i",
    "gc",
    "-i",
    "gargage collector",
    "-o",
    "AND",
    "-e",
    "gc",
    "-s",
    "medium.com",
  ]));

  let expect = "https://www.google.com/search?q=\"gc\"+javascript+gc+intitle:gc+AND+intitle:gargage+collector+site:medium.com";

  match result {
    Ok(search_uri) => assert_eq!(search_uri, expect),
    Err(error) => panic!("{}", error),
  }
}
