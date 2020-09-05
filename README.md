Learning Rust
=============
[![pixela][pixela-graph]][pixela]

[pixela-graph]: https://pixe.la/v1/users/m3y/graphs/learning-rust?mode=badge
[pixela]: https://pixe.la/v1/users/m3y/graphs/learning-rust.html

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

## repl
```
cargo install evcxr_repl
```

## watch
```
cargo install cargo-watch
```
```
cargo watch -x check -x test
```

## audit
```
cargo install cargo-audit
```
```
cargo audit
```

## wasm
npm
```
asdf install nodejs 12.18.3
asdf global nodejs 12.18.3
```
cargo-generate
```
cargo install cargo-generate
```
wasm-pack
```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## シングルバイナリ
- linux
```
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release
```
