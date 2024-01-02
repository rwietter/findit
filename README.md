# findit

Findit is a command-line tool that leverages Google's advanced search operators to refine your search results. It is built in Rust and provides a simple and intuitive interface for performing complex searches. 

## The Search Operators

Findit uses a variety of search operators to refine your search results. Here is a brief description of each operator:

| Search Operator | Description                                   |
| --------------- | --------------------------------------------- |
| `inTitle`       | Search for a keyword in the title             |
| `InText`        | Search for a keyword in the text              |
| `site`          | Display search results restricted to a domain |
| `filetype`      | Display search results based on the file type |
| `InURL`         | Search for a keyword in the URL               |
| `exact`         | Match the exact keyword in the text           |
| `OR`            | Search for a keyword or another keyword       |
| `AND`           | Search for a keyword and another keyword      |
| `exclude`       | Exclude a keyword from the search             |
| `related`       | Search for websites related to a domain       |
| `cache`         | Display the cached version of a website       |
| `info`          | Display information about a website           |
| `define`        | Display the definition of a keyword           |
| `weather`       | Display the weather of a location             |
| `map`           | Display a map of a location                   |

## Installation

```bash
git clone <repo>
cd findit
cargo build --release
./target/release/findit --help
```

## Usage

```bash
findit -k <search terms> -o <operator> -s <domain> -f <filetype> -i <inurl>
```

## Examples

```bash
findit -k "rust programming language" -s .medium.com -s .dev.to
findit -k rust lifetimes -i rust -i lifetimes -t programming -f pdf
```

```bash
Usage: findit [OPTIONS]

Options:
  -g, --engine <ENGINE>           Your search engine (google, duckduckgo, bing) [default: google]
  -k, --search [<SEARCH>...]      A simple search terms
  -t, --intitle [<INTITLE>...]    Search for a keyword in the title
  -s, --site [<SITE>...]          Display search results restricted to a domain
  -f, --filetype [<FILETYPE>...]  Display search results based on the file type
  -e, --exact [<EXACT>...]        Match the exact keyword in the text
  -o, --operator <OPERATOR>       Search for one (OR|AND) another keyword
  -i, --inurl [<INURL>...]        Search for a keyword in the URL
  -h, --help                      Print help
  -V, --version                   Print version
```

## Engine

Findit supports three search engines: Google, DuckDuckGo, and Bing. You can specify the search engine using the `-g` flag. The default search engine is Google.

```bash
findit -g duckduckgo -k rust lifetimes
```

Using environment variables, you can set the default search engine for all searches. 

```bash
export FINDIT_ENGINE=duckduckgo
```

Set the default search engine in your `.bashrc` or `.zshrc` file to persist the change.

```bash
# Bash
echo "export FINDIT_ENGINE=duckduckgo" >> ~/.bashrc

# Zsh
echo "export FINDIT_ENGINE=duckduckgo" >> ~/.zshrc
```

```bash
findit -k rust lifetimes
# > https://duckduckgo.com/?q=rust+lifetimes
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) for details