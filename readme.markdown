# Rust Rust Revolution

Second revision of the repository.

---

## Development

### Environment Setup

1. Download and install rustup. _(**required**)_

    > [https://rustup.rs/](https://rustup.rs/)
    > or install with your favourite package manager.

2. Download and install Visual Studio Code. _(**optional**, if you know an alternative)_

    > [https://code.visualstudio.com/Download](https://code.visualstudio.com/Download)
    > or install with your favourite package manager.

    - Install all of the recommended visual studio code extensions. _(**required**)_

3. Run all of the following commands in your terminal. _(**required**)_

    > ```sh
    > # Rust Toolchain
    > rustup toolchain install nightly
    > rustup target add wasm32-unknown-unknown --toolchain nightly
    > rustup component add llvm-tools-preview --toolchain nightly
    >
    > # Project Tools
    > cargo install cargo-make
    > cargo install cargo-watch
    > cargo install miniserve --locked
    > cargo install -f wasm-bindgen-cli
    > cargo install wasm-pack
    > ```

---

### Build

See [makefile.toml](makefile.toml), and any nested maketile.toml, for more details.

#### Pre-build Workspace

Builds all packages, filling intermediate files for quick iteration.

`cargo make`

#### CLI Application

`cargo make build`

#### WASM Library

`cargo make wasm`

---

### Web Iteration / Testing

- In Visual Studio Code's integrated terminal look for the `Watch - WASM` and `Serve - WASM` tasks.
- `Watch - WASM`: informs of any compiler errors.
- `Serve - WASM`: supplies the url to navigate to for previewing game.

#### (Extra) Manual Web Iteration

> This is only necessary if you are not using Visual Studio Code.

```sh
cd crates/rrr_wasm
cargo make dev
```

---

## Shell Completions

> If your shell is not listed here,
> please check completions list,
> add instructions, and make a PR.

### Powershell

```powershell
rrr completions powershell | Out-String | Invoke-Expression
```
