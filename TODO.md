# Release and Distribution TODO

## Required Before First Release

### 1. Create GitHub Release
- [ ] Tag the release: `git tag v0.1.0 && git push origin v0.1.0`
- [ ] Wait for GitHub Actions workflow to complete
- [ ] Verify all platform binaries are uploaded to the release

### 2. Update Homebrew Formula SHA256s
After the release is created and binaries are uploaded:

- [ ] Download macOS x86_64 binary and calculate SHA256:
  ```bash
  curl -LO https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-x86_64-apple-darwin.tar.gz
  shasum -a 256 gitnav-x86_64-apple-darwin.tar.gz
  ```
- [ ] Download macOS ARM64 binary and calculate SHA256:
  ```bash
  curl -LO https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-aarch64-apple-darwin.tar.gz
  shasum -a 256 gitnav-aarch64-apple-darwin.tar.gz
  ```
- [ ] Download Linux x86_64 binary and calculate SHA256:
  ```bash
  curl -LO https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-x86_64-unknown-linux-gnu.tar.gz
  shasum -a 256 gitnav-x86_64-unknown-linux-gnu.tar.gz
  ```
- [ ] Download Linux ARM64 binary and calculate SHA256:
  ```bash
  curl -LO https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-aarch64-unknown-linux-gnu.tar.gz
  shasum -a 256 gitnav-aarch64-unknown-linux-gnu.tar.gz
  ```
- [ ] Update `homebrew/gitnav.rb` with all four SHA256 values (replace `REPLACE_WITH_ACTUAL_SHA256_*`)

### 3. Create Homebrew Tap Repository
- [ ] Create a new repository on GitHub named `homebrew-gitnav`
- [ ] Clone it locally:
  ```bash
  git clone https://github.com/msetsma/homebrew-gitnav.git
  cd homebrew-gitnav
  mkdir -p Formula
  ```
- [ ] Copy the updated formula:
  ```bash
  cp /path/to/gitnav/homebrew/gitnav.rb Formula/gitnav.rb
  ```
- [ ] Commit and push:
  ```bash
  git add Formula/gitnav.rb
  git commit -m "Add gitnav v0.1.0 formula"
  git push
  ```

### 4. Test Homebrew Installation
- [ ] Test the formula locally before publishing:
  ```bash
  brew install --build-from-source ./homebrew/gitnav.rb
  gitnav --version
  brew test gitnav
  brew audit --strict gitnav
  brew uninstall gitnav
  ```
- [ ] Test installation from tap:
  ```bash
  brew install msetsma/gitnav/gitnav
  gitnav --version
  gn --help
  ```

### 5. Publish to crates.io (Optional)
- [ ] Ensure you're logged in to crates.io: `cargo login`
- [ ] Publish: `cargo publish`

## For Future Releases

### Version Update Checklist
- [ ] Update version in `Cargo.toml`
- [ ] Update version in `homebrew/gitnav.rb`
- [ ] Update CHANGELOG.md
- [ ] Commit changes: `git commit -am "Bump version to vX.Y.Z"`
- [ ] Create and push tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
- [ ] Wait for GitHub Actions to complete
- [ ] Download new binaries and calculate SHA256s
- [ ] Update `homebrew/gitnav.rb` with new version and SHA256s
- [ ] Update Homebrew tap repository
- [ ] Test installation
- [ ] Publish to crates.io if needed

## Additional Package Managers (Future)

### Scoop (Windows)
- [ ] Create scoop bucket
- [ ] Add manifest for gitnav

### APT/DEB Repository (Linux)
- [ ] Set up Debian package building
- [ ] Create APT repository

### AUR (Arch Linux)
- [ ] Create PKGBUILD
- [ ] Submit to AUR

### Cargo-binstall Support
- [ ] Verify cargo-binstall can install from GitHub releases
