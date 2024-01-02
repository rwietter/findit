use std::env;

pub fn search_engine(engine: Option<String>, query: String) -> String {
  let env_engine = env::var("FINDIT_ENGINE");

  let engine = match env_engine {
    Ok(env) => env,
    Err(_) => {
      println!("No engine found in environment variable FINDIT_ENGINE. Using default or engine provided by `--engine` flag.");
      engine.unwrap()
    }
  };

  let engine = engine.as_str().trim();

  match engine {
    "google" => format!("https://www.google.com/search?q={query}"),
    "duckduckgo" => format!("https://duckduckgo.com/?q={query}"),
    "bing" => format!("https://bing.com/search?q={query}"),
    _ => {
      println!("Engine not found. Using Google Search Engine as default.");
      format!("https://www.google.com/search?q={query}")
    }
  }
}
