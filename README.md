# gitignore

_An [Alfred] workflow to quickly create a `.gitignore` file from templates._

This is an [Alfred] workflow that lets users quickly combine multiple
`.gitignore` [templates] into a single file. It works offline, has suggestions
and autocomplete, and works with [Alfred] 3 and 4.

_Built with_ ❤️ _and_ 🦀 _by [jdno]._

## CLI

Under the hood, `alfred-gitignore` is a simple command-line utility that can be
used outside of [Alfred] as well.

**CLI mode is purely for testing, and is not officially supported.**

The CLI mode requires the `--repository` argument, which must be a path to a
folder that `alfred-gitignore` can use to store its data. As with [Alfred], the
templates must be downloaded first before they can be used with other commands.

When working on `alfred-gitignore`, run the CLI with the following command:

    cargo run -- --help

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[alfred]: https://www.alfredapp.com
[jdno]: https://github.com/jdno
[rust]: https://rust-lang.org
[templates]: https://github.com/github/gitignore
