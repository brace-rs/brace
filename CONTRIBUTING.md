# Contributing

Contribution to this project is welcomed but we ask that you adhere to the
following guidelines where applicable.

## Conduct

This project adopts the [Rust Code of Conduct][conduct] to provide a welcoming
environment for all contributors. Any queries or concerns relating to conduct
should be directed to the maintainers of this project and not the moderation
team as they are not affiliated.

## Developing

### Install rust

See the rust [installation documentation][install-rust] or use the following
command to install the rust toolchain installer `rustup`.

```sh
curl https://sh.rustup.rs -sSf | sh
```

### Install development tools

The following command installs `cargo-make`, a task runner used to simplify the
execution of development tasks. Consult the documentation or configuration file
to see the available commands.

```sh
cargo install cargo-make
```

The following commands can be used to setup your local development environment
to use git hooks that lint on commit and test on push. Be aware that the hooks
are dependent on the `cargo-make` tool as described above.

```sh
cargo install rusty-hook
rusty-hook init
```

[conduct]: https://github.com/rust-lang/rust/blob/master/CODE_OF_CONDUCT.md
[install-rust]: https://www.rust-lang.org/tools/install
