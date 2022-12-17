# Genesis Life Simulator

The project aims to be a performant life simulator written in Rust.

It is currently in very early stages of development and is primarily a side
project for me to learn Rust.

## Running the simulation

Install Rust and Bevy OS dependencies as described
[here](https://bevyengine.org/learn/book/getting-started/setup/).

Clone this repository and run `cargo r --release`. You may also want to adjust
the config in `./config/genesis.toml`. It is currently set up in more of
a development mode.

When you close a simulation it will produce a `run_data.json` file containing
the run's statistics.

## Acknowledgements

The following projects have helped inspire this one:

* [The Bibites](https://leocaussan.itch.io/);
* [NEAT-Python](https://neat-python.readthedocs.io/en/latest/);
* [NEAT-rs](https://github.com/stjepangolemac/neat-rs); and
* [Shorelark](https://github.com/Patryk27/shorelark).
