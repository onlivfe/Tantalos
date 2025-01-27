# Onlivfe Tantalos

A rust-native [onlivfe app](https://onlivfe.com) built using [slint](https://slint.rs/).

The [Tántalos](https://en.wikipedia.org/wiki/Tantalus) name refers to how the perfect pure rust stack always feels so close.
Yet after so many rewrites there still isn't a solution that I've been able to find acceptable & stick to.

## Development

Basic requirements:

- [Git](https://git-scm.com)
- [Rust](https://www.rust-lang.org/)
- [Slint](https://slint.rs/)

### Building

Start off by cloning the project with git.

```sh
git clone https://github.com/onlivfe/Tantalos
```

Then open the project folder in your terminal, & run `cargo run`.
Then get to hacking, & optionally replace the dependency in other projects by [overriding dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html).

### Translations

Run `res/lang/extract-translations.sh`, which will update the `res/lang/tantalos.pot` template file.
If you're creating a new translation, run `res/lang/new.sh [language-code]`, else run `res/lang/update.sh`.
For actually writing the translations, see [Slint docs on translating strings](https://docs.slint.dev/latest/docs/slint/guide/development/translations/#translating-strings).

After all of that, on recompilation cargo will run `res/lang/compile-translations.sh`, which will merge the source translations with a fallback for empty values.
If you want to only update the translations, when running with debug assertions that script can also be manually ran after translation changes without recompiling the app.
