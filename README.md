# ShellCommander

## Project Description

[ShellCommander][repo-cmd] is a collection of Rust ports of scripts that I've wrote or found online. The goal is to port all of my scripts to create commands that can be run from any shell. Not just nushell or powershell.

Shell scripts can be found [here][repo-shell].

## Installation

The project requires [git][git-site] to be installed on the system.

To install the Rust crate, run the following command:

```sh
cargo install --path "."
```

To develop the crate make sure that the rust nightly toolchain is installed.

### Diesel Setup

[Diesel][repo-diesel] currently manages the databases.

> Linux installation
>
> ```sh
> curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/> diesel_cli-installer.sh | sh
> ```

> Windows installation
>
> ```powershell
> Set-ExecutionPolicy RemoteSigned -scope CurrentUser
> irm https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.ps1 | iex
> ```

Create a `.env` file in the cargo directory that looks like

```plain
DATABASE_URL=path_to_db
```

#### Alternatively

You can also run the following command after installing the crate.

```sh
ShellCommander env generate-dot-env
```

#### Database Generation

```sh
diesel migration run
```

This will generate the sql database file.

## Usage

In order to see the commands provided execute the following command:

```sh
ShellCommander help
```

## Credits

Git integrations use some code from [Git2][git2-example] examples.

### Dependencies

```toml
[dependencies]
chrono = "0.4.39"
clap = { version = "4.5.27", features = ["color", "derive", "suggestions"] }
clap_complete = "4.5.44"
clap_complete_nushell = "4.5.4"
derive = "1.0.0"
directories = "6.0.0"
git2 = "0.20.0"
lazy_static = "1.5.0"
open = "5.3.1"
rand = "0.9.0"
reqwest = { version = "0.12.12", features = ["blocking"] }
libsqlite3-sys = { version = "*", features = ["bundled"] }
diesel = { version = "2.2.7", features = [
  "chrono",
  "sqlite",
], default-features = false }
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.138"
tokio = { version = "1.43.0", features = ["full"] }
toml = "0.8.19"
```

## License

This project is licensed under the [GNU General Public License][license] v3.0. See the [LICENSE][license-file] file for details.

[repo-cmd]: https://github.com/Spyder337/cmd
[repo-shell]: https://github.com/Spyder337/nu-config
[repo-diesel]: https://diesel.rs
[git2-example]: https://github.com/rust-lang/git2-rs/blob/master/examples
[git-site]: https://git-scm.com
[license]: https://www.gnu.org/licenses/gpl-3.0.en.html
[license-file]: LICENSE.md
