# GitHub Actions Workflows Documentation

**Project**: gitnav
**Last Updated**: November 2, 2025
**Maintainer**: Development Team

---

## Overview

This project uses GitHub Actions to automate testing, code quality checks, and release builds. All workflows are defined in `.github/workflows/`.

---

## Workflows

### 1. **tests.yml** - Continuous Integration (Pull Requests & Commits)

**Trigger**:

- Push to `main`, `feature/**`, or `bugfix/**` branches
- Pull requests to `main`
- Scheduled: Daily at 1 AM UTC (catches dependency issues)

**Jobs**:

#### 1.1 Test Job (Multi-Platform)

- **Runs on**: Ubuntu, macOS, Windows
- **Rust Versions**:
  - Stable (all platforms)
  - 1.80.0 (MSRV - Minimum Supported Rust Version on Linux)
- **Tests**:
  - ✅ Debug build tests
  - ✅ Release build tests
  - ✅ All unit tests

**What it does**:

```bash
cargo test --verbose              # Debug tests
cargo test --release --verbose    # Release tests
cargo test --doc --verbose        # Doc comment tests
```

**Expected Result**:

- 106 tests pass on all platforms
- Execution time: ~1-2 minutes per platform

#### 1.2 Clippy Job (Code Linting)

- **Runs on**: Ubuntu
- **Checks**:
  - Rust best practices
  - Code anti-patterns
  - Performance issues
  - Security concerns

**What it does**:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Expected Result**: No warnings or errors

#### 1.3 Formatting Job (Code Style)

- **Runs on**: Ubuntu
- **Checks**: Code formatting compliance (rustfmt)

**What it does**:

```bash
cargo fmt -- --check
```

**Expected Result**: All code is properly formatted

#### 1.4 Build Job (Multi-Platform)

- **Runs on**: Ubuntu, macOS, Windows
- **Builds**:
  - ✅ Debug binary
  - ✅ Release binary

**What it does**:

```bash
cargo build --verbose              # Debug build
cargo build --release --verbose    # Release build
```

**Expected Result**: Successful build on all platforms

#### 1.5 Security Audit Job

- **Runs on**: Ubuntu
- **Tool**: cargo-audit
- **Checks**: Known security vulnerabilities in dependencies

**What it does**:

```bash
cargo audit
```

**Expected Result**: No known vulnerabilities

#### 1.6 Code Coverage Job

- **Runs on**: Ubuntu
- **Tool**: cargo-tarpaulin
- **Reports**: Code coverage metrics
- **Upload**: Results to Codecov

**What it does**:

```bash
cargo tarpaulin --out Xml --timeout 300 --exclude-files tests/**
```

**Expected Result**: ~65% code coverage (current baseline)

#### 1.7 Documentation Job

- **Runs on**: Ubuntu
- **Checks**: All documentation comments
- **Tool**: cargo doc with warnings-as-errors

**What it does**:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
```

**Expected Result**: All doc comments are valid and properly formatted

#### 1.8 Test Results Summary

- **Aggregates** results from all jobs
- **Reports** status in job summary
- **Available** in PR checks

---

### 2. **release.yml** - Release Build & Distribution (Manual)

**Trigger**:

- Push of semantic version tags matching `[0-9]+.[0-9]+.[0-9]+` (e.g., `0.1.0`, `0.2.0`)
- Manual workflow dispatch via GitHub UI

**Overview**:

This workflow automates the entire release pipeline without external dependencies. It uses standard Rust tooling (`cargo build`) with manual cross-compilation and archiving to create releases across 6 platforms.

**Key Features**:

- Version validation (tag must match Cargo.toml)
- Semantic versioning enforcement
- Draft releases prevent premature publication
- Parallel builds for all 6 platforms
- SHA256 checksums for integrity verification
- Uses `gh` CLI for reliable artifact uploads

**Jobs**:

#### 2.1 Create Release

- **Runs on**: Ubuntu
- **Validates**: Git tag matches Cargo.toml version
- **Creates**: Draft GitHub release

**What it does**:

```bash
# Extract version from tag
VERSION="${{ github.ref_name }}"

