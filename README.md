# py_nrf
Python interface for the newton_rootfinder crate


## Workflow

In order to generate a wheel valid for you computer, you need to:

1. Install [Rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)
2. Install a Python version and install [maturin](https://pypi.org/project/maturin/): `pip install maturin`
3. Switch to Rust [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#rustup-and-the-role-of-rust-nightly) (this is due to the PyO3 dependency): `rustup default nightly`
4. Build the crate with `cargo build --release` (if it is the first build, an internet connexion is required to download dependencies from http://crates.io). If you are using macOS, check the nota bene.
5. Create the Python wheel by running `maturin build`. This build the wheel in the `target/wheels` directory. Alternatively, you can use `maturin develop` which also install it in you current Python environment.

N-B: If you are using macOS, you need to run the following command line: `cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup`. You can also edit your `.cargo/config` file and add the following content:

```
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```


## Resources

- Don't hesitate the check [maturin's documentation](https://pypi.org/project/maturin/)
- The file `./python/main.py` gives an example of using the newton-raphson solver. The model used is in `./python/model.py`. It is assumed to be run from the `python` folder using `python main.py`
- Don't hesitate to check the [newton_rootfinder](https://crates.io/crates/newton_rootfinder) crate documentation to know all the parameters you can set up. Only a subset of those is available in Python.
