# Rust Book — Studies

My walkthrough of [_The Rust Programming Language_](https://doc.rust-lang.org/book/) (a.k.a. "the book"). Each numbered directory is a small, self-contained example or exercise from a chapter.

## Layout

| Directory | Chapter | Notes |
|---|---|---|
| [`001_hello_world`](./001_hello_world) | 1.2 — Hello, World! | Single `main.rs` compiled directly with `rustc`. |
| [`002_hello_cargo`](./002_hello_cargo) | 1.3 — Hello, Cargo! | Same program, managed with Cargo (edition 2024). |
| [`003_guessing_name`](./003_guessing_name) | 2 — Programming a Guessing Game | Interactive number-guessing loop using the `rand` crate. |
| [`004_variables_shadowing`](./004_variables_shadowing) | 3.1 — Variables and Mutability | Variable shadowing across scopes and type changes. |
| [`005_functions`](./005_functions) | 3.3 — Functions | Parameters, return values, and blocks as expressions. |
| [`006_control_flow`](./006_control_flow) | 3.5 — Control Flow | `if` expressions, `loop`/`while`/`for`, and labeled breaks. |

## Running an example

For raw `rustc` examples:

```sh
cd 001_hello_world
rustc main.rs && ./main
```

For Cargo projects:

```sh
cd 002_hello_cargo
cargo run
```

## Toolchain

Install via [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This repo tracks the stable toolchain.
