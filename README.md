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

[Diesel][repo-diesel] currently manages the servers.

```sh
diesel migration run
```

This will generate the sql database file.

## Usage

### Git Commands

```sh
A set of command line utilities

Usage: cmd <COMMAND>

Commands:
  web    A set of web utilities
  git    A set of git utilities
  env    A set of utilities for interacting with the environment
  quote
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Git

```sh
Usage: cmd git <COMMAND>

Commands:
  new         Initialize a new git repository
  clone       Clone a repository
  list        List cloned repositories in the `git_dir`
  add-commit  Stage files and commit them
  ignore      A set of .gitignore utilities
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Web

```sh
A set of web utilities

Usage: cmd web <COMMAND>

Commands:
  search   Search google for a query
  history  View search history
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```sh
Usage: cmd web search [OPTIONS] <QUERY>

Arguments:
  <QUERY>  Search query

Options:
      --site <SITE>            Site to restrict search to
      --allintext <ALLINTEXT>  Search for text in the page
      --json <JSON>            Return results in JSON format [possible values: true, false]
  -h, --help                   Print help
```

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
[repo-diesel]: https://diesel.rs
[license]: https://www.gnu.org/licenses/gpl-3.0.en.html
[license-file]: LICENSE.md
