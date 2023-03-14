# pigpio-sys

Rust FFI wrapper to libpigpio.

## Build

```sh
cargo build
```

### Environment variables

Some parameters are used in build.rs.

- `LIBCLANG_INCLUDE_PATH`: Path to the target system header directory
- `LIBPIGPIO_LIB_PATH`: Path to the pigpio library
- `LIBPIGPIO_INCLUDE_PATH`: Path to the pigpio header file


### Cross build

For cross compiling, some more configurations are required.


#### Example (build for armhf)

```sh
pigpio-sys$ cat <<EOF > .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-rpi-linux-gnueabihf-gcc"
rustflags = ["-C", "link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf"]
EOF
pigpio-sys$ export LIBCLANG_INCLUDE_PATH=/usr/include/clang/7/include
pigpio-sys$ export LIBPIGPIO_LIB_PATH=/path/to/lib
pigpio-sys$ export LIBPIGPIO_INCLUDE_PATH=/path/to/include
pigpio-sys$ cargo build --target=arm-unknown-linux-gnueabihf
```

### Required package

- libpigpio1
- libpigpio-dev
- libclang-7-dev

