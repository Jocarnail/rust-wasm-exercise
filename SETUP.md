# Setup and notes

Installation and setup steps followed.
Not worth making an `.sh` file since they are mostly run once.

## wasm-pack installation

Installing wasm-pack from cargo.

```bash
cargo install wasm-pack

```

## cargo-generate

```bash
cargo install cargo-generate
```

## update npm
```bash
npm install npm@latest -g
```

## clone from template
```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

Renamed Template's README file to [README_TEMPLATE.md](README_TEMPLATE.md)
Moved all files of template to main directory.
Deleated original template directory.
