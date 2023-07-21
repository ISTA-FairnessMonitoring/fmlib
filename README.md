# FMLib

A Rust implementation for monitoring fairness over Markov chains.

This repository is mainly a Rust library. For the technical details 
of our monitors, please refer to our CAV '23 paper
[_Monitoring Algorithmic Fairness_](https://doi.org/10.1007/978-3-031-37703-7_17).

## Getting Started

To install Rust, please visit this page: 
https://www.rust-lang.org/tools/install.

Experimental scenarios are defined in files `src/experiments*.rs`.

**We are in a clean-up process for our experiments!** Please visit 
this page in a short while.

## Running Tests

In addition to the experimental scenarios, this library is 
accompanied with a suite of unit-tests. All tests are located in the
`tests/` directory. You can either run them in your terminal (using 
cargo commands), or use IDE-specific unit-testing tools.

For running using the terminal, navigate to root folder and run the 
following:
```
$ cargo test
```

## Contributors 

* Mahyar Karimi 
* Konstantin Kueffner

To contact us, please use the email address pattern 
`<first>.<last>@ist.ac.at`.