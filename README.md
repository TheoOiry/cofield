# Cofield receiver

The CLI tool to receive data from the flex sensor glove of the Cofield project

## Build

To build the cli run `cargo build --release` then find it under `/target/release/cofield-receiver`

## How to use

Run `cofield-receiver --help` to see the diffrents options

You can store the output by redirecting stdout to a file like that:
`cofield-receiver -o csv > data.csv`
