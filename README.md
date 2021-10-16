# pr

> text-based markdown presentation tool

## Functionality

This program parses and renders a markdown file into a powerpoint

## Usage

- `Command`: pr [path]

## Keybinding

- <n>: Next slide
- <p>: Previous slide
- <q>: Quit Program

## Installation

### Cargo

Run:

```sh
$ cargo install pr
```

### Build From Source

Run the following command:

```sh
// clone the repository

$ git clone https://github.com/Z5483/pr.git

// cd into the repository

$ cd pr

// Build the program

$ make
```

To install the program, run:

```sh
$ make install
```

The default install prefix is `/usr/local`, you can change it by setting the
`PREFIX` variable at the beginning of the above command.
