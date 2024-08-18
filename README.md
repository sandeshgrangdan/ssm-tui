# AWS Systems Manager - Parameter Store TUI Client

This project serves as a learning exercise in [Rust](https://www.rust-lang.org/), [Tokio](https://tokio.rs/), [Channels](https://tokio.rs/tokio/tutorial/channels), and TUI (Terminal User Interface) programming. It is a DevOps tool used to manages existing Parameter Store with a Terminal User Interface (TUI), utilizing technologies such as Tokio, Ratatui and Vim.

<img width="1166" alt="Screenshot 2024-08-17 at 10 50 15 PM" src="https://github.com/user-attachments/assets/2ef5e441-6b99-4eea-86b2-b553bb788cea">

## Prerequisite
1. [Install](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and [configure](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html) AWS CLI.
2. [Vim](https://github.com/vim/vim)
3. Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

**Note**: This project is not suitable for production use. It's designed strictly for educational purposes.

## Installation

The binary executable is `spt`.

### Homebrew

For both macOS and Linux

```bash
brew install ssm-tui
```

To update, run

```bash
brew upgrade ssm-tui
```

### Snap

For a system with Snap installed, run

```bash
snap install spt
```

The stable version will be installed for you automatically.

If you want to install the nightly build, run

```bash
snap install spt --edge
```

### AUR

For those on Arch Linux you can find the package on AUR [here](https://aur.archlinux.org/packages/ssm-tui/). If however you're using an AUR helper you can install directly from that, for example (in the case of [yay](https://github.com/Jguer/yay)), run

```bash
yay -S ssm-tui
```

### Nix

Available as the package `ssm-tui`. To install run:

```bash
nix-env -iA nixpkgs.ssm-tui
```

Where `nixpkgs` is the channel name in your configuration. For a more up-to-date installation, use the unstable channel.
It is also possible to add the package to `environment.systemPackages` (for NixOS), or `home.packages` when using [home-manager](https://github.com/rycee/home-manager).

### Void Linux

Available on the official repositories. To install, run

```bash
sudo xbps-install -Su ssm-tui
```

### Fedora/CentOS

Available on the [Copr](https://copr.fedorainfracloud.org/coprs/atim/ssm-tui/) repositories. To install, run

```bash
sudo dnf copr enable atim/ssm-tui -y && sudo dnf install ssm-tui
```

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

## Setup Instructions (Development)

To get the project up and running, follow these steps:

1. Clone the repository: `git clone git@github.com:sandeshgrangdan/ssm-tui.git`
2. Start the tui: `cargo run`
