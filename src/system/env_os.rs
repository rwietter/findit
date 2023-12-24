pub fn open_browser() -> String {
  let os = std::env::consts::OS;
  match os {
    "linux" => "xdg-open".into(),
    "macos" => "open".into(),
    "windows" => "start".into(),
    _ => panic!("Sorry, your OS is not supported yet."),
  }
}
