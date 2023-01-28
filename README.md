<img src="./assets/docs/banner.png">

# LemonCord
A fast & light-weight Discord Client written in Rust using the [wry](https://docs.rs/wry/) crate.

## Features
- Fast, light-weight, easy to use.
- 100% Open sourced.
- No suspicious activity unlike some other modded clients.
- Cross platform support.

## Is this against the Discord ToS?
Long story short.. LemonCord is "possibly" against the Discord Terms-of-Service, LemonCord is an embedded version of the [Discord](https://discord.com/app) website. It cannot be guaranteed if LemonCord is against ToS or not, but you should keep reading:

As of right now, Discord is not going out of their way to detect client mods or ban client mod users. On top of that, LemonCord does not make any manual HTTP requests unlike certain client mods / plugins, so your client's user agent is the same as a legitimate client. Meaning Discord doesn't detect a client app like LemonCord. They can go our of their way to start detecting it but they don't. Even if they somwhow did start detecting LemonCord, users are very unlikely to be banned on sight. It doesn't make sense for Discord to start banning a substantial part of it's userbase (unofficial client users) without any kind of warning. Not to mention that LemonCord doesn't have any type of client modifications or anything suspicious, implying LemonCord users can't be banned for indirect ToS violations (e.g. selfbotting).

## Platform-specific notes
All platforms use [TAO](https://github.com/tauri-apps/tao) to build the window, and [wry](https://github.com/tauri-apps/wry) re-exports it as an application module. Here's the underlying web engine each platform uses, and some dependencies you might need to install before running [LemonCord](https://github.com/japandotorg/LemonCord).

<details>
<summary> Linux </summary>

Tao uses [gtk-rs](https://gtk-rs.org/) and it's related libraries for window creation and wry also needs [WebKitGTK](https://webkitgtk.org/) for WebView. So please make sure the following packages are installed:

**Arch based distrobutions:**
```sh
$ sudo pacman -Syu webkit2gtk-4.1 libappindicator-gtk3
```

The `libayatana-indicator` package can be installed from the Arch User Repository (AUR).

**Debain based distrobutions**
```sh
$ sudo apt install libwebkit2gtk-4.1-dev libayatana-appindicator3-dev
```

**Fedora:**
```sh
$ sudo dnf install gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel
```

Fedora does not have the Ayatana package yet, so you need to use the GTK one.

</details>

<details>

<summary> MacOS </summary>

WebKit is native on macOS so everything should be fine.

If you are cross-compiling for macOS using [osxcross](https://github.com/tpoechtrager/osxcross) and encounter a runtime panic like Class with name `WKWebViewConfiguration could not be found` it's possible that `WebKit.framework` has not been linked correctly, to fix this set the `RUSTFLAGS` environment variable:
```sh
RUSTFLAGS="-l framework=WebKit" cargo build --target=x86_64-apple-darwin --release
```

</details>

<details>
<summary> Windows </summary>

WebView2 provided by Microsoft Edge Chromium is used. So wry supports Windows 7, 8, 10 and 11.

</details>

## Installation

> **Note**  
> In the future, there will be a GUI installer for general users. For now, you will have to use the
> command line.

First of all, to install LemonCord you'll have to install [Rust](https://rustup.rs/) in your machine.
You can check if you've successfully installed Rust or not using the following command below:
```sh
 # To check the currently active rustc compiler version, run
$ rustc --version
```

Alright, now that we have rust installed, it's time to download LemonCord
```sh
 # Download from github
$ git clone https://github.com/japandotorg/LemonCord.git
$ cd LemonCord
 
 # Build LemonCord
$ cargo build --release
```

You can run the executable everytime by either running the following command:
```sh
$ cargo run --release
```
or you can wish to use the executable file you just built using the `cargo build --release` command, to use the executable instead, run the following commands first:
```sh
 # *nix
$ sudo mv /LemonCord/target/release/lemon-cord /usr/local/bin/LemonCord
$ LemonCord  # Opening!

 # Windows
move /LemonCord/target/release/lemon-cord.exe /LemonCord/
 # Open "lemon-cord.exe"
```

## Discord / Support

Join our [Discord Server](https://melonbot.io/support) for further updates, announcements and Support, or just to chat with the developer.

## Contributors

For information on contributing to this project, please see [CONTRIBUTING.md](/CONTRIBUTING.md).

[![♥ Contributors][contributors-image]][contributors-link]

[contributors-image]: https://contrib.rocks/image?repo=japandotorg/LemonCord
[contributors-link]: https://github.com/japandotorg/LemonCord/graphs/contributors
