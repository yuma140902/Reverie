# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0](https://github.com/yuma140902/reverie/compare/v0.3.0..v0.4.0) - 2025-06-01

### ✨ Features

- Multiple color on ColoredComponent - ([13f5f96](https://github.com/yuma140902/reverie/commit/13f5f966b3242469b75956fb7fc0cab1941c0921))
- [**breaking**] Rename Scene.new_entity to new_sprite_entity - ([0ca9de8](https://github.com/yuma140902/reverie/commit/0ca9de888fa6a056e34be0bfb403a98f207c5c78))
- Add ColoredComponent - ([3904fd0](https://github.com/yuma140902/reverie/commit/3904fd091f1d84123594bcb4affe7666428458a5))
- [**breaking**] Rename wgpu_wrapper module to render, WgpuResource struct to RenderingResource - ([ced9888](https://github.com/yuma140902/reverie/commit/ced9888df7934883eea2560e9bc6865f363bcfbb))
- Enable alpha_to_coverage_enabled - ([72d8c84](https://github.com/yuma140902/reverie/commit/72d8c84c9703fc5cbefc7e8e69d9a9742baf6a6a))
- Add depth texture - ([3b2eed1](https://github.com/yuma140902/reverie/commit/3b2eed1f702ef34dab29c4e6e8b3a0fc8b20d0fe))

### 🐛 Bug Fixes

- Constant - ([951591e](https://github.com/yuma140902/reverie/commit/951591e25ac54c0624891eb128e138c70c506d81))

### ♻️ Refactor

- Add render::uniform module - ([f118965](https://github.com/yuma140902/reverie/commit/f118965340a63a4b0c760792bc19a2e3e753da37))
- Rename render_pipeline to sprite_pipeline - ([15d85cc](https://github.com/yuma140902/reverie/commit/15d85cce6d234dab10d563437b784a4d63630d7e))
- Texture_bind_group_layout と uniform_bind_group はシェーダ固有の値なので sprite module へ移動 - ([5d4162b](https://github.com/yuma140902/reverie/commit/5d4162b32b26fa85c5cb144951c69cb3a25f6806))
- Shader constants and binding layouts - ([0bf6a42](https://github.com/yuma140902/reverie/commit/0bf6a42383ff39471e75eff93325cbcefbd6394f))
- Move SpriteRenderPipeline to wgpu_wrapper - ([5dc8342](https://github.com/yuma140902/reverie/commit/5dc83429e7c83b261fc358e6fc2cd094a5efc5c0))
- Move Vertex trait to wgpu_wrapper - ([858372d](https://github.com/yuma140902/reverie/commit/858372d8c6c99e570b532d8dc8fc0e524285ce1c))
- Add SpriteRenderPipeline - ([481a74b](https://github.com/yuma140902/reverie/commit/481a74bfd4f9c87d6457f21f4b8c5c8eefa6bec3))
- Add Vertex trait - ([a62ba6d](https://github.com/yuma140902/reverie/commit/a62ba6db9cd0f320ead224ce521d11dd2d13d77c))
- Make VertexIndexBuffer generic to vertex struct - ([b35c711](https://github.com/yuma140902/reverie/commit/b35c711c181311ec6f4c86c039f0c892a5e57509))
- Rename UvVertex to SpriteVertex - ([9b15057](https://github.com/yuma140902/reverie/commit/9b150571f010f4119f4199eb86316690f2af6e33))

### 📝 Documentation

- Update - ([1797703](https://github.com/yuma140902/reverie/commit/17977034023939217ba3070479bdf50d085146c1))

### 🔀 Pull Requests

- [#171](https://github.com/yuma140902/reverie/pull/171)
- [#168](https://github.com/yuma140902/reverie/pull/168)

## [0.3.0](https://github.com/yuma140902/reverie/compare/v0.2.0..v0.3.0) - 2025-05-31

### ✨ Features

- More const fn - ([1a6e348](https://github.com/yuma140902/reverie/commit/1a6e3488a68dca993c20f0cfc8c6d1a220e22be2))
- Update MSRV, use edition 2024 - ([93b7164](https://github.com/yuma140902/reverie/commit/93b7164a2c9f0902314eb2e5944b8bf5d2978708))
- Add parameters to OrthographicCamera - ([f93a49c](https://github.com/yuma140902/reverie/commit/f93a49cd818576790f6b855649481da6df41784c))
- Add PerspectiveCamera - ([8d07aec](https://github.com/yuma140902/reverie/commit/8d07aecaa074801238ace6a215bdf74ca2cff7ef))
- Add OrthographicCamera - ([7091968](https://github.com/yuma140902/reverie/commit/7091968c9847f4f224d1a06a22e0ae9e0d861300))
- Add Viewport - ([e44f4b7](https://github.com/yuma140902/reverie/commit/e44f4b7e238973953a6cc615c328a30b6c6014f9))

### ♻️ Refactor

- Use TransformComponent in PerspectiveCamera - ([a0cd85e](https://github.com/yuma140902/reverie/commit/a0cd85e22aefeb32b282f4d68be175ac82a0bb86))
- Remove unused import - ([8677074](https://github.com/yuma140902/reverie/commit/86770743c2077b06084ec2718ba63d69e5767912))
- Remove unused imports - ([3a0009f](https://github.com/yuma140902/reverie/commit/3a0009fac91abd67627879f0fedefecfaf222a0e))
- Remove OPENGL_TO_WGPU_MATRIX - ([c4dc46c](https://github.com/yuma140902/reverie/commit/c4dc46c2547318c63fe6d3d0758f8c4db371c2cd))

### 📝 Documentation

- Add github actions to publish docs - ([5de08df](https://github.com/yuma140902/reverie/commit/5de08dfacb9932a8fd8dbe8f49fc8a3e98b4fa9b))
- 概要を追加 - ([e91451a](https://github.com/yuma140902/reverie/commit/e91451a99fd61e6f0cca46218fe95fdf93c48449))
- Language=ja - ([276983c](https://github.com/yuma140902/reverie/commit/276983c79522dfc69729b582acb6e6aeb559a3f3))
- 日本語検索 - ([9d12359](https://github.com/yuma140902/reverie/commit/9d123598b0818ca1e2d620f668e7f00816f7dd94))
- Setup mdbook - ([65b50bc](https://github.com/yuma140902/reverie/commit/65b50bc9523880327b11df65360cefa6b8aa11a4))
- 座標系に関するドキュメント - ([a676d58](https://github.com/yuma140902/reverie/commit/a676d58043a38002760f9c6bc1acb366e71b9b07))

### 👷 Build System

- Add support github actions cache service v2 - ([72a9502](https://github.com/yuma140902/reverie/commit/72a950273dc7bdda93fb2de94145df212075d378))
- Add rust-toolchain.toml - ([8da8949](https://github.com/yuma140902/reverie/commit/8da8949880b317b7dab3ddc188cd9a7409b4b5bf))
- Update dependencies - ([67676a4](https://github.com/yuma140902/reverie/commit/67676a465f5d4694443db9b23e99ebf9dc2ce332))
- Add Cargo.lock - ([d10e118](https://github.com/yuma140902/reverie/commit/d10e1187fd3bbc10df2be9bec2a599441c47adb1))

### 🔧 Miscellaneous Tasks

- Use new method - ([86f51ca](https://github.com/yuma140902/reverie/commit/86f51cac0922f62df6bbcc868a110c3b965196bf))
- Clippy fix - ([6d9bbde](https://github.com/yuma140902/reverie/commit/6d9bbdedd87ff5cba2aa458773e1c59a1cef62df))

### 🔀 Pull Requests

- [#163](https://github.com/yuma140902/reverie/pull/163)
- [#162](https://github.com/yuma140902/reverie/pull/162)
- [#161](https://github.com/yuma140902/reverie/pull/161)
- [#160](https://github.com/yuma140902/reverie/pull/160)
- [#143](https://github.com/yuma140902/reverie/pull/143)

### Tmp

- With_transform - ([4473466](https://github.com/yuma140902/reverie/commit/4473466c75c859ab01b8c705eb741f0e9328a592))

## [0.2.0](https://github.com/yuma140902/reverie/compare/v0.1.0..v0.2.0) - 2025-05-01

### ✨ Features

- [**breaking**] Remove reverie-engine-opengl - ([7181b96](https://github.com/yuma140902/reverie/commit/7181b96c66afde309d5307672f6e9358265889d2))
- [**breaking**] Fix typo `create_altas_texture` -> `create_atlas_texture` - ([aae1e70](https://github.com/yuma140902/reverie/commit/aae1e70597295840d37266ec60769eee79cbf96d))
- Update TransformComponent methods - ([aa85321](https://github.com/yuma140902/reverie/commit/aa85321cc945075dad236743f5df3f5a3b8abde1))
- Remove generational-arena and use slotmap instead - ([3ad2502](https://github.com/yuma140902/reverie/commit/3ad250298a57b64cf2a9bbdb63555a80455c7f9e))
- Update wgpu to v23.0.0 - ([29f6d22](https://github.com/yuma140902/reverie/commit/29f6d22972335da3488e42c2ad93321afccc2135))

### 🐛 Bug Fixes

- Lag on closing windows - ([df10819](https://github.com/yuma140902/reverie/commit/df10819a0e3af5deeb44fefcd1880cb36ad0ad0f))

### ♻️ Refactor

- Clippy fix - ([3a3a8dc](https://github.com/yuma140902/reverie/commit/3a3a8dc10969a612839303e1c9d71fd4a8158374))
- Elide lifetimes - ([47d7f84](https://github.com/yuma140902/reverie/commit/47d7f843c692cf6c9b6b05343f5d5cc1a4c06c85))
- Rename wgpu_layer to wgpu_wrapper - ([2f02d05](https://github.com/yuma140902/reverie/commit/2f02d053cb5dff119def3d97c546201afdca105f))
- Wgpu_layer module - ([764d262](https://github.com/yuma140902/reverie/commit/764d262b68fe67d7b9db098706d65e3cd0c1ed69))
- Scene module - ([1e5c099](https://github.com/yuma140902/reverie/commit/1e5c099265b8f6b6c3d85f087c09c34424f872db))
- Game module - ([6cb4d3e](https://github.com/yuma140902/reverie/commit/6cb4d3e973fdcc36d4f03d3c2f9f56fa8fc36cc8))
- Enable extra lints - ([10e4775](https://github.com/yuma140902/reverie/commit/10e47751bfed350dc7a1279b589e608045981de8))

### 👷 Build System

- *(deps)* Update pollster requirement from 0.3.0 to 0.4.0 - ([c5d6f55](https://github.com/yuma140902/reverie/commit/c5d6f5511b60e2f98e441f53b6b03695122e6866))
- *(reverie-util)* Use workspace dependencies - ([dfa194c](https://github.com/yuma140902/reverie/commit/dfa194cc9f03a4bc1acc8a1032876d5687ab7f09))
- Update - ([877567b](https://github.com/yuma140902/reverie/commit/877567bac6c88ca7663bbeef1c3ba3b544609ef4))
- Update - ([3a2a320](https://github.com/yuma140902/reverie/commit/3a2a320d65adedac1504d62610b39dc9592eec1a))
- Update wgpu from v23 to v25 - ([20f8ef4](https://github.com/yuma140902/reverie/commit/20f8ef44c3065e5e4fa53fb194ced86bf9c60c56))
- Update dependencies - ([bc9fd36](https://github.com/yuma140902/reverie/commit/bc9fd369eaf00adbe109b1ab3d597dcbb3937f78))
- Update dependencies - ([0008134](https://github.com/yuma140902/reverie/commit/00081341d60e2c5777dc1c28d43ad42a8a782177))
- Update sccache action from v0.0.5 to v0.0.6 - ([dec2849](https://github.com/yuma140902/reverie/commit/dec2849833d86ef6c46db98a07137391472c1f99))
- Update reverie-engine-opengl dependencies - ([32d89e2](https://github.com/yuma140902/reverie/commit/32d89e24833c2de10635b1ba6526f324885f0cf8))
- Update reverie-util dependencies - ([45180c4](https://github.com/yuma140902/reverie/commit/45180c4983d62c756daeca3d02580ee7d63bd1b5))
- Update reverie-engine dependencies - ([8de62e5](https://github.com/yuma140902/reverie/commit/8de62e55f256e4ebc6e76897a0ae2f86edb41e0e))

### 🔧 Miscellaneous Tasks

- Format - ([8639a0f](https://github.com/yuma140902/reverie/commit/8639a0fbbba153b30f2eee5f53eac38360aa9949))

### 🔀 Pull Requests

- [#159](https://github.com/yuma140902/reverie/pull/159)
- [#158](https://github.com/yuma140902/reverie/pull/158)
- [#157](https://github.com/yuma140902/reverie/pull/157)
- [#156](https://github.com/yuma140902/reverie/pull/156)
- [#147](https://github.com/yuma140902/reverie/pull/147)
- [#146](https://github.com/yuma140902/reverie/pull/146)
- [#142](https://github.com/yuma140902/reverie/pull/142)
- [#141](https://github.com/yuma140902/reverie/pull/141)
- [#140](https://github.com/yuma140902/reverie/pull/140)
- [#139](https://github.com/yuma140902/reverie/pull/139)
- [#138](https://github.com/yuma140902/reverie/pull/138)
- [#137](https://github.com/yuma140902/reverie/pull/137)
- [#132](https://github.com/yuma140902/reverie/pull/132)
- [#136](https://github.com/yuma140902/reverie/pull/136)
- [#133](https://github.com/yuma140902/reverie/pull/133)

## [0.1.0](https://github.com/yuma140902/reverie/compare/v0.0.8..v0.1.0) - 2024-09-19

### ✨ Features

- *(example-misc)* Add example - ([990add5](https://github.com/yuma140902/reverie/commit/990add552eee179569d75c492faf9d332a61257e))
- Use wgpu - ([232842b](https://github.com/yuma140902/reverie/commit/232842b5f727a9ee1fb38403654f5eed8b004ae1))
- [**breaking**] Remove reverie-engine-opengl from reverie-engine - ([6b5e271](https://github.com/yuma140902/reverie/commit/6b5e271106f51693c6be4d16f778a11d04941850))

### ♻️ Refactor

- Use mul_add - ([f97aef5](https://github.com/yuma140902/reverie/commit/f97aef5ea6d0f841e7d720d4771e0c49bb4344d6))
- Rename old examples - ([28b9c46](https://github.com/yuma140902/reverie/commit/28b9c46d7a6edb8a6819a1d809dd515643e1f614))

### 📝 Documentation

- Generate changelog (v0.0.8) - ([40c9c68](https://github.com/yuma140902/reverie/commit/40c9c68f795befd959da27d11550e58a9bdbb37c))

### 👷 Build System

- Update to v0.1.0 - ([bfcf671](https://github.com/yuma140902/reverie/commit/bfcf67110d8bb1a6a7ea732629b8bfe32d21aac2))
- 依存関係を整理 - ([33c39d8](https://github.com/yuma140902/reverie/commit/33c39d8bbfcebcdfec6278d26141f059d24e28d0))
- MSRV を 1.81.0 に引き上げる - ([3afc661](https://github.com/yuma140902/reverie/commit/3afc6615917fe56594f3b8f926765431bfcd635d))
- Reverie-engine-opengl と reverie-util のバージョンをワークスペースと連動しないようにする - ([a6fcc20](https://github.com/yuma140902/reverie/commit/a6fcc206c4832ae9bb6bedac0154019b04a62180))

### 🔀 Pull Requests

- [#127](https://github.com/yuma140902/reverie/pull/127)
- [#126](https://github.com/yuma140902/reverie/pull/126)

## [0.0.8](https://github.com/yuma140902/reverie/compare/v0.0.7..v0.0.8) - 2024-09-17

### ✨ Features

- Ignore warnings by `cargo build` - ([322cbf1](https://github.com/yuma140902/reverie/commit/322cbf17b91e739614257c298d80ab64b20ec337))
- [**breaking**] Remove `Vao::new()`, `Vao::draw()`, and `Vao::draw_triangles()`. - ([a20ddd4](https://github.com/yuma140902/reverie/commit/a20ddd49d4e1275cffd8bb6114d097f10b539b21))
- [**breaking**] Make `Vao::new` unsafe because it may dereference unsafe pointer - ([0446db3](https://github.com/yuma140902/reverie/commit/0446db3dd33a9e07014d845012bd90f087275ef1))

### 🐛 Bug Fixes

- Once_cell version - ([4c74c13](https://github.com/yuma140902/reverie/commit/4c74c1306983cdedec0acb158a3b79691e7e225d))
- [**breaking**] Use simple type - ([4a751c6](https://github.com/yuma140902/reverie/commit/4a751c6783f5b1beebd831a791cf22bea043cd29))
- Lint errors - ([2abe9e1](https://github.com/yuma140902/reverie/commit/2abe9e17029e60548486f0aa6a24050630a85833))
- Crates.ioに公開できるようにreverie-engine-openglのバージョンを指定する - ([1936c91](https://github.com/yuma140902/reverie/commit/1936c916ae959333035de9a1f9c138f8f189aff0))

### ♻️ Refactor

- Fix lint warnings - ([dc478d0](https://github.com/yuma140902/reverie/commit/dc478d0f84ca12cea46c59563cf1faa0ee3d8a11))
- Fix unused variables - ([598892f](https://github.com/yuma140902/reverie/commit/598892fbcee1a632b3dc13daaa07ccb6d78d1e17))
- Remove unused lifetime - ([483549d](https://github.com/yuma140902/reverie/commit/483549dcd7d70b2a91e79f15e0d27ab112a6941b))
- Use slice - ([ec9620b](https://github.com/yuma140902/reverie/commit/ec9620b2808dc529f409113a42b3eb8464a69350))
- Use map_or - ([480671b](https://github.com/yuma140902/reverie/commit/480671b0b928f846d308de18eaab49980c28e851))
- Fix INFINITY - ([13063c1](https://github.com/yuma140902/reverie/commit/13063c113962afe461f6eea05d28f8dc197c4f04))
- Const fn - ([0dd7cc7](https://github.com/yuma140902/reverie/commit/0dd7cc7bcb10f0d6a4577ce698abb2500bef41fd))
- Allow too many args - ([c5ee529](https://github.com/yuma140902/reverie/commit/c5ee529c308f6ce4df4322f0f643fb19b5726515))
- Use unwrap_or_else - ([d286804](https://github.com/yuma140902/reverie/commit/d286804188a5dfc3097a4b1a3a21c89b246233f6))
- Const fn - ([b53b179](https://github.com/yuma140902/reverie/commit/b53b1793dfc71da5faadb2cde592ed071d024ed2))
- Justify unsafe pointer conversion - ([c43fe44](https://github.com/yuma140902/reverie/commit/c43fe44d698ffc8e94512928f33a9b2e9bbfed17))
- Use std::iter::once - ([68d2277](https://github.com/yuma140902/reverie/commit/68d2277d549081f76269790880366c21583835d6))
- Self - ([0bfbc64](https://github.com/yuma140902/reverie/commit/0bfbc64b5b12a9001bdfef2d17b147588aeb24d6))
- Suppress warnings for missing safety doc - ([f91948c](https://github.com/yuma140902/reverie/commit/f91948c281d656ac1298e86bf32b1d5bdbf8bc52))
- Disable clippy lint for generated code - ([66c1bac](https://github.com/yuma140902/reverie/commit/66c1bacfb46460a21ff8eba6879086ce04ecfdd8))
- Add deprecated attributes - ([3b2f90b](https://github.com/yuma140902/reverie/commit/3b2f90b6c276b2f7e601b46c3f60845d640edbfb))
- Add const - ([de9421f](https://github.com/yuma140902/reverie/commit/de9421ff71d0ed3d2edf1540cf9cd7cfcb074c92))
- Use From impls - ([2b7e4ac](https://github.com/yuma140902/reverie/commit/2b7e4ac6a41d9c54e5a3e108b10c7a32b184b2c4))
- Sort dependencies - ([203a0ef](https://github.com/yuma140902/reverie/commit/203a0ef6142edd396b3c83578ef76c38dab3b52b))
- Move examples - ([73c5f9e](https://github.com/yuma140902/reverie/commit/73c5f9eca6bcbdd15c1376637599cfc664a78134))

### 📝 Documentation

- Fix link - ([ade6810](https://github.com/yuma140902/reverie/commit/ade6810e4fb75fd55c747cf128bc860e00a5be86))
- Update - ([821a74b](https://github.com/yuma140902/reverie/commit/821a74b978a06a5e3d7ec43f4dd9f5c8256f2c3b))

### 👷 Build System

- Update to v0.0.8 - ([1de3ed7](https://github.com/yuma140902/reverie/commit/1de3ed7230041468e8819b10ac20c01525039443))
- Add `exclude` to matrix - ([57b50b1](https://github.com/yuma140902/reverie/commit/57b50b174134f13d3a6343a38ecd222371a5c8be))
- Fix arguments - ([f81cec0](https://github.com/yuma140902/reverie/commit/f81cec0a9281f123011a035b35ae4e2fe588de49))
- Install packages - ([6ec80c9](https://github.com/yuma140902/reverie/commit/6ec80c93c4da3c3c941450606ecb646032d687cb))
- Install gl package on Ubuntu - ([6c35d3b](https://github.com/yuma140902/reverie/commit/6c35d3b983df4f3e92d7c172760f9f7f48968d3a))
- Fix - ([012007e](https://github.com/yuma140902/reverie/commit/012007e761836997e0b01d8b3abcbb0c77537733))
- Update ci - ([21def5e](https://github.com/yuma140902/reverie/commit/21def5ea2dc4a5ab5bbdf9a6e297f2e6dd663ecc))
- Update dependencies - ([6c2b5ea](https://github.com/yuma140902/reverie/commit/6c2b5eaf9a06628419da02c84c11bd8ac81eb766))

### 🔧 Miscellaneous Tasks

- Add `publish = false` to examples - ([0a484e6](https://github.com/yuma140902/reverie/commit/0a484e6d7c01f41003ca26c5ea1739d48a48ce17))

### 🔀 Pull Requests

- [#125](https://github.com/yuma140902/reverie/pull/125)
- [#124](https://github.com/yuma140902/reverie/pull/124)
- [#123](https://github.com/yuma140902/reverie/pull/123)
- [#122](https://github.com/yuma140902/reverie/pull/122)
- [#121](https://github.com/yuma140902/reverie/pull/121)
- [#120](https://github.com/yuma140902/reverie/pull/120)
- [#119](https://github.com/yuma140902/reverie/pull/119)
- [#118](https://github.com/yuma140902/reverie/pull/118)

## [0.0.7](https://github.com/yuma140902/reverie/compare/v0.0.6..v0.0.7) - 2024-06-04

### ✨ Features

- *(util)* Add rotation util - ([8df6b8f](https://github.com/yuma140902/reverie/commit/8df6b8f68fd2c037b80fc161e45852e91b2991df))
- [**breaking**] Reverie-engine を reverie-engine-opengl にリネーム - ([5bb03d6](https://github.com/yuma140902/reverie/commit/5bb03d6f7971bdedbc375356db0dd39329b3b863))
- Examplesとreverie-utilのバージョンをワークスペースのバージョンに合わせる - ([e659060](https://github.com/yuma140902/reverie/commit/e659060e4673547d0bceead7b9b327bac1ea991a))
- ExamplesもMIT OR Apache-2.0ライセンスにする - ([2b0c135](https://github.com/yuma140902/reverie/commit/2b0c135c29c1082fc8eab2cd209087c57a0333b5))
- ライセンスをMPL-2.0からMIT OR Apache-2.0に変更 - ([8a60be2](https://github.com/yuma140902/reverie/commit/8a60be2a2a3970924e265cb839e9dad924467653))
- Add Camera - ([917a27a](https://github.com/yuma140902/reverie/commit/917a27af0faa380112ae8ffdc3acaf49507e1dc4))
- Make raw id getters unsafe - ([d3a52f4](https://github.com/yuma140902/reverie/commit/d3a52f489f5bc61c2ea95f36a1fe476e497e6d02))
- Apply texture alpha - ([0ac8bf8](https://github.com/yuma140902/reverie/commit/0ac8bf88b992e79ebd1845a2cb4449aa7f70f068))

### ♻️ Refactor

- Clippy fix - ([95307f7](https://github.com/yuma140902/reverie/commit/95307f79823d8814b7fb5ddc49e5ad1f2ade2375))

### 📝 Documentation

- Update changelog (v0.0.7) - ([ebbe939](https://github.com/yuma140902/reverie/commit/ebbe939e4af6c606073cac23dd299577e32eb5ff))
- Fix badges - ([d2401c7](https://github.com/yuma140902/reverie/commit/d2401c7f1252fd84eb1c7f35dbc104193b0390be))
- Generate CHANGELOG-util.md - ([5513678](https://github.com/yuma140902/reverie/commit/55136785c1093d519ab8058fc16849d6963ab820))

### 👷 Build System

- Update to v0.0.7 - ([810a596](https://github.com/yuma140902/reverie/commit/810a59615652bc7ec2aafe8d11e38119c766bb7e))
- Remove rustfmt.toml; use default settings - ([0950e4c](https://github.com/yuma140902/reverie/commit/0950e4c7b62296abcd009159d4d6af18d5afef8f))
- 最新版に更新できないクレートをワークスペースの依存関係から削除、各クレートに移動 - ([10fc3b2](https://github.com/yuma140902/reverie/commit/10fc3b270569182709a26411429a35e25d906a96))
- 依存クレートのアップグレード - ([cd18d4c](https://github.com/yuma140902/reverie/commit/cd18d4c44fa21398d652bad001d6bf9e4b7d5702))
- Move build.rs - ([49d01be](https://github.com/yuma140902/reverie/commit/49d01be332eb32591e1a52b5517af609339b75ca))
- MSRVを追加(1.78.0) - ([f84bb70](https://github.com/yuma140902/reverie/commit/f84bb704100cd855ab70f010e07f370e237dcc04))
- 依存関係をワークスペースのCargo.tomlに集約 - ([1a1c470](https://github.com/yuma140902/reverie/commit/1a1c47003224beb4f35ce319ae8b37642fdf294f))
- Update github actions runner - ([2e46522](https://github.com/yuma140902/reverie/commit/2e4652209425066638ed05eed23e459649a0017a))

### 🔧 Miscellaneous Tasks

- Remove VSCode settings - ([ab67b5d](https://github.com/yuma140902/reverie/commit/ab67b5d5e6d925d9d51e58f7f494d8cdea83a0da))
- Fix license file name - ([02d26c6](https://github.com/yuma140902/reverie/commit/02d26c6b48ad2364e69eee9ca64bb66dd4f84555))
- Remove git-chglog, use git-cliff - ([6830358](https://github.com/yuma140902/reverie/commit/6830358c61d06523fbf60203a1c8a984127666e2))
- Use rust-cache action - ([ffdb9ed](https://github.com/yuma140902/reverie/commit/ffdb9edd8eed2832f533bbfd7c7059888afb4324))
- Minimize image crate - ([47af9a7](https://github.com/yuma140902/reverie/commit/47af9a7dda46220f3043dc34b4b77bd96e3349a0))
- Changelog generation for monorepo - ([465f671](https://github.com/yuma140902/reverie/commit/465f671b9c609a4c3f2863fdd7abb3a45b87162d))
- Fix readme location - ([2106fda](https://github.com/yuma140902/reverie/commit/2106fdaddd3d200979518da1adc9bd3ccba60a7f))
- Fix dependencies - ([4921c4c](https://github.com/yuma140902/reverie/commit/4921c4c4bba12650da975384b85cc2ce759307cc))

### 🔀 Pull Requests

- [#117](https://github.com/yuma140902/reverie/pull/117)
- [#115](https://github.com/yuma140902/reverie/pull/115)
- [#114](https://github.com/yuma140902/reverie/pull/114)
- [#112](https://github.com/yuma140902/reverie/pull/112)
- [#110](https://github.com/yuma140902/reverie/pull/110)
- [#109](https://github.com/yuma140902/reverie/pull/109)
- [#108](https://github.com/yuma140902/reverie/pull/108)
- [#107](https://github.com/yuma140902/reverie/pull/107)
- [#106](https://github.com/yuma140902/reverie/pull/106)
- [#105](https://github.com/yuma140902/reverie/pull/105)
- [#97](https://github.com/yuma140902/reverie/pull/97)
- [#96](https://github.com/yuma140902/reverie/pull/96)
- [#92](https://github.com/yuma140902/reverie/pull/92)
- [#91](https://github.com/yuma140902/reverie/pull/91)
- [#90](https://github.com/yuma140902/reverie/pull/90)
- [#89](https://github.com/yuma140902/reverie/pull/89)
- [#88](https://github.com/yuma140902/reverie/pull/88)
- [#87](https://github.com/yuma140902/reverie/pull/87)
- [#86](https://github.com/yuma140902/reverie/pull/86)
- [#85](https://github.com/yuma140902/reverie/pull/85)
- [#84](https://github.com/yuma140902/reverie/pull/84)

### Example

- *(craft)* Use camera - ([69cf6c7](https://github.com/yuma140902/reverie/commit/69cf6c774890eec180b0d60d0f72b69d348603ea))
- *(craft)* Use ImageManager#load_from_memory - ([9c1d73a](https://github.com/yuma140902/reverie/commit/9c1d73a895a889b378f9220ebb6bff91d4a6cbd2))
- Use default shader - ([951a3f1](https://github.com/yuma140902/reverie/commit/951a3f17db6ebb67526ce582934b4d5f3f1824b7))

## [0.0.6](https://github.com/yuma140902/reverie/compare/util-v0.0.0..v0.0.6) - 2022-10-01

### 🐛 Bug Fixes

- Fix readme path - ([d9641c4](https://github.com/yuma140902/reverie/commit/d9641c4edef45a50e68965bc10142c4ce247a3ad))

### 📝 Documentation

- Update readme - ([a517ba0](https://github.com/yuma140902/reverie/commit/a517ba05a930cc29560888bb9017f6697c9a9a17))
- Changelog - ([2c69198](https://github.com/yuma140902/reverie/commit/2c69198cef1ed6f542df810e3e0fe29ad0f7dc57))

### 🔧 Miscellaneous Tasks

- Generate changelog - ([7b0089f](https://github.com/yuma140902/reverie/commit/7b0089f3d336f3b197475dcc3cc73f4f8b2083e3))
- Update to v0.0.6 - ([d59ae00](https://github.com/yuma140902/reverie/commit/d59ae00d20177a766fcb5a724eebb6bd16d2518c))
- Add "reverie-util" to workspace - ([13f28a5](https://github.com/yuma140902/reverie/commit/13f28a5c3eca5d30018aefcea954b311196ccf62))
- Upgrade rust edition to 2021 - ([7871471](https://github.com/yuma140902/reverie/commit/7871471e3e59f1adb72adb0845542f712fcfdf23))
- Changelog - ([7ad659a](https://github.com/yuma140902/reverie/commit/7ad659aeb2d5ef16ac2693ec9c8ea0543cffd44f))

### 🔀 Pull Requests

- [#82](https://github.com/yuma140902/reverie/pull/82)

## [util-v0.0.0](https://github.com/yuma140902/reverie/compare/v0.0.5..util-v0.0.0) - 2022-09-30

### 🐛 Bug Fixes

- Fix process_event arguments in examples - ([19e2fd1](https://github.com/yuma140902/reverie/commit/19e2fd1fae7a3d9d4a48f1d7f608bd6eb4887b9b))
- Fix process_event arguments in examples - ([251eaa9](https://github.com/yuma140902/reverie/commit/251eaa940a23df5ae2dc9c079c11f5419fbbf453))
- Fix num_attributes - ([a114738](https://github.com/yuma140902/reverie/commit/a114738832512777455a913186d3d979aa7c28aa))
- Fix yaw and pitch - ([8a3b0d6](https://github.com/yuma140902/reverie/commit/8a3b0d69641dc8d92e4d5e603f160f0dc77c43e8))
- Fix feature cfg - ([7d19c53](https://github.com/yuma140902/reverie/commit/7d19c5339230fd5602077f8e83f9a080a2cc2ec1))
- Fix clippy: needless borrow - ([49a4929](https://github.com/yuma140902/reverie/commit/49a4929075423a35748e84574caa948999b998f7))
- Fix clippy: needless borrow, return - ([87aec90](https://github.com/yuma140902/reverie/commit/87aec908fa9d71c328ec4684b392cee3403c6856))

### 🔀 Pull Requests

- [#79](https://github.com/yuma140902/reverie/pull/79)
- [#76](https://github.com/yuma140902/reverie/pull/76)
- [#72](https://github.com/yuma140902/reverie/pull/72)
- [#71](https://github.com/yuma140902/reverie/pull/71)
- [#70](https://github.com/yuma140902/reverie/pull/70)
- [#65](https://github.com/yuma140902/reverie/pull/65)
- [#61](https://github.com/yuma140902/reverie/pull/61)
- [#60](https://github.com/yuma140902/reverie/pull/60)
- [#59](https://github.com/yuma140902/reverie/pull/59)
- [#58](https://github.com/yuma140902/reverie/pull/58)
- [#56](https://github.com/yuma140902/reverie/pull/56)
- [#57](https://github.com/yuma140902/reverie/pull/57)
- [#49](https://github.com/yuma140902/reverie/pull/49)
- [#47](https://github.com/yuma140902/reverie/pull/47)
- [#46](https://github.com/yuma140902/reverie/pull/46)
- [#44](https://github.com/yuma140902/reverie/pull/44)
- [#43](https://github.com/yuma140902/reverie/pull/43)
- [#40](https://github.com/yuma140902/reverie/pull/40)
- [#38](https://github.com/yuma140902/reverie/pull/38)
- [#37](https://github.com/yuma140902/reverie/pull/37)
- [#36](https://github.com/yuma140902/reverie/pull/36)
- [#34](https://github.com/yuma140902/reverie/pull/34)
- [#33](https://github.com/yuma140902/reverie/pull/33)
- [#32](https://github.com/yuma140902/reverie/pull/32)
- [#28](https://github.com/yuma140902/reverie/pull/28)

## [0.0.5](https://github.com/yuma140902/reverie/compare/v0.0.4..v0.0.5) - 2022-08-26

### 🐛 Bug Fixes

- Fix Cargo.toml and #[cfg] - ([314d9f3](https://github.com/yuma140902/reverie/commit/314d9f3a11b738e7f6df58a42d97c415d5d17a41))

### 🔧 Miscellaneous Tasks

- *(VSCode)* Add CodeLLDB - ([b16bb7d](https://github.com/yuma140902/reverie/commit/b16bb7d99a5a37f86f515dcfbb99d9efe9fa71dd))
- *(VSCode)* Add CodeLLDB - ([403b9c4](https://github.com/yuma140902/reverie/commit/403b9c4d35419566b4608e7f26e1b69fa329dd4a))
- *(VSCode)* Add Rust Analyzer - ([0dcbdbe](https://github.com/yuma140902/reverie/commit/0dcbdbef01319b1c6e5091ee81299d4513883468))

### ⏪️ Revert

- "TextureUV::of_atlas() → ::new()" - ([c7f7953](https://github.com/yuma140902/reverie/commit/c7f7953d0a778ac3471b6fae834700914582851b))

### 🔀 Pull Requests

- [#24](https://github.com/yuma140902/reverie/pull/24)
- [#21](https://github.com/yuma140902/reverie/pull/21)
- [#19](https://github.com/yuma140902/reverie/pull/19)
- [#18](https://github.com/yuma140902/reverie/pull/18)
- [#15](https://github.com/yuma140902/reverie/pull/15)

### TextureUV

- :of_atlas() → ::new() - ([a76aa0a](https://github.com/yuma140902/reverie/commit/a76aa0a8d144146736a64759f08352f001fa46c8))

### Example

- Reduce CPU usage - ([22dbb8c](https://github.com/yuma140902/reverie/commit/22dbb8cf1f5ef1cc7d868108beb2711a83b0c0ef))

## [0.0.4](https://github.com/yuma140902/reverie/compare/v0.0.2..v0.0.4) - 2021-11-06

### 🔧 Miscellaneous Tasks

- VSCode - ([9f0d394](https://github.com/yuma140902/reverie/commit/9f0d39491ab9d811db285ffcf74df48b27687095))

### ADD

- Shader::from_code() - ([7d422c1](https://github.com/yuma140902/reverie/commit/7d422c145093fb0d1afb32c7e23e9eacb9d608fd))

### Add

- Interpolation - ([cbb7dc5](https://github.com/yuma140902/reverie/commit/cbb7dc5975ed5ba357f9180dd6b4ab54ae2bb486))

### Update

- Cargo.toml - ([28e31c5](https://github.com/yuma140902/reverie/commit/28e31c533b473a5fca0563e8065868315939e091))
- README.md - ([edaa1cf](https://github.com/yuma140902/reverie/commit/edaa1cfc5cd8447ba152b5b10dfc129f74c84918))

### VaoBuffer

- :with_capacity()の仕様変更 - ([c549f81](https://github.com/yuma140902/reverie/commit/c549f814875b0c56767cae73da0cf8aa140152a2))
- :build()が消費しないようになった - ([be0bb8c](https://github.com/yuma140902/reverie/commit/be0bb8c1a3786f9bd009b870eb6d54e5eb616d15))
- :append()を追加 - ([28c1fe3](https://github.com/yuma140902/reverie/commit/28c1fe3d0adba4425f31b501432e064997412529))

## [0.0.2](https://github.com/yuma140902/reverie/compare/v0.0.1..v0.0.2) - 2021-10-08

## [0.0.1](https://github.com/yuma140902/reverie/compare/v0.0.0..v0.0.1) - 2021-10-04

### Add

- Uniform, UniformVariables - ([0d086a0](https://github.com/yuma140902/reverie/commit/0d086a075fc3a47df0283c16d039a5cb04f0587d))

## [0.0.0] - 2021-10-04

<!-- generated by git-cliff -->
