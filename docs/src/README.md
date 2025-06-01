# Reverie Engine

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/yuma140902/reverie/rust.yml?logo=github&label=CI)](https://github.com/yuma140902/reverie/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/reverie-engine)](https://crates.io/crates/reverie-engine)
[![docs.rs](https://img.shields.io/docsrs/reverie-engine?logo=docsdotrs)](https://docs.rs/reverie-engine/latest/reverie-engine/)

Reverie Engine は、Rust で書かれたゲームエンジンです。
主に学習目的や小規模なゲームの開発をターゲットとしています。
wgpu ライブラリをレンダリングバックエンドとして使用し、Entity Component System (ECS) アーキテクチャ (hecs を利用) を採用しています。

**主要な機能や特徴:**

- 2D スプライトベースのレンダリング
- テクスチャ管理機能 (個別のテクスチャ読み込み、テクスチャアトラスのサポート)
- エンティティコンポーネントシステム (ECS)
    *   TransformComponent (位置、回転、スケール)
    *   SpriteComponent (スプライト表示)
- カスタムロジックを実装するためのシステム登録機能
- wgpu を利用したクロスプラットフォームなグラフィックス描画
- ウィンドウ管理とイベント処理 (winit を利用)

## デモ

```sh
cargo run -p example-misc
```

## リンク

- [GitHub](https://github.com/yuma140902/Reverie)
- [crates.io](https://crates.io/crates/reverie-engine)
- [Documentation](https://yuma14.net/Reverie/)
- [API Documentation](https://docs.rs/reverie-engine/)

## ライセンス

このプロジェクトは、以下のいずれかのライセンスの下で利用可能です。

- MIT License
- Apache License, Version 2.0

利用者は、自身の裁量でいずれかのライセンスを選択できます。
