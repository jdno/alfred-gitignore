# gitignore

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/jdno/alfred-gitignore/Rust)](https://github.com/jdno/alfred-gitignore/actions)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/jdno/alfred-gitignore)](https://github.com/jdno/alfred-gitignore/releases)

_An [Alfred] workflow to quickly create a `.gitignore` file from templates._

This is an [Alfred] workflow that lets users quickly combine multiple
`.gitignore` [templates] into a single file. It works offline, has suggestions
and autocomplete, and works with [Alfred] 3 and 4.

_Built with_ ❤️ _and_ 🦀 _by [jdno]._

## Getting Started

The latest version of the workflow can be downloaded from [Packal] or from the
[releases] on GitHub. Open the workflow with [Alfred], and follow its
instructions to set it up.

The workflow can be started by typing the following keyword into [Alfred]:

    gitignore

**Important!** `alfred-gitignore` is shipped as an unsigned binary, which in
recent versions of macOS this will prompt a warning. Go to `System Preferences`
in macOS, click on `Security & Privacy`, and allow `alfred-gitignore` to run.

When running the workflow for the first time, only an action to update the
templates will be shown. Run this action to download the [latest templates from
GitHub][templates]. When the download is done, press `Enter` to start building a
`.gitignore` file.

The worflow shows a list of the currently installed workflows. Select one, and
press `Enter` to add it to the list. Do this for all templates you want to
combine. Then select the `Build` action at the top of the list, and wait for the
file to be created.

Once ready, the workflow prompts you to either open the file or copy it to the
clipboard. Select your preferred option and hit `Enter` to finish the workflow.

## Contributing

✨ Thanks for your interest in making this workflow better! 👋

### Report a bugs

Found an bug? Please check the [issues] to see if a similar problem has already
been reported. If so, feel free to add a comment. This helps me understand if
the issue is limited to a single user or more widely spread. If there isn't an
issue yet, create one and describe the bug in as much detail as possible. Some
are really difficult to reproduce, and the more information you add the more
likely it is that I can reproduce and fix the problem.

### Request a feature

Have an idea you'd like to see in `alfred-gitignore`? Open an [issue][issues]
and propose it! Be precise, but don't invest too much time yet. As the
maintainer of the workflow, I have to decide whether I can support your feature
in the future, and might have to decline it because it'll be too complex or too
much work.

### Contribute code

Interested in working on `alfred-gitignore`? Help fixing bugs is always welcome.
If you want to implement a new feature, please reach out to me first and
[request the feature](#request-a-feature) to make sure it is a good fit for the
workflow.

#### Install dependencies

`alfred-gitignore` is written in [Rust], and working on the project requires a
working [Rust] environment. Check the official documentation to learn how to
install [Rust] on your local machine.

The project uses [pre-commit] to configure a wide range of Git pre-commit-hooks.
These hooks enforce a clean and consistent code style. The hooks are also run
during CI, so it is not absolutely necessary to install them locally. But it is
strongly recommended if you plan to do more than just change a few lines.

#### Set up project

Clone the repository, open a terminal, and install the [pre-commit] hooks. These
hooks are run when committing code, and enforce a consistent code style.

    pre-commit install

Then test that Rust is working correctly by building the project:

    cargo build

#### Write code

Implement your changes to `alfred-gitignore`, and make sure to write tests for
them as well. Run [Rustfmt](https://github.com/rust-lang/rustfmt) and
[Clippy](https://github.com/rust-lang/rust-clippy) and fix any issues they bring
up. When you're done, push your work in a new branch and open a
[pull request][pull-requests].

Please follow a few guidelines when working on `alfred-gitignore`. As the
maintainer, I am responsible for your code once it is merged, and that is only
possible when it is clean code.

- Write tests for your code.
- Document public interfaces.
- Follow the coding style (`rustfmt` and `clippy`).

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
[issues]: https://github.com/jdno/alfred-gitignore/issues
[jdno]: https://github.com/jdno
[packal]: https://www.packal.org/workflow/gitignore-0
[pre-commit]: https://pre-commit.com/
[pull-requests]: https://github.com/jdno/alfred-gitignore/pulls
[releases]: https://github.com/jdno/alfred-gitignore/releases
[rust]: https://rust-lang.org
[templates]: https://github.com/github/gitignore
