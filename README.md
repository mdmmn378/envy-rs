# envy - A Config File Example Generator

---

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/mdmmn378/envy-rs/ci.yaml)

Envy is a tool that generates example configuration files from your existing `.env` or `.toml` files. It masks sensitive information and helps you maintain up-to-date example configuration files for your projects.

![Example](./docs/example.gif)

## Installation

Install the binary with the following command:

```sh
cargo install --path .
```

## Usage

The main command for the tool is `generate`, which requires the path to the `.env` or `.toml` file as an argument.

```sh
envy generate path/to/.env
```

or

```sh
envy generate path/to/config.toml
```

This will generate a `.env.example` or `example.toml` file in the current directory.

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

[MIT](MIT-LICENSE.txt)
