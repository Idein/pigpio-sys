# pigpio-sys

Rust FFI wrapper to libpigpio.

## Build

```sh
cargo build
```

### Environment variable


- `LIBCLANG_INCLUDE_PATH`: Path to the root of the target system header directory

    ```sh
    LIBCLANG_INCLUDE_PATH=/usr/include/clang/7/include cargo build
    ```


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
pigpio-sys$ cargo build --target=arm-unknown-linux-gnueabihf
```

### Required package

- libpigpio1
- libpigpio-dev
- libclang-7-dev