# Verify version matches Cargo.toml
grep -q "version = \"$VERSION\"" Cargo.toml

# Create draft release
gh release create $VERSION --draft --verify-tag --title "Release $VERSION"
```

**Expected Result**: Draft release created, ready for artifacts

#### 2.2 Build Release (Multi-Platform, Parallel)

- **Runs on**: Different OS runners for each platform
- **Platforms** (6 total):
  - Linux x86_64 (GNU libc)
  - Linux x86_64 (musl - fully static)
  - Linux ARM64 (aarch64)
  - macOS x86_64
  - macOS ARM64 (Apple Silicon)
  - Windows x86_64 (MSVC)

**What it does** (for each platform):

```bash
# 1. Install Rust toolchain for target
rustup target add ${{ matrix.target }}

# 2. Install cross-compilation tools (Linux only)
sudo apt-get install -y musl-tools gcc-aarch64-linux-gnu

# 3. Build optimized release binary
cargo build --release --target ${{ matrix.target }}

# 4. Create archive (tar.gz or zip)
tar czf gitnav-${{ matrix.target }}.tar.gz gitnav/
# or
Compress-Archive -Path gitnav -DestinationPath gitnav-${{ matrix.target }}.zip

# 5. Generate SHA256 checksum
sha256sum gitnav-${{ matrix.target }}.* > gitnav-${{ matrix.target }}.sha256

# 6. Upload artifacts
```

**Expected Result**: Compiled binaries, archives, and checksums for all 6 platforms

#### 2.3 Publish Release

- **Runs on**: Ubuntu
- **Waits for**: All build jobs to complete
- **Publishes**: All artifacts to GitHub Release

**What it does**:

```bash
# 1. Download all platform artifacts
gh run download $RUN_ID

# 2. Collect all archives and checksums
find artifacts -type f \( -name "*.tar.gz" -o -name "*.zip" -o -name "*.sha256" \)

# 3. Upload to release
gh release upload $VERSION *.tar.gz *.zip *.sha256 --clobber
```

**Expected Result**:

- 6 compiled binaries (with README and LICENSE)
- 6 archive files (.tar.gz or .zip)
- 6 SHA256 checksum files
- All attached to GitHub Release
- Release remains in draft status (manual publication)

#### 2.4 Update Homebrew Formula

- **Runs on**: Ubuntu
- **Waits for**: Publish job to complete
- **Only runs on**: Tagged releases (not test releases)
- **Updates**: Homebrew tap repository with new formula

**What it does**:

1. Extracts SHA256 checksum from Linux build
2. Clones the `homebrew-gitnav` repository
3. Creates/updates the Homebrew formula file:
   - Sets version from git tag
   - Points to release artifacts on GitHub
   - Includes SHA256 checksums for integrity
   - Configures platform-specific URLs (macOS x86_64, ARM64, Linux)
4. Commits and pushes changes to Homebrew tap repository

**Expected Result**:

- Homebrew formula updated automatically
- Users can install with: `brew install msetsma/gitnav/gitnav`
- Formula automatically updated on every release

---

## Workflow Status

### Pull Request Checks

When you create a PR to `main`:

```text
✓ Tests - ubuntu-latest (stable)
✓ Tests - macos-latest (stable)
✓ Tests - windows-latest (stable)
✓ Tests - ubuntu-latest (1.70.0 MSRV)
✓ Clippy Linter
✓ Code Formatting
✓ Build - ubuntu-latest
✓ Build - macos-latest
✓ Build - windows-latest
✓ Security Audit
✓ Code Coverage Report
✓ Documentation
✓ Test Results Summary
```

**PR will merge only if all checks pass** (enforced by branch protection rules)

### Commit to Main/Feature/Bugfix

Same checks run automatically on every push. Scheduled runs occur daily at 1 AM UTC to catch dependency issues.

### Release Process

1. Update version in `Cargo.toml`
2. Create semantic version tag: `git tag 0.2.0` (no `v` prefix!)
3. Push tag: `git push origin 0.2.0`
4. Workflow triggers automatically:
   - Validates version matches Cargo.toml
   - Creates draft GitHub release
   - Builds binaries for all 6 platforms in parallel
   - Generates SHA256 checksums
   - Uploads all artifacts
5. Review release on GitHub and publish when ready

---

## Configuration

### Branch Protection Rules (Recommended)

For `main` branch:

- ✅ Require status checks to pass before merging
- ✅ Require branches to be up to date before merging
- ✅ Dismiss stale PR approvals when new commits are pushed
- ✅ Require code reviews before merging

**Required Checks**:

- `test` (all platforms and MSRV)
- `clippy`
- `fmt`
- `build` (all platforms)
- `security`
- `docs`

### Caching Strategy

**Caches used** to speed up workflow execution:

- Cargo registry
- Cargo git index
- Build artifacts

**Cache keys** include `Cargo.lock` hash to ensure cache validity.

---

## Performance

### Average Execution Times

| Job | Time |
|-----|------|
| Tests (3 platforms) | 2-3 min per platform |
| Tests (MSRV) | 2-3 min |
| Clippy | 1-2 min |
| Formatting | <1 min |
| Build (3 platforms) | 1-2 min per platform |
| Security | <1 min |
| Coverage | 2-3 min |
| Documentation | <1 min |
| **Total (serial)** | **~15-20 min** |
| **Total (parallel)** | **~8-10 min** (most jobs run in parallel) |

**With caching**: Subsequent runs are faster (artifact reuse)

---

## Common Scenarios

### Running Checks Locally

Before pushing, run:

```bash
# Run all tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy --all-targets --all-features

