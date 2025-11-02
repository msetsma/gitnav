#!/usr/bin/env bash
set -euo pipefail

# Update Homebrew Formula Script
# This script updates the gitnav Homebrew formula with the latest release.
# It uses the template in homebrew/gitnav.rb as the source, replacing version
# and SHA256 checksums with actual values from the release.

VERSION="${VERSION:?VERSION environment variable required}"
LINUX_GNU_SHA="${LINUX_GNU_SHA:?LINUX_GNU_SHA environment variable required}"
LINUX_ARM_SHA="${LINUX_ARM_SHA:?LINUX_ARM_SHA environment variable required}"
MACOS_X86_SHA="${MACOS_X86_SHA:?MACOS_X86_SHA environment variable required}"
MACOS_ARM_SHA="${MACOS_ARM_SHA:?MACOS_ARM_SHA environment variable required}"
HOMEBREW_TOKEN="${HOMEBREW_TOKEN:?HOMEBREW_TOKEN environment variable required}"

echo "Updating Homebrew formula for version: $VERSION"
echo "SHA256 Checksums:"
echo "  Linux GNU:   $LINUX_GNU_SHA"
echo "  Linux ARM:   $LINUX_ARM_SHA"
echo "  macOS x86:   $MACOS_X86_SHA"
echo "  macOS ARM:   $MACOS_ARM_SHA"

# Verify the template file exists
if [ ! -f "homebrew/gitnav.rb" ]; then
  echo "❌ ERROR: Template file homebrew/gitnav.rb not found"
  exit 1
fi

# Clone the Homebrew tap repository
echo "Cloning homebrew-gitnav repository..."
git clone https://${HOMEBREW_TOKEN}@github.com/msetsma/homebrew-gitnav.git /tmp/homebrew-gitnav
cd /tmp/homebrew-gitnav

# Create Formula directory
mkdir -p Formula

# Copy the template and update it
echo "Creating Homebrew formula from template..."
cp "$GITHUB_WORKSPACE/homebrew/gitnav.rb" Formula/gitnav.rb

# Replace version placeholder
sed -i "s|VERSION_PLACEHOLDER|${VERSION}|g" Formula/gitnav.rb

# Replace SHA256 placeholders
sed -i "s|LINUX_GNU_SHA_PLACEHOLDER|${LINUX_GNU_SHA}|g" Formula/gitnav.rb
sed -i "s|LINUX_ARM_SHA_PLACEHOLDER|${LINUX_ARM_SHA}|g" Formula/gitnav.rb
sed -i "s|MACOS_X86_SHA_PLACEHOLDER|${MACOS_X86_SHA}|g" Formula/gitnav.rb
sed -i "s|MACOS_ARM_SHA_PLACEHOLDER|${MACOS_ARM_SHA}|g" Formula/gitnav.rb

# Show the generated formula
echo "Generated formula:"
cat Formula/gitnav.rb

# Configure git and commit
echo "Committing changes..."
git config user.email "github-actions[bot]@users.noreply.github.com"
git config user.name "github-actions[bot]"
git add Formula/gitnav.rb

if git commit -m "Update gitnav to ${VERSION}"; then
  echo "✅ Changes committed successfully"
else
  echo "⚠️ No changes to commit (formula already up to date)"
fi

# Push changes
if git push; then
  echo "✅ Successfully pushed to homebrew-gitnav"
else
  echo "⚠️ Push failed - formula may already be up to date"
fi
