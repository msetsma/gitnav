# GitHub Actions Workflows Documentation

**Project**: gitnav
**Last Updated**: November 1, 2025
**Maintainer**: Development Team

---

## Overview

This project uses GitHub Actions to automate testing, code quality checks, and release builds. All workflows are defined in `.github/workflows/`.

---

## Workflows

### 1. **tests.yml** - Continuous Integration (Pull Requests & Commits)

**Trigger**:

- Push to `main`, `develop`, or `feature/**` branches
- Pull requests to `main` or `develop`

**Jobs**:

#### 1.1 Test Job (Multi-Platform)

- **Runs on**: Ubuntu, macOS, Windows
- **Rust Version**: Stable
- **Tests**:
  - ✅ Debug build tests
  - ✅ Release build tests
  - ✅ Doc comment tests
  - ✅ All 106 unit tests

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

#### 1.7 Test Results Summary

- **Aggregates** results from all jobs
- **Reports** status in job summary
- **Available** in PR checks

---

### 2. **release.yml** - Release Build & Distribution

**Trigger**:

- Push of tags matching `v*` (e.g., `v0.2.0`)
- Manual workflow dispatch

**Jobs**:

#### 2.1 Create Release

- Creates GitHub release
- Extracts version from tag

#### 2.2 Build Release (Multi-Platform)

- **Platforms**:
  - Linux x86_64 (GNU)
  - Linux x86_64 (musl)
  - Linux ARM64
  - macOS x86_64
  - macOS ARM64 (Apple Silicon)
  - Windows x86_64

**What it does**:

- Builds optimized release binary
- Strips symbols (Unix)
- Creates archives (.tar.gz / .zip)
- Generates SHA256 checksums
- Uploads to GitHub Release

**Expected Result**: Binary available for all platforms

---

## Workflow Status

### Pull Request Checks

When you create a PR to `main` or `develop`:

``` text
✓ Tests - ubuntu-latest
✓ Tests - macos-latest
✓ Tests - windows-latest
✓ Clippy Linter
✓ Code Formatting
✓ Build - ubuntu-latest
✓ Build - macos-latest
✓ Build - windows-latest
✓ Security Audit
✓ Code Coverage Report
✓ Test Results Summary
```

**PR will merge only if all checks pass** (enforced by branch protection rules)

### Commit to Main/Develop

Same checks run automatically on every push.

### Release Process

1. Create a tag: `git tag v0.2.0`
2. Push tag: `git push origin v0.2.0`
3. Workflow triggers automatically
4. Creates release with binaries for all platforms

---

## Configuration

### Branch Protection Rules (Recommended)

For `main` branch:

- ✅ Require status checks to pass before merging
- ✅ Require branches to be up to date before merging
- ✅ Dismiss stale PR approvals when new commits are pushed
- ✅ Require code reviews before merging

**Required Checks**:

- `test` (all platforms)
- `clippy`
- `fmt`
- `build` (all platforms)
- `security`

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
| Tests | 2-3 min per platform |
| Clippy | 1-2 min |
| Formatting | <1 min |
| Build | 1-2 min per platform |
| Security | <1 min |
| Coverage | 2-3 min |
| **Total** | **~10-15 min** |

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

4. **Automated Releases**
   - Auto-tag versions
   - Generate changelogs
   - Create releases automatically

5. **Docker Image Builds**
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

**Last Updated**: November 1, 2025
**Workflow Status**: ✅ Production Ready
