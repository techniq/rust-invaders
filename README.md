Followed along with the wonderful [Rust Bevy Full Tutorial](https://www.youtube.com/watch?v=j7qHwb7geIM)

```rs
cargo run
```

## WASM

To setup and run via WASM

Add WASM support to your Rust installation

```sh
rustup target install wasm32-unknown-unknown
```

Install 

```sh
cargo install wasm-server-runner
```

Set up cargo to use it, in `.cargo/config.toml`
```
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
```

```sh
cargo run --target wasm32-unknown-unknown
```

More details available in the [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/platforms/wasm.html)