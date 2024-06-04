# Reverie

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/yuma140902/reverie/rust.yml?logo=github&label=CI)](https://github.com/yuma140902/reverie/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/reverie-engine)](https://crates.io/crates/reverie-engine)
[![docs.rs](https://img.shields.io/docsrs/reverie-engine?logo=docsdotrs)](https://docs.rs/reverie-engine/latest/reverie-engine/)

A toy game engine

## Examples

- `cargo run -p example-craft`
- `cargo run -p example-window`
- `cargo run -p example-raw`

## Links

- [GitHub](https://github.com/yuma140902/Reverie)
- [crates.io](https://crates.io/crates/reverie-engine)
- [Docs.rs](https://docs.rs/reverie-engine/)

## Development

### Changelog

See [CHANGELOG.md](./CHANGELOG.md).

Install [git-chglog](https://github.com/git-chglog/git-chglog). To generate the changelog of `reverie-engine`, run `git-chglog --output CHANGELOG.md`. To generate the changelog of `reverie-util`, run `git-chglog --config .chglog/config-util.yml --output CHANGELOG-util.md`

### Commit message

See [.gitmessage](./.gitmessage). It is recommended to run `git config commit.template .gitmessage`.

### Branch name

All branches related to `reverie-util` must have the `util-` prefix.
