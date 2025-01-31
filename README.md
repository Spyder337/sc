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

### Diesel Setup

[Diesel][repo-diesel] currently manages the servers.

Create a `.env` file in the cargo directory that looks like

```plain
DATABASE_URL=path_to_db
```

#### Database Generation

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

Git integrations use some code from [Git2][git2-example] examples.

### Dependencies

```toml
[dependencies]
chrono = "0.4.39"
clap = { version = "4.5.23", features = ["color", "derive", "suggestions"] }
clap_complete = "4.5.38"
clap_complete_nushell = "4.5.4"
derive = "1.0.0"
diesel = { version = "2.2.7", features = ["chrono", "sqlite"] }
directories = "6.0.0"
git2 = "0.20.0"
lazy_static = "1.5.0"
open = "5.3.1"
rand = "0.9.0"
reqwest = { version = "0.12.9", features = ["blocking"] }
rusqlite = { version = "0.33.0", features = ["bundled"] }
serde = { version = "1.0.216", features = ["derive"] }
serde_json = "1.0.134"
tokio = { version = "1.42.0", features = ["full"] }
toml = "0.8.19"
```

## License

This project is licensed under the [GNU General Public License][license] v3.0. See the [LICENSE][license-file] file for details.

[repo-cmd]: https://github.com/Spyder337/cmd
[repo-shell]: https://github.com/Spyder337/nu-config
[repo-diesel]: https://diesel.rs
[git2-example]: https://github.com/rust-lang/git2-rs/blob/master/examples
[license]: https://www.gnu.org/licenses/gpl-3.0.en.html
[license-file]: LICENSE.md