# Build release
cargo build --release
```

### Fixing Workflow Failures

#### Test Failures

```bash
cargo test --verbose
# Fix code causing test failure
git add .
git commit -m "Fix test failure"
git push
```

#### Formatting Issues

```bash
cargo fmt
git add .
git commit -m "Format code"
git push
```

#### Clippy Warnings

```bash
cargo clippy --all-targets --all-features
# Review warnings and fix code
git add .
git commit -m "Fix clippy warnings"
git push
```

---

## Debugging Workflow Issues

### View Workflow Logs

1. Go to **Actions** tab on GitHub
2. Select the failed workflow run
3. Click on the failed job
4. Scroll to see detailed logs

### Re-run Failed Jobs

1. Go to failed workflow run
2. Click **Re-run jobs** button
3. Choose:
   - Re-run all jobs
   - Re-run failed jobs only

### Local Reproducibility

Most workflow steps can be reproduced locally:

```bash
# Test on specific OS (if available locally)
cargo test

# Use Docker to simulate Linux environment
docker run --rm -v $(pwd):/work -w /work rust:latest cargo test

# Check tool versions match workflow
rustc --version
cargo --version
```

---

## GitHub Actions Features Used

### Caching

- Speeds up repeated runs
- Saves CI/CD minutes
- Uses `actions/cache@v3`

### Matrix Strategy

- Tests on multiple platforms
- Tests with multiple Rust versions (if configured)
- Parallelizes execution

### Artifacts & Reports

- Code coverage uploaded to Codecov
- Test results in PR summary
- Job logs available for debugging

### Conditionals

- `if: always()` - Run even if previous jobs fail
- `if: matrix.os == 'ubuntu-latest'` - Platform-specific steps

---

## Maintenance

### Updating Workflows

When updating workflows:

1. Test changes in a feature branch
2. Create PR to review changes
3. Workflow runs on PR to validate
4. Merge to main when validated

### Adding New Jobs

1. Add job to `.github/workflows/tests.yml`
2. Include appropriate caching
3. Set meaningful step names
4. Document in this file
5. Update branch protection rules if needed

### Updating Dependencies

When updating Rust version or tools:

1. Update in workflow file
2. Verify all jobs still pass
3. Document version in this file

---

## Troubleshooting

### Workflow Takes Too Long

**Solutions**:

- Check if cache is being used (should see "Cache hit" messages)
- Reduce number of platforms tested (if acceptable)
- Use `fail-fast: true` to stop on first failure

### Intermittent Failures

**Common causes**:

- Network timeouts downloading dependencies
- Flaky tests
- Platform-specific issues

**Solutions**:

- Re-run failed jobs
- Fix flaky tests with better isolation
- Debug platform-specific issues locally

### Coverage Reports Not Updating

**Check**:

- Codecov integration token
- Repository settings in Codecov
- XML file being generated correctly

---

## Security

### Secrets Management

**No secrets used in current workflows** (can be added for releases if needed):

- Release token (if auto-deploying)
- Deployment credentials
- Third-party service tokens

### Workflow Permissions

```yaml
permissions:
  contents: read        # Read repo contents
  checks: write         # Write check results
  pull-requests: write  # Write PR comments
