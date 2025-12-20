# Reverie

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/yuma140902/reverie/rust.yml?logo=github&label=CI)](https://github.com/yuma140902/reverie/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/reverie-engine)](https://crates.io/crates/reverie-engine)
[![docs.rs](https://img.shields.io/docsrs/reverie-engine?logo=docsdotrs)](https://docs.rs/reverie-engine/latest/reverie-engine/)

A toy game engine

## Example

```sh
cargo run -p example-misc
```

## Links

- [GitHub](https://github.com/yuma140902/Reverie)
- [crates.io](https://crates.io/crates/reverie-engine)
- [Documentation](https://yuma14.net/Reverie/)
- [API Documentation](https://docs.rs/reverie-engine/)

## Development

### Changelog

See [CHANGELOG.md](./CHANGELOG.md).

To generate changelog, install [git-cliff](https://github.com/orhun/git-cliff) and run

```sh
git checkout master
git cliff --config git-cliff.toml --tag [next version] -o CHANGELOG.md
git commit -m "release [next version]"
git tag [next version]
git push --tags
cargo publish -p reverie-engine
cargo publish -p reverie-util
```

### Commit message

See [.gitmessage](./.gitmessage). It is recommended to run `git config commit.template .gitmessage`.

