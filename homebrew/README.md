# Homebrew Formula

This directory contains the Homebrew formula for gitnav.

## Publishing to Homebrew

### Option 1: Homebrew Tap (Recommended for initial releases)

1. Create a tap repository: `homebrew-gitnav`
2. Place `gitnav.rb` in the tap repository
3. Users install with: `brew install msetsma/gitnav/gitnav`

Steps:
```bash
# Create a new repository named homebrew-gitnav on GitHub
# Clone it locally
git clone https://github.com/msetsma/homebrew-gitnav.git
cd homebrew-gitnav

# Copy the formula
cp /path/to/gitnav/homebrew/gitnav.rb Formula/gitnav.rb

# Commit and push
git add Formula/gitnav.rb
git commit -m "Add gitnav formula"
git push
```

Users can then install with:
```bash
brew install msetsma/gitnav/gitnav
```

### Option 2: Submit to homebrew-core (For established projects)

Once your project is stable and popular, you can submit a PR to [homebrew-core](https://github.com/Homebrew/homebrew-core).

## Updating the Formula

After each release:

1. Update the `version` field
2. Update the URLs to point to the new release
3. Update the SHA256 checksums:
   ```bash
   # Download the release archives
   curl -LO https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-x86_64-apple-darwin.tar.gz
   curl -LO https://github.com/msetsma/gitnav/releases/download/v0.1.0/gitnav-aarch64-apple-darwin.tar.gz

   # Calculate SHA256
   shasum -a 256 gitnav-x86_64-apple-darwin.tar.gz
   shasum -a 256 gitnav-aarch64-apple-darwin.tar.gz
   ```

4. Replace the placeholder SHA256 values in the formula
5. Test the formula:
   ```bash
   brew install --build-from-source ./gitnav.rb
   brew test gitnav
   brew audit --strict gitnav
   ```

## Testing Locally

Before publishing, test the formula locally:

```bash
# Install from local formula
brew install --build-from-source ./homebrew/gitnav.rb

# Test it works
gitnav --version
gn --help

# Run brew tests
brew test gitnav

# Audit the formula
brew audit --strict gitnav

# Uninstall when done testing
brew uninstall gitnav
```
