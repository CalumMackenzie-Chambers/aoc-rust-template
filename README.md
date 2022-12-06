<h3 align="center">Advent of code rust template</h3>

<div align="center">

  [![Status](https://img.shields.io/badge/status-active-success.svg)]() [![GitHub Issues](https://img.shields.io/github/issues/kylelobo/The-Documentation-Compendium.svg)](https://github.com/kylelobo/The-Documentation-Compendium/issues) [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/kylelobo/The-Documentation-Compendium.svg)](https://github.com/kylelobo/The-Documentation-Compendium/pulls) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE-MIT) [![License](https://img.shields.io/badge/license-APACHE2.0-blue.svg)](/LICENSE-APACHE)

</div>

---

<p align="center"> Template for keeping all of your rust advent of code solutions in one place.
    <br>
</p>

## 📝 Table of Contents
- [Getting Started](#getting_started)
- [TODO](../TODO.md)
- [Acknowledgments](#acknowledgement)

## 🏁 Getting Started <a name = "getting_started"></a>

To use the repository, rust must be installed on your device. You can find [installations instructions here](https://www.rust-lang.org/tools/install)

To get started using this template just click the use this template button at the top of the repository.

Open src/solutions/day_01.rs for the day that you wish to solve and fill out the body for functions part_a and part_b. You will need to provide your data file given by Advent of Code. This file should be saved in [../data/input]().

You can run these functions by typing `cargo solve 1`

For further usage instructions see [usage](#usage)

## 🔧 Running the tests <a name = "tests"></a>
Each solution file contains two tests by default. One test for part a and another for part b.

These tests have to be provided with the test data that is given in the advent of code question. This should be put in the folder [../data/input/test_data]() with a file name XX.txt, e.g **01.txt** for day 1

You can add extra tests as required in the tests module.

You can use the following commands to run tests:

```
# Run all tests
cargo test

# Run tests for day 1
cargo test 01
```

## 🎉 Acknowledgements <a name = "acknowledgement"></a>
- [The-Documentation-Compendium](https://github.com/kylelobo/The-Documentation-Compendium) - Documentation templates
- [advent-of-code-rust-template](https://github.com/Wasabi375/advent-of-code-rust-template) - An alternative template that you might prefer!
- [Advent of Code](https://adventofcode.com/)
