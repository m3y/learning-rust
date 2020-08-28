Learning Rust
=============

setup
-----

### install
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```
cargo install cargo-edit
```

### LSP install
```
cargo new hello
```
```
vi hello/src/main.rs
# :LspInstallServer を実行することで rust-analyzer がインストールされる
```

### 保存時の自動フォーマット
rust.vim pluginを利用する
```dein.toml
[[plugins]]
repo = 'rust-lang/rust.vim'
hook_add = 'let g:rustfmt_autosave = 1'
```

### フォーマッター
```
cargo fmt
```

### リンター
```
cargo clippy
```

### テスト
`#[test]`を付与した関数で書き、以下のマクロで確認する
- assert_eq!
- assert_nq!
- などなど

実行は、
```
cargo test
```

### ドキュメント
```
cargo doc --no-deps --open
```
- `//` が通常コメント
- `///` がドキュメント用コメント
