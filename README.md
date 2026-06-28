# sol-program-counter

A small Solana program written in Rust that keeps a doubling counter on chain.

This is a simple example to learn how a Solana program reads from an account,
changes some data, and writes it back.

## What it does

The program stores a single number (the count) inside an account. Each time
you call the program, it updates the count:

- If the count is 0, it becomes 1
- If the count is more than 0, it doubles

So if you keep calling it, the count grows like this:

```
1, 2, 4, 8, 16, 32, ...
```

## How it works

1. The program reads the first account passed to it. This account holds the
   counter.
2. It reads the current count from the account data.
3. It works out the new count (set to 1 if it was 0, otherwise double it).
4. It writes the new value back to the account.

## Data format

The counter is stored as a `u32` value. The program uses Borsh to turn the
data into bytes and back again.

## Project layout

```
sol-program-counter/
  src/lib.rs     the program code
  Cargo.toml     project settings and dependencies
```

## Build

You need Rust and the Solana tool suite installed.

```bash
cargo build-sbf
```

This builds the program into a `.so` file that you can deploy to a Solana
cluster.

## Deploy

```bash
solana program deploy ./target/deploy/sol_program_counter.so
```

Run this against your chosen cluster, for example a local validator or devnet.
