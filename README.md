<img src="./assets/banner.png">

# LemonCord
[LemonCord](https://github.com/japandotorg/LemonCord), A fast & light-weight Discord Client written in Rust using the [wry](https://docs.rs/wry/) crate.

## Features
- Fast, light-weight, easy to use.
- 100% Open sourced.
- No suspicious activity unlike some other modded clients.
- Cross platform support.

## Is this against the Discord ToS?
Long story short.. LemonCord is "possibly" against the Discord Terms-of-Service, LemonCordis an embedded version of the [Discord](https://discord.com/app) website. It cannot be guaranteed if LemonCord is against ToS or not, but you should keep reading:

As of right now, Discord is not going out of their way to detect client mods or ban client mod users. On top of that, LemonCord does not make any manual HTTP requests unlike certain client mods / plugins, so your client's user agent is the same as a legitimate client. Meaning Discord doesn't detect a client app like LemonCord. They can go our of their way to start detecting it but they don't. Even if they somwhow did start detecting LemonCord, users are very unlikely to be banned on sight. It doesn't make sense for Discord to start banning a substantial part of it's userbase (unofficial client users) without any kind of warning. Not to mention that LemonCord doesn't have any type of client modifications or anything suspicious, implying LemonCord users can't be banned for indirect ToS violations (e.g. selfbotting).

## Platform-specific notes
All platforms use [TAO](https://github.com/tauri-apps/tao) to build the window, and [wry](https://github.com/tauri-apps/wry) re-exports it as an application module. Here's the underlying web engine each platform uses, and some dependencies you might need to install before running [LemonCord](https://github.com/japandotorg/LemonCord).

### Linux

Tao uses [gtk-rs](https://gtk-rs.org/) and it's related libraries for window creation and wry also needs [WebKitGTK](https://webkitgtk.org/) for WebView. So please make sure the following packages are installed:

**Arch Linux / Manjaro:**
```sh
$ sudo pacman -S webkit2gtk-4.1
$ sudo pacman -S libappindicator-gtk3
```

The `libayatana-indicator` package can be installed from the Arch User Repository (AUR).

**Debain / Ubuntu:**
```sh
$ sudo apt install libwebkit2gtk-4.1-dev
$ sudo apt install libayatana-appindicator3-dev
```

**Fedora:**
```sh
$ sudo dnf install gtk3-devel webkit2gtk4.1-devel
$ sudo dnf install libappindicator-gtk3-devel
```

Fedora does not have the Ayatana package yet, so you need to use the GTK one.

### MacOS

WebKit is native on macOS so everything should be fine.

If you are cross-compiling for macOS using [osxcross](https://github.com/tpoechtrager/osxcross) and encounter a runtime panic like Class with name `WKWebViewConfiguration could not be found` it's possible that `WebKit.framework` has not been linked correctly, to fix this set the `RUSTFLAGS` environment variable:
```sh
RUSTFLAGS="-l framework=WebKit" cargo build --target=x86_64-apple-darwin --release
```

### Windows

WebView2 provided by Microsoft Edge Chromium is used. So wry supports Windows 7, 8, 10 and 11.

## Installation

> **Note**  
> In the future, there will be a GUI installer for general users. For now, you will have to use the
> command line.

First of all, to install LemonCord you'll have to install [Rust](https://rust-lang.org) in your machine using the command given below:
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If the above command doesn't work for you and you're running Windows. To install Rust you'll have to download the installer [[32-BIT Rust Installer](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe) [64-BIT Rust Installer](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)], then run the program and follow the onscreen instructions. You may need to install the [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) when prompted to do so.

If you're having trouble with installing Rust, you should consider checking out the  [other-installation-methods](https://forge.rust-lang.org/infra/other-installation-methods.html) page of the official website.

You can check if you've successfully installed Rust or not using the following command below:
```sh
# To check the currently active rustc compiler version, run
$ rustc --version
# To check the currently active rustup toolchain manager, run
$ rustup --version
# To check the currently active cargo package manager, run
$ cargo --version
```

Alright, now that we have rust installed, it's time to clone and cd to the repository in your machine using the following commands:
```sh
$ git clone https://github.com/japandotorg/LemonCord.git
$ cd LemonCord
```

Then you can start building the executable by running the following command:
```sh
$ cargo build --release
```

You can run the executable everytime by either running the following command:
```sh
$ cargo run --release
```
or you can wish to use the executable file you just built using the `cargo build --release` command, to use the executable instead, run the following commands first:
```sh
# Linux / MacOS
mv /LemonCord/target/release/lemon-cord.exe /LemonCord
# Windows
move /LemonCord/target/release/lemon-cord.exe /LemonCord/
```

After that, open the `lemon-cord.exe` executable from your folder.
You should be able to use Discord just as you could with any other client.

## Discord / Support

Join our [Discord Server](https://melonbot.io/support) for further Updates, Announcements, Support or just to chat with the developer.

## Contributors

For information on contributing to this project, please see [CONTRIBUTING.md](/CONTRIBUTING.md).

[![Contributors][contributors-image]][contributors-link]

[contributors-image]: https://contrib.rocks/image?repo=japandotorg/LemonCord
[contributors-link]: https://github.com/japandotorg/LemonCord/graphs/contributors
