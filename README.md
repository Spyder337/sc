# ShellCommander

## Project Description

[ShellCommander][repo-cmd] is a collection of Rust ports of scripts that I've wrote/found online. The goal is to port all of my scripts to create commands that can be run from any shell. Not just nushell or powershell.

Shell scripts can be found [here][repo-shell].

## Installation

To install the Rust crate, run the following command:

```sh
cargo install cmd
```

To develop the crate make sure that the rust nightly toolchain is installed.

## Usage

Provide usage instructions and examples here. For example, calling the crate from the command line:

```sh
cmd help

Usage: cmd [COMMAND]

Commands:
  git         Git repo interactions.
  web-search  Search google.
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Git Commands

```sh
Usage: cmd git <COMMAND>

Commands:
  update         Stage changes and commit them.
  list           List repos in the user git directory.
  set-dir        Set the repo directory.
  get-dir        Get the repo directory.
  fetch-ignore   Fetch a .gitignore file.
  fetch-ignores  Display valid ignore files.
  read-me
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')
```

### Roadmap

#### Commands

- [x] git **(6/6)**

    Contains all the functionality for managing a git repo and easily interacting with repos on disk.

  - [x] update
  - [x] get-dir
  - [x] set-dir
  - [x] fetch-ignore
  - [x] fetch-ignores
  - [x] list
- [ ] web-search **(0/3)**

    Contains the functionality for performing google search queries.

  - [ ] No filter query
  - [ ] Domain filter
  - [ ] Complex query
- [ ] welcome **(0/1)**

    The message of the day that should be presented when a terminal boots up.

  - [ ] Message of the Day
- [ ] quotes **(0/4)**
  
    The quotes command will have four subcommands. These commands will allow for interacting with a local storage for quotes along with history of previous daily quotes. There will also be a link to a public API for providing quotes for in case the user wants to fetch a new quote.

    A potential providers: [Paper Quotes][api-paper-quotes], [Forbes Thoughts][api-forbes]

    > Note:
    > Forbes uses an API that provides for the quotes page. It's url is `http://www.forbes.com/forbesapi/thought/uri.json?enrich=true&query=1&relatedlimit=5`
    >
    >query=1: Query today
    >
    >relatedlimit=5: Only get up to five more related quotes

  - [ ] Daily Quote
  - [ ] Random Quote
  - [ ] Saved Quotes
  - [ ] Online Database

**Progress**: **6/14**

## Credits

List any contributors, third-party libraries, or other resources you want to acknowledge here.

### Dependencies

```toml
# Time crates
chrono = "0.4.39"
# Command Line Application
clap = { version = "4.5.23", features = ["derive", "suggestions"] }
# Command Line Completion generation
clap_complete = "4.5.38"
derive = "1.0.0"
# System Folders
directories = "5.0.1"
# Git interactions
git2 = "0.19.0"
# Initialize statics at runtime
lazy_static = "1.5.0"
reqwest = { version = "0.12.9", features = ["blocking"] }
# Local database
rusqlite = { version = "0.32.1", features = ["bundled"] }
# Used for parsing some toml data 
serde = { version = "1.0.216", features = ["derive"] }
tokio = { version = "1.42.0", features = ["full"] }
```

## License

This project is licensed under the [GNU General Public License][license] v3.0. See the [LICENSE][license-file] file for details.

[repo-cmd]: https://github.com/Spyder337/cmd
[repo-shell]: https://github.com/Spyder337/nu-config
[api-paper-quotes]: https://paperquotes.com
[api-forbes]: https://www.forbes.com/quotes/1/
[license]: https://www.gnu.org/licenses/gpl-3.0.en.html
[license-file]: LICENSE.md
