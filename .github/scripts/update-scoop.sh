#!/usr/bin/env bash
set -euo pipefail

# Update Scoop Manifest Script
# Updates the gitnav Scoop bucket manifest with the latest release version and checksum.

VERSION="${VERSION:?VERSION environment variable required}"
WINDOWS_X86_SHA="${WINDOWS_X86_SHA:?WINDOWS_X86_SHA environment variable required}"
SCOOP_TOKEN="${SCOOP_TOKEN:?SCOOP_TOKEN environment variable required}"

echo "Updating Scoop manifest for version: $VERSION"
echo "SHA256 Checksums:"
echo "  Windows x86_64: $WINDOWS_X86_SHA"

# Verify the template file exists
if [ ! -f "scoop/gitnav.json" ]; then
  echo "ERROR: Template file scoop/gitnav.json not found"
  exit 1
fi

# Clone the Scoop bucket repository
echo "Cloning scoop-gitnav repository..."
git clone https://${SCOOP_TOKEN}@github.com/msetsma/scoop-gitnav.git /tmp/scoop-gitnav
cd /tmp/scoop-gitnav

mkdir -p bucket

# Copy template and replace placeholders
echo "Creating Scoop manifest from template..."
cp "$GITHUB_WORKSPACE/scoop/gitnav.json" bucket/gitnav.json

sed -i "s|VERSION_PLACEHOLDER|${VERSION}|g" bucket/gitnav.json
sed -i "s|WINDOWS_X86_SHA_PLACEHOLDER|${WINDOWS_X86_SHA}|g" bucket/gitnav.json

echo "Generated manifest:"
cat bucket/gitnav.json

echo "Committing changes..."
git config user.email "github-actions[bot]@users.noreply.github.com"
git config user.name "github-actions[bot]"
git add bucket/gitnav.json

if git commit -m "Update gitnav to ${VERSION}"; then
  echo "Changes committed successfully"
else
  echo "No changes to commit (manifest already up to date)"
fi

if git push; then
  echo "Successfully pushed to scoop-gitnav"
else
  echo "Push failed - manifest may already be up to date"
fi
