# Reverie

[![](https://badgen.net/github/checks/yuma140902/Reverie/master/ubuntu?label=linux)](https://github.com/yuma140902/Reverie/actions/workflows/rust.yml)
[![](https://badgen.net/github/checks/yuma140902/Reverie/master/windows?icon=windows)](https://github.com/yuma140902/Reverie/actions/workflows/rust.yml)
[![](https://badgen.net/github/checks/yuma140902/Reverie/master/macos?icon=apple)](https://github.com/yuma140902/Reverie/actions/workflows/rust.yml)
[![](https://badgen.net/crates/v/reverie-engine?color=blue)](https://crates.io/crates/reverie-engine)
[![](https://docs.rs/reverie-engine/badge.svg)](https://docs.rs/reverie-engine/)

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
