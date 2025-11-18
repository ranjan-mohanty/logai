# Homebrew Installation

## Install from GitHub Releases

You can install `logai` directly from the formula in this repository:

```bash
brew install --formula scripts/homebrew/logai.rb
```

## Create a Homebrew Tap (Recommended)

For easier installation and updates, create a Homebrew tap:

1. Create a new repository named `homebrew-logai` on GitHub
2. Copy `logai.rb` to the root of that repository
3. Users can then install with:

```bash
brew tap ranjan-mohanty/logai
brew install logai
```

## Update Formula

The formula is automatically updated by GitHub Actions when a new release is created. The workflow:

1. Builds binaries for all platforms
2. Calculates SHA256 checksums
3. Updates the formula with new version and checksums
4. Commits and pushes the changes

## Manual Update

If you need to manually update the formula:

1. Download the release artifacts
2. Calculate SHA256 checksums:
   ```bash
   sha256sum logai-*.tar.gz
   ```
3. Update version and checksums in `logai.rb`
4. Test the formula:
   ```bash
   brew install --build-from-source --formula logai.rb
   brew test logai
   ```

## Testing

Test the formula locally:

```bash
brew install --build-from-source --formula scripts/homebrew/logai.rb
brew test logai
logai --version
```
