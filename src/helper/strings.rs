#![allow(dead_code)]

pub mod format {
  pub fn format_exact_search(exact: String) -> String {
    exact
      .split("%")
      .into_iter()
      .map(|s| {
        if !s.is_empty() {
          return format!("\"{}\" ", s);
        }
        String::new()
      })
      .collect::<String>()
      .trim()
      .to_string()
  }
}
