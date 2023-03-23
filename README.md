# Onlivfe app_rs

A rust-native [onlivfe app](https://onlivfe.com) built using [egui](https://github.com/emilk/egui), working as a more trimmed down, lightweight, and ugly alternative to the [full onlivfe desktop app](https://github.com/onlivfe/desktop) that's built with web technologies & [tauri](https://tauri.app).

Also note that the license is [AGPL](https://tldrlegal.com/license/gnu-affero-general-public-license-v3-(agpl-3.0)).

## Development

Basic requirements:

- [Git](https://git-scm.com)
- [Rust](https://www.rust-lang.org/)

### Building

Start off by cloning the project with git.

```sh
git clone https://github.com/onlivfe/core
```

Then open the project folder in your terminal, & run `cargo build`.
Then get to hacking, & optionally replace the dependency in other projects by [overriding dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html).

### Android

WIP, based on <https://github.com/rust-windowing/android-ndk-rs#quick-start-hello-world-crate-on-android>

Install the Android NDK and SDK as per <https://github.com/rust-windowing/android-ndk-rs#1-install-the-android-ndk-and-sdk>

Install Android target

```sh
rustup target add aarch64-linux-android
```

Install cargo-apk

```sh
cargo install cargo-apk
```

Run the Android app

```sh
cargo apk run --lib
```
