# Onlivfe app_rs

A rust-native [onlivfe app](https://onlivfe.com) built using [tauri](https://tauri.app/) and [yew](https://yew.rs/), working as a more ugly alternative to the [VRPeeps app](https://github.com/onlivfe/desktop) that's built with [Angular](https://angular.io/) & [tauri](https://tauri.app).

Also note that the license is [AGPL](https://tldrlegal.com/license/gnu-affero-general-public-license-v3-(agpl-3.0)).

## Development

Basic requirements:

- [Git](https://git-scm.com)
- [Rust](https://www.rust-lang.org/)
- [Tauri CLI](https://crates.io/crates/tauri-cli) (`cargo install tauri-cli`)
- [Trunk](https://trunkrs.dev/) (`cargo install --locked trunk`)

### Building

Start off by cloning the project with git.

```sh
git clone https://github.com/onlivfe/app_rs
```

Then open the project folder in your terminal, & run `cargo tauri dev` for the whole build, or `trunk serve` for just the UI.
Then get to hacking, & optionally replace the dependency in other projects by [overriding dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html).
