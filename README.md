# Unfolder

![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://github.com/pre-commit/pre-commit)

An extremely fast directory exploration tool to find:
- [x] Largest files
- [x] Duplicated files
- [ ] ... to be continued

In directories of any size and structure.

## Use cases

Unfolder can be useful for:

* **Software maintainers** to reduce repo size and eliminate duplicate files, within or across projects.
* **Project managers** to avoid extra data storage costs and have single location for each key artifact.

## Benchmarks

Unfolder analyzes codebases of large open-source projects in under half a second:

| Project                                             | Files | Folders | Elapsed time, ms |
|-----------------------------------------------------|-------|---------|------------------|
| [Apache Airflow](https://github.com/apache/airflow) | 7,558 | 1,713   | 409              |
| [Ruff](https://github.com/astral-sh/ruff)           | 7,374 | 615     | 328              |
| [React](https://github.com/facebook/react)          | 6,467 | 532     | 406              |
| [CPython](https://github.com/python/cpython)        | 5,182 | 420     | 253              |
| [Kedro](https://github.com/kedro-org/kedro)         | 527   | 122     | 152              |

## Getting started

### Installation

Currently, only installation from source is supported:

1. Make sure you have Rust toolchain set up.
    - This can either be done [as the Rust guide suggests](https://www.rust-lang.org/tools/install).
    - Or if you've using [RustRover IDE](https://www.jetbrains.com/rust/), it manages it automatically.
1. Clone [project repo](https://github.com/yury-fedotov/unfolder) locally, and `cd` there.
1. Run `cargo build --release` to build the binary executable file for the tool.
1. Run `cargo install --path .` to install this executable and make it available under `unfolder` namespace in CLI.

### Usage

The tool currently has just one CLI command which is available as:

```bash
unfolder path/to/directory/
```

In addition to path to directory, it can take 3 optional arguments:

| Argument                                             | Short | Long            | Options                                                                                           | Default       |
|------------------------------------------------------|-------|-----------------|---------------------------------------------------------------------------------------------------|---------------|
| List of file extensions to analyze                   | -e    | --extensions    | Comma-separated: e.g. py,png                                                                      | All           |
| Minimum file size to consider for duplicate analysis |       | --min_file_size | One of the following alias: blank, config, code, excel, document, image, gif, audio, video, large | code (100 Kb) |
| Number of largest files to return based on size      | -n    | --n_top         | Any positive integer                                                                              | 5             |

So for example:

```bash
unfolder path/to/directory/ -e csv,pkl,png,gif --min_file_size image
```

Would:
* Analyze `path/to/directory/`.
* Consider only files of `csv`, `pkl`, `png` and `gif` extensions.
* While identifying duplicates, ignore files smaller than `image` alias implies (10 Mb).

You can also run `unfolder -h` to get info on arguments.
