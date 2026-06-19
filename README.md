# Park

[![Build & Test](https://github.com/314arhaam/park-rs/actions/workflows/build.yml/badge.svg)](https://github.com/314arhaam/park-rs/actions/workflows/build.yml)
[![Release](https://github.com/314arhaam/park-rs/actions/workflows/release.yml/badge.svg)](https://github.com/314arhaam/park-rs/actions/workflows/release.yml)

A fast CLI tool for inspecting **Parquet files**, written in Rust.

```sh
park tail test_10k.parquet 
┌───────────────────────┬────────────────────────────────────────┬──────────────┬────────┐
│ date_                 ┆ user_id                                ┆ user_name    ┆ score  │
╞═══════════════════════╪════════════════════════════════════════╪══════════════╪════════╡
│ "2025-01-16 05:18:58" ┆ "23d704eb-3558-4106-817f-b63c8049f402" ┆ "SWMIPDGZEU" ┆ 7.06   │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ "2025-02-22 17:44:03" ┆ "19fde562-0ad6-4174-a8c7-86a65e96067b" ┆ "ZBESQ"      ┆ 45.983 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ "2025-07-13 04:59:14" ┆ "c898e98f-f378-4803-8f3c-bb8bb60cd89d" ┆ "JMJSDH"     ┆ 7.153  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ "2025-12-21 03:48:41" ┆ "1cee108f-98e6-4f3c-b544-18354b5b073b" ┆ "AYSNVDJM"   ┆ 39.583 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ "2025-07-26 18:12:34" ┆ "dc27434b-6174-4fc9-8f34-8461506b3bd0" ┆ "WMQLOW"     ┆ 33.723 │
└───────────────────────┴────────────────────────────────────────┴──────────────┴────────┘

[*] Elapsed Time: 323 ms
[*] Peak Memory Usage:	7772 KiB
```


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

### tail

Display the first rows of a Parquet file.

```sh
park tail <file.parquet> --num 10
```

---

## Example

```sh
park info users.parquet
park head users.parquet --num 5 [--csv]
park tail users.parquet --num 3 [--csv]
```

---

## Full help

```sh
Usage: park head [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  

Options:
  -n, --num <NUM>        [default: 5]
  -f, --format <FORMAT>  [default: table] [possible values: csv, raw, table]
  -h, --help             Print help
Usage: park tail [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  

Options:
  -n, --num <NUM>        [default: 5]
  -f, --format <FORMAT>  [default: table] [possible values: csv, raw, table]
  -h, --help             Print help
Usage: park info <FILENAME>

Arguments:
  <FILENAME>  

Options:
  -h, --help  Print help
Usage: park shape <FILENAME>

Arguments:
  <FILENAME>  

Options:
  -h, --help  Print help
Usage: park columns <FILENAME>

Arguments:
  <FILENAME>  

Options:
  -h, --help  Print help
```

---

## Examples

### Head

```sh
$ park head test_10k.parquet 
┌───────────────────────┬────────────────────────────────────────┬─────────────┬────────┐
│ date_                 ┆ user_id                                ┆ user_name   ┆ score  │
╞═══════════════════════╪════════════════════════════════════════╪═════════════╪════════╡
│ "2025-05-05 00:04:51" ┆ "c68a1ba2-bb19-4947-9b1c-246662ea7c1a" ┆ "UCQSHQQKY" ┆ 85.711 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ "2025-08-23 23:14:35" ┆ "0f1b7b2c-6d26-4269-aaea-4087535be8a1" ┆ "MTDDOMZ"   ┆ 20.237 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ "2025-04-20 03:24:19" ┆ "4c667352-81a6-448d-8b52-1412d8201c36" ┆ "JPZWNFNHW" ┆ 18.758 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ "2025-07-25 12:54:17" ┆ "a67e94dd-5a07-495d-990b-91be1b2910b9" ┆ "NSKCC"     ┆ 7.053  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ "2025-11-08 07:08:33" ┆ "a1a3e35f-a5cc-4e91-bb37-403010627955" ┆ "KUBCMPNOF" ┆ 75.526 │
└───────────────────────┴────────────────────────────────────────┴─────────────┴────────┘

[*] Elapsed Time: 323 ms
[*] Peak Memory Usage:	7932 KiB
```

### shape

```sh
$ park shape test_10k.parquet 
Filename:	test_10k.parquet
Columns:	4
Rows:		10000

[*] Elapsed Time: 285 ms
[*] Peak Memory Usage:	6556 KiB
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
