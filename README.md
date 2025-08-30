
````markdown
# Bool Algebra CLI

A Rust-based command-line tool for evaluating Boolean algebra operations and generating truth tables.

## Features

- Evaluate Boolean operations: `AND`, `OR`, `NOT`, `XOR`, `NAND`, `NOR`, `XNOR`, `IMP` (implication).
- Display truth tables for individual operations or all supported operations at once.
- Supports multiple input formats for boolean values: `1`, `0`, `true`, `false`, `t`, `f`, `yes`, `no`, `y`, `n`.

## Installation

Ensure you have Rust and Cargo installed. Then:

```bash
git clone https://github.com/calebpentest/bool_algebra.git
cd bool_algebra
cargo build --release
````

Binaries will be available in `target/release/`.

## Usage

### Evaluate a Boolean operation

```bash
cargo run -- eval <operation> <A> [<B>]
```

Examples:

```bash
cargo run -- eval and 1 0      # Outputs: 0
cargo run -- eval not 1        # Outputs: 0
```

### Print the truth table for one operation

```bash
cargo run -- table <operation>
```

Example:

```bash
cargo run -- table xor

# Output:
# A B | Xor
# -------------
# 0 0 |   0
# 0 1 |   1
# 1 0 |   1
# 1 1 |   0
```

### Print truth tables for all operations

```bash
cargo run -- all-tables
```

## Supported Operations

| Operation | Description                   |
| --------- | ----------------------------- |
| `and`     | Logical AND                   |
| `or`      | Logical OR                    |
| `not`     | Logical NOT (unary)           |
| `xor`     | Exclusive OR                  |
| `nand`    | NOT (A AND B)                 |
| `nor`     | NOT (A OR B)                  |
| `xnor`    | NOT XOR (equivalence)         |
| `imp`     | Implication (A → B, `¬A ∨ B`) |

## Input Formats for Booleans

Accepted formats:

* `1`, `0`
* `true`, `false`
* `t`, `f`
* `yes`, `no`, `y`, `n`

## Development Workflow

To make changes:

1. Edit `src/main.rs`.
2. Run tests (if implemented).
3. Build:

   ```bash
   cargo build
   ```
4. Run as above to verify behavior.


