# Mosfhet-RS

**Mosfhet-RS** is a Rust binding for the Mosfhet cryptographic library written in C. It provides Rust access to the cryptographic primitives and functions implemented in the Mosfhet C library.

## Prerequisites

Before building the project, ensure you have the following installed:

- Rust (stable)
- A C compiler (e.g., `gcc` or `clang`)
- `bindgen` and `cc` Rust crates

## Build Instructions

To build the project, simply run:

```bash
cargo build
```

This will:
- Compile the C code from the `mosfhet` library.
- Generate Rust bindings for the C functions using `bindgen`.
- Build the Rust project and link the C library.

If the build is successful, it will create an executable or library depending on your project's configuration.

### Build Flags

The `build.rs` script automatically enables certain CPU-specific instructions, such as:
- `RDRAND` (`-mrdrnd`)
- `AVX` (`-mavx`)
- `AVX2` (`-mavx2`)

Ensure your environment supports these instructions. If you're building on a machine without these features, modify the `build.rs` or the C source code to provide a fallback.

## Usage

Once built, you can use the Mosfhet library in your Rust code by including the generated bindings in `main.rs`:

```rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        // Example: Call functions from the Mosfhet library
        // let result = some_function_from_mosfhet();
    }
}
```

Replace `some_function_from_mosfhet` with the actual functions from the Mosfhet library.

## Troubleshooting

### Compilation Issues
If you encounter errors related to AVX or RDRAND instructions, ensure that:
- Your CPU supports the required instructions.
- The correct compiler flags (`-mavx`, `-mavx2`, `-mrdrnd`) are enabled.

You can modify the `build.rs` to remove or adjust these flags if necessary.

### Rebuilding

If you make changes to the C code or the build script, you can force a clean build by running:

```bash
cargo clean
cargo build
```

## License

This project is licensed under the same terms as the Mosfhet library. See [LICENSE](https://github.com/antoniocgj/MOSFHET/blob/main/LICENSE) for more details.