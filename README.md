# Rust Apps Lints

[Dylint] lints for apps created with Rust.

A collection of lints for apps built with Rust. Initially started to check
patterns used in [Leptos] and [web-sys] based apps.

## Quick start

Install [Dylint] with

```sh
cargo install cargo-dylint dylint-link
```

Put the next configuration in the _Cargo.toml_ of your workspace.

<!-- markdownlint-disable line-length -->

```toml
[workspace.metadata.dylint]
libraries = [{ git = "https://github.com/mondeja/rs-apps-lints", tag = "v0.1.0", pattern = "lints/*" }]
```

<!-- markdownlint-enable line-length -->

But note that this will enable all lints in the workspace, which may not be what you want.

Run the lints with

```sh
cargo dylint --all
```

See `cargo dylint --help` for more information.

## Configuration

### Workspace

<!-- markdownlint-disable line-length -->

```toml
[workspace.metadata.dylint]
libraries = [{ git = "https://github.com/mondeja/rs-apps-lints", tag = "v0.1.0", pattern = "lints/*" }]
```

<!-- markdownlint-enable line-length -->

If you want to enable only, for example, lints for [web-sys] based apps, you can
use the `pattern` field to specify which lints to include.

<!-- markdownlint-disable line-length -->

```toml
[workspace.metadata.dylint]
libraries = [{ git = "https://github.com/mondeja/rs-apps-lints", tag = "v0.1.0", pattern = "lints/web-sys" }]
```
<!-- markdownlint-enable line-length -->

See all available lint libraries in the [lints directory] at _lints/_.

[lints directory]: https://github.com/mondeja/rs-apps-lints/tree/main/lints

### Lint levels

#### RUSTFLAGS

Use the `RUSTFLAGS` environment variable to set custom lint levels for each lint.

For example, to set [`web_sys_reexports`] lint to `deny`, run the next command.

```sh
RUSTFLAGS="-Dweb_sys_reexports" cargo dylint --all
```

Or in the file _.cargo/config.toml_ to avoid repeating the command.

```toml
[target.'cfg(all())']
rustflags = ["-Dweb_sys_reexports"]
```

The downside of this approach is that the project will be compiled from scratch
every time you edit the `RUSTFLAGS` variable.

#### Cargo.toml

Use `[lints.rust]` table in _Cargo.toml_ to set custom lint levels for each lint.

For example, to set [`web_sys_reexports`] lint to `deny` in a workspace,
add the next lines to the _Cargo.toml_ file.

```toml
[workspace.lints.rust]
unknown_lints = "allow"
web_sys_reexports = "deny"
```

The downside of this approach is that unknown lints will be allowed by default.

## Lints

### web-sys

Lints for [web-sys] based apps.

| Rule                    | Description                                                      |
| ----------------------- | ---------------------------------------------------------------- |
| [`web_sys_reexports`]    | Check for usages of third party library re-exports from web-sys. |

[`web_sys_reexports`]: https://github.com/mondeja/rs-apps-lints/tree/main/lints/web-sys/web_sys_reexports#readme

### leptos-more

More lints for Leptos apps.

This library is an expansion of [Leptos official lints]. The goal is to provide
additional lints that help developers follow best practices and avoid common
pitfalls in Leptos applications, but are not included in the official lints for
being too specific or opinionated.

[Leptos official lints]: https://github.com/leptos-rs/leptos-lints

| Rule                    | Description                                                     |
| ----------------------- | --------------------------------------------------------------- |
| [`leptos_reexports`]    | Check for usages of third party library re-exports from leptos. |

[`leptos_reexports`]: https://github.com/mondeja/rs-apps-lints/tree/main/lints/leptos-more/leptos_reexports#readme

[Dylint]: https://github.com/trailofbits/dylint
[Leptos]: https://leptos.dev
[web-sys]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
