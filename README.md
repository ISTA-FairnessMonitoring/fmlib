# FMLib

This a Rust library for trying out different fairness monitoring approaches.

## Getting Started

Open `main.rs`; it will have a structure similar to the following:

```rust
fn main() {
    env_logger::init();

    _frequentist();
}

fn _frequentist() { ... }
.
.
.
```

Select one of the functions defined in `main.rs`; then, in your terminal, type the following:

```
$ RUST_LOG=info cargo run
```

You can change the log level to adjust its verbosity.

## Running Tests

All tests are located in the `tests/` directory.
You can either run them in your terminal (using cargo commands),
or use IDE-specific tools (VS Code has [an extension] for this matter.)
For running using the terminal, navigate to the repository folder and run the following:
```
$ cargo test
```

## Directory Structure

`drafts/` Some rough calculations, not part of the monitoring code

`src/` Contains modules for standard Markov chains, various monitoring techniques, and project-wide source files

`target/` Build folder, from RUST

`tests/` Unit tests, which create monitors, computes the outputs of the monitors by running the Markov chain up to some pre-specified number of steps, and then checks if the true values of the formulas are inside the bounds computed by the monitor or not

## Documentation

All documentation for this project will be cumulatively written in the Wiki pages.

[an extension]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## Todo:

* P-value
* Bayes factor (?)
* Register based frequentist
* Some operations in both the Bayesian and the frequentist are missing
