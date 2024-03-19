# Price Impact on a Constant Product Market Maker (CPMM)

Assuming a constant product market maker pool with two tokens, A and B, the price of token A in terms of token B is given by the following formula:

$A • B = k$

where A is the amount of token A in the pool, B is the amount of token B in the pool, and k is a constant.

## Introduction

This project attempts to calculate the price impact of a trade in a constant product market maker. However, it is important to note that this code should not be considered a substitute for learning about DEFI Uniswap V2 constant product market maker in-depth. It is always recommended to thoroughly understand the underlying concepts and principles before relying solely on automated calculations.

## Definitions

- `A`: The token A in the pool
- `B`: The token B in the pool
- `k`: A constant
- `x`: The amount of token A after the trade
- `y`: The amount of token B after the trade
- `delta` and `lambda`: amount out `x` and amount in `y` depending on whether token A or token B is being bought or sold
- `p`: The price of token A in terms of token B before the trade
- `pe`: The effective price of token A in terms of token B after the trade (i.e., the price actual price paid of token A in terms of token B after accounting for the price impact)
- `i`: The price impact of the trade
- `pi`: The price of token A in terms of token B after the trade

## Getting Started

### Prerequisites

- Rust
- Cargo

### Installation

1. Clone the repo

```sh
git clone https://github.com/mmsaki/price-impact-calculations.git
```

2. Install the Rust programming language and Cargo package manager

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Navigate to the project directory

```sh
cd price-impact-calculations
```

4. Install the dependencies

```sh
cargo install
```

### Usage

To run the project, execute the following command:

```rust
cargo run
```

### Example

```rust
fn main() {
    let a = 200;                        // ETH
    let b = 800_000;                    // USDC
    let k = a * b;                      // constant
    let delta = 200_000;                // usdc amount_in
    let p = b / a;                      // price before trade
    let pe = lambda / (a - x);          // effective price after trade
    let i = lambda - p * (a - x);       // price impact of the trade
    let pi = y / x                      // price after trade
    println!("Price Impact: {}", i);
    println!("Price after trade: {}", pi);
}
```

## License

Distributed under the MIT License. See `LICENSE` for more information.
