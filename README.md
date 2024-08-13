# AWS Systems Manager - Parameter Store TUI Client

This project serves as a learning exercise in [Rust](https://www.rust-lang.org/), [Tokio](https://tokio.rs/), [Channels](https://tokio.rs/tokio/tutorial/channels), and TUI (Terminal User Interface) programming. It is a DevOps tool used to manages existing Parameter Store with a Terminal User Interface (TUI), utilizing technologies such as Tokio, Ratatui and Vim.

## Prerequisite
1. [Install](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and [configure](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html) AWS CLI
2. [Vim](https://github.com/vim/vim)
3. Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

**Note**: This project is not suitable for production use. It's designed strictly for educational purposes.

## Setup Instructions

To get the project up and running, follow these steps:

1. Clone the repository: `git@github.com:sandeshgrangdan/ssm-tui.git`
2. Start the server: `cargo run`

## Installation
```
git@github.com:sandeshgrangdan/ssm-tui.git
cargo build --release
cd target/release/
./ssm-tui
```