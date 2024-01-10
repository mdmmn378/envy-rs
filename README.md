# envy - A `.env` Example Generator

This project is a Rust-based tool that generates `.env` example files from existing `.env` files. It's useful for creating template environment files without exposing sensitive information.

## Features

- Reads `.env` files and generates `.env.example` files.
- Supports `.env` file format.
- Removes comments and cleans up the text in the `.env` files.

## Installation

Install the binary with the following command -

```sh
cargo install --path .
```

## Usage

The main command for the tool is `generate`, which requires the path to the `.env` file as an argument.

```sh
envy generate path/to/.env
```

This will generate a `.env.example` file in the same directory as the `.env` file.

## Development

This project uses Cargo, the Rust package manager.

To build the project:

```sh
cargo build
```

To run tests:

```sh
cargo test
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)

```

Please replace the license link with the actual link to your license file if you're not using the MIT license.
```
