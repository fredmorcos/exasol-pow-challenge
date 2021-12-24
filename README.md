# `exasol-pow-challenge`: Protocol and proof-of-work implementation for the Exasol POW coding challenge

[Github Repository](https://github.com/fredmorcos/exasol-pow-challenge)

## Building

It is recommended to build in release mode, and perhaps even pass `target-cpu=native` to
the Rust compiler:

```sh
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

To build with stats output and extra checks (slower runtime):

```sh
RUSTFLAGS="-C target-cpu=native" cargo build --release --features stats
```

## Usage

The application takes two required arguments, one is a key and certificates PEM file, and
the other is a JSON file containing the user data to submit.

The key and certificates PEM file must contain an EC private key and a certificate chain.

The user data JSON file has the following format:

```json
{
    "name": "FirstName LastName",
    "emails": [
        "email1@domain.com",
        "email2@domain.com"
    ],
    "birth_date": "01.01.2001",
    "country": "France",
    "address": [
        "Address Line 1",
        "Address Line 2"
    ]
}
```

Notes:

- The `name` field must be a first and last name separated by a space.

- The `birth_date` must be in the `strftime` format `%d.%m.%Y`.

- The list of accepted country names can be found
  [here](https://www.countries-ofthe-world.com/all-countries.html).

### Command-line

To print out more information about what the application is doing, use `-v`. Multiple
occurrences of `-v` on the command-line will increase the verbosity level:

```sh
exasol-pow-challenge -vvv --cert-file CERT-FILE --data-file DATA-FILE
```

To find help, see the `--help` flag:

```sh
exasol-pow-challenge --help
```
