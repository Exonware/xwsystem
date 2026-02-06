# Build Fixes for xwsystem

## Fixing aws-lc-sys NASM Requirement

The `aws-lc-sys` crate (used by `reqwest` and `mongodb` via `rustls`) requires NASM on Windows, but you can use prebuilt objects instead.

### Solution: Set Environment Variable

Before running `cargo build` or `cargo check`, set the following environment variable:

**Windows (CMD):**
```cmd
set AWS_LC_SYS_PREBUILT_NASM=1
cargo build
```

**Windows (PowerShell):**
```powershell
$env:AWS_LC_SYS_PREBUILT_NASM="1"
cargo build
```

**Permanent Fix (Windows):**
1. Open System Properties → Environment Variables
2. Add new System Variable: `AWS_LC_SYS_PREBUILT_NASM` = `1`
3. Restart your terminal/IDE

### Alternative: Install NASM

If you prefer to install NASM:
1. Download from https://www.nasm.us/
2. Install and add to PATH
3. No environment variable needed

## Fixing cbor Compatibility Issues

The old `cbor` crate has been replaced with `ciborium` (modern, maintained CBOR library).

- `cbor` crate removed (had compatibility issues with rustc-serialize)
- `ciborium` added as optional dependency
- Enable with: `cargo build --features cbor`

## Optional Dependencies

- `bcrypt` - Now optional (enable with `--features bcrypt`)
- `cbor` - Now optional (enable with `--features cbor`)

Both are optional because:
- `bcrypt` requires NASM (via ring/aws-lc-sys)
- `cbor` was replaced with `ciborium` for better compatibility

