# PowerShell script to test builds on Windows
# Test script to verify builds work locally before pushing to CI

Write-Host "Testing local builds..." -ForegroundColor Cyan
Write-Host ""

# Check if cross is installed (only needed for aarch64)
$needCross = $false
foreach ($target in $targets) {
    if ($target -eq "aarch64-unknown-linux-gnu") {
        $needCross = $true
        break
    }
}

if ($needCross) {
    $crossInstalled = Get-Command cross -ErrorAction SilentlyContinue
    if (-not $crossInstalled) {
        Write-Host "WARNING: 'cross' not found (needed for aarch64). Install with:" -ForegroundColor Yellow
        Write-Host "   cargo install cross --git https://github.com/cross-rs/cross"
        Write-Host ""
        $install = Read-Host "Install cross now? (y/n)"
        if ($install -eq 'y' -or $install -eq 'Y') {
            cargo install cross --git https://github.com/cross-rs/cross
        } else {
            Write-Host "ERROR: Cannot proceed without cross for aarch64. Exiting." -ForegroundColor Red
            exit 1
        }
    }

    # Check if Docker is running (required for cross)
    try {
        docker info | Out-Null
    } catch {
        Write-Host "ERROR: Docker is not running. Please start Docker and try again." -ForegroundColor Red
        Write-Host "   cross requires Docker to create cross-compilation environments."
        exit 1
    }
}

# Array of targets to test
$targets = @(
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-gnu"
)

# Track results
$passed = @()
$failed = @()

Write-Host "Testing Linux targets with cross..." -ForegroundColor Cyan
Write-Host "======================================"
Write-Host ""

foreach ($target in $targets) {
    Write-Host "Building for $target..." -ForegroundColor Yellow

    # Use cross only for aarch64, cargo for others
    if ($target -eq "aarch64-unknown-linux-gnu") {
        $result = & cross build --release --target $target 2>&1
    } else {
        $result = & cargo build --release --target $target 2>&1
    }

    if ($LASTEXITCODE -eq 0) {
        Write-Host "SUCCESS: $target" -ForegroundColor Green
        $passed += $target
    } else {
        Write-Host "FAILED: $target" -ForegroundColor Red
        $failed += $target
    }
    Write-Host ""
}

# Test native Windows build
Write-Host "Building for x86_64-pc-windows-msvc (native)..." -ForegroundColor Yellow
$result = & cargo build --release --target x86_64-pc-windows-msvc 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "SUCCESS: x86_64-pc-windows-msvc" -ForegroundColor Green
    $passed += "x86_64-pc-windows-msvc"
} else {
    Write-Host "FAILED: x86_64-pc-windows-msvc" -ForegroundColor Red
    $failed += "x86_64-pc-windows-msvc"
}
Write-Host ""

# Summary
Write-Host "======================================"
Write-Host "Summary" -ForegroundColor Cyan
Write-Host "======================================"
Write-Host ""
Write-Host "Passed: $($passed.Count)" -ForegroundColor Green
foreach ($target in $passed) {
    Write-Host "   - $target"
}
Write-Host ""

if ($failed.Count -gt 0) {
    Write-Host "Failed: $($failed.Count)" -ForegroundColor Red
    foreach ($target in $failed) {
        Write-Host "   - $target"
    }
    Write-Host ""
    exit 1
} else {
    Write-Host "All builds passed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Binaries are in: target\<target>\release\gitnav.exe"
}
