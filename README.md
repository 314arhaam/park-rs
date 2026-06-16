# Park

A fast CLI tool for inspecting **Parquet files**, written in Rust.

---

## Features

- View Parquet file metadata
- Preview rows (head)
- Lightweight and fast (no Python required)
- Cross-platform (Linux, macOS, Windows)

---

## Installation

### Recommended (Install Script)

```sh
curl -fsSL https://raw.githubusercontent.com/314arhaam/park-rs/main/install.sh | bash
```

### Manual Install (GitHub Releases)

Download the latest binary from:
https://github.com/314arhaam/park-rs/releases

---

## Usage

```sh
park <COMMAND>
```

### Help

```sh
park --help
```

---

## Commands

### info

Display metadata and schema information for a Parquet file.

```sh
park info <file.parquet>
```

---

### head

Display the first rows of a Parquet file.

```sh
park head <file.parquet> --num 10
```

---

## Example

```sh
park info users.parquet
park head users.parquet --num 5
```

---

## Why Park?

A minimal CLI alternative to heavier tooling for quickly inspecting Parquet files.

---

## Development

```sh
cargo build --release
cargo run -- head data.parquet
```

---

## License

MIT
