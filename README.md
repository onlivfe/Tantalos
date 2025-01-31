# Onlivfe Tantalos

A rust-native [onlivfe app](https://onlivfe.com) built using [freya](https://freyaui.dev/).

The [Tántalos](https://en.wikipedia.org/wiki/Tantalus) name refers to how the perfect pure rust stack always feels so close.
Yet after so many rewrites there still isn't a solution thsat I've been able to find acceptable & stick to.
List of victims of my perfectionism include:

- [Freya](https://freyaui.dev/) (too early, so many things broken)
- [Slint](https://slint.dev/) (theming seems painful)
<!-- - [Dioxus](https://dioxuslabs.com/) (wasm) -->
- [Leptos](https://github.com/leptos-rs/leptos) (wasm)
- [yew](https://yew.rs) with [Tauri](https://tauri.app/) (wasm)
- [egui](https://github.com/emilk/egui) (state management & async issues)

## Development

Basic requirements:

- [Git](https://git-scm.com)
- [Rust](https://www.rust-lang.org/)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/) (`cargo install dioxus-cli`)

### Building

Start off by cloning the project with git.

```sh
git clone https://github.com/onlivfe/Tantalos
```

Then open this folder and run `dx serve --platform {desktop/web/mobile}`

Then get to hacking, & optionally replace the dependency in other projects by [overriding dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html).

To create a published version of the gui, use [`cargo-packager`](https://github.com/crabnebula-dev/cargo-packager) (`cargo install cargo-packager --locked`): `cargo packager --release -p tantalos`

### Translations

Edit the `res/i18n/{lang}.flt` files.
They are defined in the [Fluent localization system](https://projectfluent.org/)'s syntax.
If a locale is missing, create a new file and submit the changes.

## License

TBD, most like AGPL. In the meanwhile, I at least grant you the permission to view and redistribute the source code as-is, without any warranty. Do remember that you still need to follow the dep crates' licenses too.
