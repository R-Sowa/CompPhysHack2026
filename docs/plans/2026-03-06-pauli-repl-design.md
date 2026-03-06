# Pauli Symbolic REPL Design

**Date:** 2026-03-06

## Goal

3日間のハッカソン向けに、Pauli 代数に限定した量子記号簡約エンジンを Rust で実装し、CLI/REPL から式変形を体験できるようにする。発表では「非可換性がその場で見えること」と「どの規則で式が変わったかを追えること」を主な価値として見せる。

## Product Shape

- 形式: CLI ベースの REPL
- 対象: `I`, `X`, `Y`, `Z`, `i`, 整数スカラー, 加算, 乗算, 括弧, 交換子
- 主用途: デモ、学習、量子力学の基礎概念の説明
- 非目標:
  - 一般の量子回路簡約
  - 任意演算子の非可換代数
  - GUI / Web UI
  - 浮動小数の数値計算

## MVP Scope

MVP では次の入力を安定して扱えるようにする。

- `X*Y`
- `[X,Y]`
- `(X+Y)*(X+Y)`

成功条件は次の通り。

- 代表的な Pauli 式を正しく簡約できる
- REPL で `parsed`, `steps`, `result` を表示できる
- 発表時に 2〜3 個の入力例を安定してデモできる

## Architecture

システムは 4 層で構成する。

1. `parser`
   - 入力文字列を AST に変換する
   - 最小文法のみを実装する
2. `core ast`
   - 式を型安全に表現する
   - `Add` と `Mul` は平坦な `Vec` として持つ
3. `simplifier`
   - 正規化、局所書換え、固定点までの反復を担当する
   - 各書換えステップにルール名を付ける
4. `repl`
   - 1行入力を受け取り、パース結果、変形手順、最終結果を表示する

## Data Model

Rust の内部表現は最小限に絞る。

```rust
enum Pauli {
    I,
    X,
    Y,
    Z,
}

struct Scalar {
    re: i32,
    im: i32,
}

enum Expr {
    Scalar(Scalar),
    Sym(Pauli),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Commutator(Box<Expr>, Box<Expr>),
}
```

`Scalar` は `a + bi` のガウス整数だけを扱う。これで `i^2 = -1` や `[X,Y] = 2iZ` を誤差なく処理できる。

## Simplification Rules

最低限必要な規則は次の通り。

- 恒等元:
  - `I*A -> A`
  - `A*I -> A`
- 自乗:
  - `X*X -> I`
  - `Y*Y -> I`
  - `Z*Z -> I`
- 積:
  - `X*Y -> iZ`
  - `Y*Z -> iX`
  - `Z*X -> iY`
- 逆順の符号反転:
  - `Y*X -> -iZ`
  - `Z*Y -> -iX`
  - `X*Z -> -iY`
- 分配:
  - `A*(B+C) -> AB + AC`
  - `(A+B)*C -> AC + BC`
- 交換子:
  - `[A,B] -> AB - BA`
- スカラー整理:
  - `i*i -> -1`
  - 整数係数の合成
- 正規化:
  - `Add` / `Mul` の平坦化
  - 0 項や 1 項の除去
  - 同類項の結合

実装は汎用書換えエンジンではなく、`normalize -> local rewrite -> repeat until fixed point` の単純ループを採用する。3日間ではこの方が安全で、追跡表示も作りやすい。

## REPL UX

入力 1 行に対して、次の 3 要素を返す。

- `parsed`
- `steps`
- `result`

想定出力:

```text
> (X+Y)*(X+Y)

parsed: (X + Y)(X + Y)
step 1: distribute -> XX + XY + YX + YY
step 2: square/product -> I + iZ - iZ + I
step 3: collect scalars -> 2I
result: 2I
```

発表向けデモ入力は以下で固定する。

- `X*Y`
- `[X,Y]`
- `(X+Y)*(X+Y)`

## Error Handling

エラーは 3 種類に限定する。

- `parse error`
  - 括弧不整合、無効トークン
- `unsupported expression`
  - 未対応シンボルや未対応構文
- `simplification stuck`
  - 現状ルールではこれ以上処理できない入力

エラーメッセージは自己修正しやすい文面にする。

```text
unknown symbol: H (supported: I, X, Y, Z, i)
```

## Verification Strategy

テストは少数の強いケースに絞る。

- 単体テスト:
  - `X*X -> I`
  - `X*Y -> iZ`
  - `Y*X -> -iZ`
- 複合テスト:
  - `(X+Y)*(X+Y) -> 2I`
- 交換子テスト:
  - `[X,Y] -> 2iZ`
- パーサテスト:
  - 括弧、加算、乗算、交換子
- REPL スモークテスト:
  - 代表入力が落ちずに整形出力を返す

必要ならテスト専用の 2x2 行列表現を裏で使い、簡約結果の等価性を確認する。

## Hackathon Narrative

発表では次の順で見せる。

1. `X*Y` で Pauli 積のルールを示す
2. `[X,Y]` で非可換性と量子力学らしさを示す
3. `(X+Y)*(X+Y)` で分配と簡約の追跡表示を見せる

この順番にすることで、「ルールがある」「順番で結果が変わる」「複合式でも追える」の 3 点を短時間で伝えられる。
