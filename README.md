<p align="center">
  <img src="doc/logo.png">
</p>
<p align="center">
  <a href="https://travis-ci.org/xmr-rs/xmr">
    <img src="https://travis-ci.org/xmr-rs/xmr.svg?branch=master" title="Travis CI Status">
  </a>
  <a href="https://ci.appveyor.com/project/jeandudey/xmr">
    <img src="https://ci.appveyor.com/api/projects/status/h34w8k04857dmkuc?svg=true" title="AppVeyor CI Status">
  </a>
</p>

# Xmr Monero node

Xmr is an implementation of the Monero cryptocurrency. It aims to be a full
Monero node with wallet functionality.

*This is a work in progress that is not yet functional*

## Building Xmr

Xmr is built with the [Cargo][1] package manager. First, ensure you have the
latest stable Rust version (it hasn't been tested with older Rust versions).
Then run this in the `xmr` repository folder:

[1]: https://crates.io/

```bash
cargo build
```

This will create the `dxmr` binary in the *target/debug* directory.

To build the binary optimized for release:

```bash
cargo build --release
```

## License
Some parts of the code are GPLv3 due to them being base on the `parity-bitcoin` code.
However the code that isn't related to the `parity-bitcoin` code is licensed under
the MIT or Apache 2.0 at your option.
