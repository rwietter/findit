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

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) for details