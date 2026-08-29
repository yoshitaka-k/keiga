<p align="center">
  <img src="assets/icon.png" alt="Keiga" width="96">
</p>

<h1 align="center">Keiga</h1>

<p align="center">
  Lightweight image optimizer
</p>

<p align="center">
  <img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/yoshitaka-k/keiga">
  <a href="https://github.com/yoshitaka-k/keiga/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/github/license/yoshitaka-k/keiga"></a>
  <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/yoshitaka-k/keiga">
  <img alt="Lines of code" src="https://www.aschey.tech/tokei/github/yoshitaka-k/keiga">
  <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/yoshitaka-k/keiga">
</p>

Rust の勉強がてら、自分用に Image Optimization ってことで、Keiga（軽画）でも作ってみようと思って作成なぅ。

フォルダや画像をドロップすると、対応形式をその場で最適化します。出力先が空なら元ファイルを上書きし、指定していれば相対パスを保ったままそちらへ書き出します。

<p align="center">
  <img src="assets/readme/keiga-preview.png" alt="Keiga screenshot" width="640">
</p>

## Supported formats

最適化できるのは次の拡張子のみです。

| Extension | Optimization |
| --- | --- |
| `.jpg` / `.jpeg` | 非可逆（JPEG Quality で再エンコード） |
| `.png` | 可逆（[oxipng](https://github.com/oxipng/oxipng)） |

ダイアログには他の画像拡張子も表示されますが、最適化対象外は `Unsupported extension` になります。

最適化後のサイズが元より小さくならない場合は上書きせず、`Unchanged`（No savings）になります。出力先が別パスなら、元ファイルをそのままコピーします。

## Usage

- フォルダまたはファイルを **ドラッグ＆ドロップ**
- 右上のフォルダボタンから開く（macOS はファイルとフォルダを同時選択可。それ以外はフォルダのみ）

追加されたファイルは待機（standby）から順に自動で最適化されます。フォルダ経由で追加した行はフォルダアイコン、単体ファイルは画像アイコンです。

同じパスがすでに待機中・最適化中なら `Already in progress` になります。完了済みの同じ入出力は、設定の Skip same path に従ってスキップできます。

## Status

一覧と下部バーで使う状態です。

| Status | Meaning |
| --- | --- |
| Standby | 待機中 |
| Optimizing | 最適化中 |
| Optimized | 最適化完了（サイズと節約率、所要時間を表示） |
| Unchanged | 縮小できなかった |
| Skipped | 同じ入出力が完了済みのためスキップ |
| Canceled | キャンセル済み |
| Error | 失敗（メッセージを表示） |

下部バーは standby / optimizing / completed / error の件数と平均節約率です。completed にホバーすると optimized / no savings / skipped の内訳、左のアイコンにホバーすると合計所要時間が出ます。

## Mouse & keyboard

一覧の行を対象にした操作です。ダブルクリック・Enter・Space は、出力先があればそちらを開きます。

| Input | Behavior |
| --- | --- |
| Click | 行を選択 |
| Click on empty area | 選択を解除 |
| Double-click / <kbd>Enter</kbd> | Finder / Explorer でファイルの場所を表示 |
| <kbd>↑</kbd> / <kbd>↓</kbd> | 選択行を移動 |
| <kbd>Space</kbd> | [Quick Look](https://support.apple.com/guide/mac-help/mchlp1119/mac) でプレビュー（**macOS のみ**） |
| <kbd>Backspace</kbd> | 選択中の最適化をキャンセル（Optimized / Unchanged / Skipped / Error は対象外） |

アプリ全体のショートカットです。macOS は <kbd>⌘</kbd>、その他は <kbd>Ctrl</kbd> です。

| Input | Behavior |
| --- | --- |
| <kbd>⌘O</kbd> / <kbd>Ctrl+O</kbd> | 開く（macOS はファイルとフォルダ。その他はファイル） |
| <kbd>Ctrl+Shift+O</kbd> | フォルダを開く（**macOS 以外**） |
| <kbd>⌘,</kbd> / <kbd>Ctrl+,</kbd> | 設定を開く |
| <kbd>⌘W</kbd> / <kbd>Ctrl+W</kbd> | 設定ウィンドウを閉じる |

右下のクリアボタンは、実行中の最適化を止めて一覧を空にします。

## Settings

歯車アイコン、または <kbd>⌘,</kbd> / <kbd>Ctrl+,</kbd> から設定ウィンドウを開きます。閉じるのは <kbd>⌘W</kbd> / <kbd>Ctrl+W</kbd> です。値は次回起動時に復元されます。

### General

| Setting | Behavior |
| --- | --- |
| Skip same path | 完了済みの同じ入出力はスキップ。キャンセルとエラーは再実行できる |
| Output path | 書き出し先。空なら元ファイルを上書き |

### Concurrent

| Setting | Range | Notes |
| --- | --- | --- |
| Concurrent All files | 3–8 | 全体の同時実行数（既定 4） |
| Concurrent PNG files | 1–3 | PNG の同時実行数。All に含まれる（既定 2） |

### Quality

| Setting | Range | Notes |
| --- | --- | --- |
| JPEG Quality | 50–99 | 非可逆（既定 80） |
| PNG Preset | Min / Fast / Default / Best / Max | 可逆。oxipng のプリセット |

### About

バージョン・ライセンス・リポジトリと、GitHub Releases へのアップデート確認があります。

## License

[Apache-2.0](https://github.com/yoshitaka-k/keiga/blob/main/LICENSE)
