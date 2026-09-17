Under MemorySanitizer (`EagerChecks = true`):

1.  **`rustc`** attaches `noundef` to hidden `sret` pointer parameters (`ptr
    writable sret(...) align 8 noundef`), so LLVM's `MemorySanitizerPass` skips
    writing `0` to `__msan_param_tls[0]` at the Rust call site.
2.  **Clang** omits `noundef` on `sret` pointer parameters (`%agg.result`), so
    LLVM's `MemorySanitizerPass` checks `__msan_param_tls[0]` upon entering the
    C++ function.

When Rust passes an 8-byte aggregate/enum with uninitialized padding bytes by
value (`PassMode::Cast(i64)` without `noundef`), `rustc` writes a non-zero
shadow into `__msan_param_tls[0]`. A subsequent call from Rust to a C++ `sret`
function (`> 16` bytes return type) leaves `__msan_param_tls[0]` uncleared,
causing the C++ function to trap on stale TLS shadow.

## Usage

`.cargo/config.toml` is pre-configured with `-Zsanitizer=memory`.

### Run Standalone Binary (`src/main.rs`)

```bash
cargo run
```

### Run Test Suite (`src/lib.rs`)

-   **Control test (passes):**

    ```bash
    cargo test test_control_clean_tls_passes
    ```
-   **Repro 1 — Padded Enum:**

    ```bash
    cargo test test_msan_repro_padded_enum
    ```
-   **Repro 2 — Padded Struct (`PassMode::Cast(i64)`):**

    ```bash
    cargo test test_msan_repro_padded_struct
    ```
-   **Repro 3 — `Option<[u32; 1]>::None` (`PassMode::Cast(i64)`):**

    ```bash
    cargo test test_msan_repro_option_none
    ```
