# C <-> Rust Pointer Experiment
Messing around with pointers between Rust and C.
The "fake C lib" is inspired by libGPhoto2 as that's where the struggle started.

## Quickstart
```console
make
```

## Layout
### `./gp_c/`
Simple C library source
### `./gp_rs/`
Rust source and binndings to the C library to match `./gp_c/main.c`
