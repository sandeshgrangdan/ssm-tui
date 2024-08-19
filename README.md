# AWS Systems Manager - Parameter Store TUI Client

This project serves as a learning exercise in [Rust](https://www.rust-lang.org/), [Tokio](https://tokio.rs/), [Channels](https://tokio.rs/tokio/tutorial/channels), and TUI (Terminal User Interface) programming. It is a DevOps tool used to manages existing Parameter Store with a Terminal User Interface (TUI), utilizing technologies such as Tokio, Ratatui and Vim.

<img width="1166" alt="Screenshot 2024-08-17 at 10 50 15 PM" src="https://github.com/user-attachments/assets/2ef5e441-6b99-4eea-86b2-b553bb788cea">

## Prerequisite
1. [Install](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and [configure](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html) AWS CLI.
2. [Vim](https://github.com/vim/vim)
3. Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

**Note**: This project is not suitable for production use. It's designed strictly for educational purposes.

## Installation

### Cargo

Use this option if your architecture is not supported by the pre-built binaries found on the [releases page](https://github.com/Rigellute/ssm-tui/releases).

First, install [Rust](https://www.rust-lang.org/tools/install) (using the recommended `rustup` installation method) and then

```bash
cargo install ssm-tui
```
This method will build the binary from source.

To update, run the same command again.
```
git clone git@github.com:sandeshgrangdan/ssm-tui.git
cargo build --release
cd target/release/
./ssm-tui
```

### From binaries (Linux, macOS, Windows)

- Download the [latest release binary](https://github.com/sandeshgrangdan/ssm-tui/releases) for your system
- Set the `PATH` environment variable

## Usage

```
$ ssm-tui
```

```
$ ssm-tui -h
AWS Systems Manager - Parameter Store TUI Client

Usage: ssm-tui [OPTIONS]

Options:
  -p, --profile <PROFILE>  Name of your AWS profile [default: None]
  -r, --region <REGION>    AWS Region [default: None]
  -h, --help               Print help
  -V, --version            Print version
```

## Setup Instructions (Development)

To get the project up and running, follow these steps:

1. Clone the repository: `git clone git@github.com:sandeshgrangdan/ssm-tui.git`
2. Start the tui: `cargo run`
