#!/usr/bin/env bash
# Test script to verify builds work locally before pushing to CI

set -e

echo "Testing local builds..."
echo ""

# Check if cross is installed (only needed for aarch64)
NEED_CROSS=false
for target in "${TARGETS[@]}"; do
    if [[ "$target" == "aarch64-unknown-linux-gnu" ]]; then
        NEED_CROSS=true
        break
    fi
done

if $NEED_CROSS; then
    if ! command -v cross &> /dev/null; then
        echo "WARNING: 'cross' not found (needed for aarch64). Install with:"
        echo "   cargo install cross --git https://github.com/cross-rs/cross"
        echo ""
        read -p "Install cross now? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            cargo install cross --git https://github.com/cross-rs/cross
        else
            echo "ERROR: Cannot proceed without cross for aarch64. Exiting."
            exit 1
        fi
    fi

    # Check if Docker is running (required for cross)
    if ! docker info &> /dev/null; then
        echo "ERROR: Docker is not running. Please start Docker and try again."
        echo "   cross requires Docker to create cross-compilation environments."
        exit 1
    fi
fi

# Array of targets to test
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
)

# Track results
PASSED=()
FAILED=()

echo "Testing Linux targets with cross..."
echo "======================================"
echo ""

for target in "${TARGETS[@]}"; do
    echo "Building for $target..."

    # Use cross only for aarch64, cargo for others
    if [[ "$target" == "aarch64-unknown-linux-gnu" ]]; then
        BUILD_CMD="cross build --release --target $target"
    else
        BUILD_CMD="cargo build --release --target $target"
    fi

    if $BUILD_CMD; then
        echo "SUCCESS: $target"
        PASSED+=("$target")
    else
        echo "FAILED: $target"
        FAILED+=("$target")
    fi
    echo ""
done

# Test native Windows build if on Windows
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo "Building for x86_64-pc-windows-msvc (native)..."
    if cargo build --release --target x86_64-pc-windows-msvc; then
        echo "SUCCESS: x86_64-pc-windows-msvc"
        PASSED+=("x86_64-pc-windows-msvc")
    else
        echo "FAILED: x86_64-pc-windows-msvc"
        FAILED+=("x86_64-pc-windows-msvc")
    fi
    echo ""
fi

# Summary
echo "======================================"
echo "Summary"
echo "======================================"
echo ""
echo "Passed: ${#PASSED[@]}"
for target in "${PASSED[@]}"; do
    echo "   - $target"
done
echo ""

if [ ${#FAILED[@]} -gt 0 ]; then
    echo "Failed: ${#FAILED[@]}"
    for target in "${FAILED[@]}"; do
        echo "   - $target"
    done
    echo ""
    exit 1
else
    echo "All builds passed!"
    echo ""
    echo "Binaries are in: target/<target>/release/gitnav"
fi
