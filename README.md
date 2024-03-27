# Bitwallet Lite

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

```sh
# Clone the repository
git clone git@github.com:isaack-njama/bitwallet.git

# Navigate to the project directory
cd bitwallet

# Build the project
cargo build
```

## Usage

```sh
# Run the project
cargo run
```

## Testing

```sh
# Run tests
cargo test
```

### Run Individual Tests

```sh
# Replace <test-feature> with feature to run test on. See list of tests.
cargo test --package bit_wallet_solution --bin bit_wallet_solution -- wallet_struct_test::tests::<test-name> --exact --nocapture
```

Here is a list of individual tests you can run:

- test_generate_mnemonic: Test the generation of a mnemonic phrase.
- test_create_wallet: Tests the creation of a wallet.
- test_get_wallet: Tests retrieving a wallet.
- test_import_wallet: Tests importing a wallet.
- test_get_address: Tests retrieving an address from a wallet.
- test_get_transactions: Tests retrieving an address from a wallet.
- test_get_balance: Tests retrieving the balance of a wallet.
- test_send_bitcoin: Tests sending bitcoins from a wallet to a recipient address.

## Contributing

Contributions are welcome! If you'd like to contribute to this project, feel free to fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the [MIT License](./LICENSE).
