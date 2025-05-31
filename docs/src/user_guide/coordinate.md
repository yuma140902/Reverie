# 座標系

このドキュメントでは、Reverie Engine で使用される座標系について説明します。
Reverie Engine はグラフィックスAPIとして wgpu を利用しています。

## 1. 基本事項

- **座標系の種類**: 左手座標系 (Left-Handed Coordinate System)
- **深度範囲**: 0.0 (Near Plane) から 1.0 (Far Plane) - クリップ座標系において

## 2. 各座標系の詳細

### 2.1. モデル座標系 (Model Space)

- 各モデル固有のローカルな座標系です。
- モデルの頂点データは、このモデル座標系で定義されます。
- 通常、モデルの中心や設計上の基点が原点 (0,0,0) となります。
- `TransformComponent` (`translation`, `rotation`, `scale`) は、このモデル座標系からワールド座標系への変換を定義します。

### 2.2. ワールド座標系 (World Space)

- シーン全体のグローバルな基準となる座標系です。
- エンティティの位置 (`translation`)、回転 (`rotation`)、スケール (`scale`) は、このワールド座標系で定義されます (`TransformComponent`)。
- デフォルトでは、Y軸が上方向、X軸が右方向、Z軸が奥方向となります（左手系）。

### 2.3. ビュー座標系 (View Space / Camera Space)

- カメラから見た世界の座標系です。
- ワールド座標系からビュー行列によって変換されます。
- カメラは自身のローカル座標系の **+Z軸方向** を向きます。

### 2.4. クリップ座標系 (Clip Space)

- ビュー座標系からプロジェクション行列（正射影または透視投影）によって変換された座標系です。
- この空間では、表示される可能性のある領域が正規化された直方体にマッピングされます。
- 座標範囲:
	- X: [-1, 1] (左端から右端)
	- Y: [-1, 1] (下端から上端)
	- Z: [0, 1] (近クリップ面から遠クリップ面)

### 2.5. wgpu テクスチャ座標系 (wgpu Texture Coordinate System - UV)

- テクスチャマッピングに使用される2次元座標系です。
- **原点 (U=0, V=0)**: テクスチャの左上
- **U軸**: 右方向が正 (+u)
- **V軸**: 下方向が正 (+v)
- 座標範囲は通常 [0, 1] です。

## 3. 変換の概要

1. モデル変換 (Model Transform / World Transform) - `TransformComponent` による
	- モデル座標系 → ワールド座標系
2. ビュー変換 (View Transform) - `Camera::get_matrix_world_to_render_coordinate` 内の `view` 行列
	- ワールド座標系 → ビュー座標系
3. プロジェクション変換 (Projection Transform) - `Camera::get_matrix_world_to_render_coordinate` 内の `proj` 行列
	- ビュー座標系 → クリップ座標系
