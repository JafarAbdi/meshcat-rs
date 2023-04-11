# meshcat

[![Crates.io](https://img.shields.io/crates/v/meshcat.svg)](https://crates.io/crates/meshcat)
[![Docs.rs](https://docs.rs/meshcat/badge.svg)](https://docs.rs/meshcat)
[![CI](https://github.com/JafarAbdi/meshcat-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/JafarAbdi/meshcat-rs/actions/workflows/ci.yml)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install meshcat`

## Usage

First you need to install meshcat package to open the server:

```bash
# https://mamba.readthedocs.io/en/latest/installation.html
micromamba create -f environment.yml
micromamba run -n meshcat meshcat-server --open
```

See the [demo](https://github.com/JafarAbdi/meshcat-rs/blob/main/examples/demo.rs) in the examples folder

https://user-images.githubusercontent.com/16278108/231297164-04a4dc92-c612-49aa-8700-074935fd2ec5.mp4


## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).
