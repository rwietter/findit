use crate::config::search::Search;
use crate::system::env_os::open_browser;

pub fn open_uri(uri: String) {
  std::process::Command::new(open_browser())
    .arg(&uri)
    .output()
    .expect(
      format!(
        "[FAIL_TO_OPEN_URI]: sorry, isn't possible open the uri: {}",
        &uri
      )
      .as_str(),
    );
}

pub fn make_uri(s: Search) -> String {
  let values = vec![s.exact, s.search, s.intitle, s.site, s.filetype, s.inurl];

  values
    .iter()
    .filter(|value| !value.is_empty())
    .map(|value| value.replace(" ", "+"))
    .collect::<Vec<String>>()
    .join("+")
}
