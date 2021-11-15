# rust-rpn

逆ポーランド記法の計算ができるコマンドラインツールです

## install

プロジェクトルートで下記を実行します。

```console
$ cargo install --path .
```

`cargo`コマンドがない場合は、[rustup を事前にインストール](https://www.rust-lang.org/ja/tools/install)してください。

下記のように表示されれば、インストール成功です。

```console
...
Installed package `rpncalc v0.1.0 (/.../rust-rpn)` (executable `rpncalc`)
```

## コマンドの実行

### 標準入力から読み込み

```console
$ echo '1 1 +' | rpncalc
2

$ echo '5 10 *' | rpncalc
50
```

### テキストファイルから読み込み

```console
$ cat input.txt
1 1 +
5 10 *

$ rpncalc input.txt
2
50
```