```

Minimal permissions needed for operation.

---

## Release Workflow Details

### Manual Release Process Benefits

Unlike cargo-dist, the manual release workflow provides:

- **No external tool dependencies** - Uses only standard Rust tooling
- **Full transparency** - See exactly what's happening in each step
- **Easy customization** - Modify build or archive process as needed
- **Reliable versioning** - Semantic versioning enforced
- **Draft releases** - Safe preview before publication

### Release Artifacts

Each release includes:

**For each of 6 platforms**:
- Compiled binary
- README.md and LICENSE files
- tar.gz archive (Unix) or zip archive (Windows)
- SHA256 checksum file

**Total artifacts per release**: 18 files (6 platforms × 3 files)

### Semantic Versioning

This project uses [Semantic Versioning](https://semver.org/):

- **Major.Minor.Patch** format (e.g., `0.1.0`, `1.2.3`)
- **No `v` prefix** on tags
- Tags must match `Cargo.toml` version exactly

### Future Enhancements

If needed in the future:

1. **Installers**
   - Homebrew formula (currently manual)
   - Shell installer script
   - PowerShell installer

2. **Automated Changelog**
   - Parse commit history
   - Generate release notes
   - Include contributors list

3. **Release Signing**
   - GPG sign artifacts
   - Generate signatures for each binary

4. **Additional Platforms**
   - 32-bit Linux/Windows
   - Big-endian architectures
   - RISC-V support

## Future Improvements

### Recommended Additions

1. **Code Coverage Tracking**
   - Set coverage targets
   - Fail if coverage drops below threshold
   - Track coverage trends

2. **Performance Benchmarks**
   - Track build times
   - Monitor test execution times
   - Alert on regressions

3. **Automated Dependency Updates**
   - Use Dependabot
   - Auto-update security patches
   - Create PRs for review

4. **Docker Image Builds**
   - Build Docker images
   - Push to registry
   - Test in containers

---

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI/CD Best Practices](https://rustup.rs/)
- [Cargo Commands](https://doc.rust-lang.org/cargo/)
- [Clippy Lints](https://docs.rs/clippy/)

---

## Contact & Support

For questions about workflows:

1. Check this documentation
2. Review workflow files in `.github/workflows/`
3. Check GitHub Actions logs for specific failures
4. Consult Rust documentation

---

**Last Updated**: November 2, 2025
**Workflow Status**: ✅ Production Ready

---

## Recent Changes

The workflows have been updated to follow industry best practices from projects like ripgrep:

### Tests Workflow Improvements

1. **Scheduled CI Runs**
   - Runs daily at 1 AM UTC
   - Catches issues with dependency updates automatically

2. **MSRV Testing**
   - Tests against Rust 1.70.0
   - Ensures compatibility with older Rust versions
   - Validates minimum supported version

3. **Documentation Checks**
   - New `docs` job validates all doc comments
   - Warnings treated as errors
   - Prevents documentation decay

4. **Expanded Triggers**
   - Now runs on pushes to `main` branch
   - Maintains code quality on main

### Release Workflow Simplification

Switched from cargo-dist to **manual release process** for:

- ✅ **Reliability** - No external tool dependencies
- ✅ **Transparency** - Full visibility into each step
- ✅ **Simplicity** - Uses standard `cargo build`
- ✅ **Control** - Easy to customize build or archive process
- ✅ **Semantic Versioning** - Enforces proper version formats

**Key Features**:
- Version validation against Cargo.toml
- Draft releases prevent premature publication
- Parallel builds for 6 platforms
- SHA256 checksums for integrity
- Uses `gh` CLI for reliability
