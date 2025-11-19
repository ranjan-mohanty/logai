# Release Process

This document describes how to create a new release of logai.

## Prerequisites

1. **GitHub Secrets**: Ensure these secrets are configured in your repository:
   - `CARGO_TOKEN`: Your crates.io API token (get from <https://crates.io/me>)
   - `GITHUB_TOKEN`: Automatically provided by GitHub Actions

2. **Permissions**: You need write access to the repository

## Release Steps

### 1. Update Version

Update the version in `Cargo.toml`:

For stable releases:

```toml
version = "0.2.0"
```

For beta/pre-releases:

```toml
version = "0.2.0-beta.1"
```

### 2. Update CHANGELOG

Add release notes to `CHANGELOG.md` with the new version and changes.

### 3. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to 0.2.0"
git push
```

### 4. Create and Push Tag

```bash
git tag v0.2.0
git push origin v0.2.0
```

### 5. Create GitHub Release

Go to GitHub → Releases → Draft a new release:

- Choose the tag you just pushed
- For pre-releases (beta, rc, alpha), check "Set as a pre-release"
- Add release notes (can copy from CHANGELOG)
- Click "Publish release"

### 6. Automated Process

Once the release is published, GitHub Actions will automatically:

1. **Build binaries** for all platforms (Linux, macOS, Windows - x86_64 and
   aarch64)
2. **Create GitHub release** with binaries attached
3. **Publish to crates.io** using your CARGO_TOKEN
4. **Update Homebrew formula** with new version and SHA256 checksums

## What Happens Automatically

### Build Release Job

- Compiles for 5 targets: Linux (x86_64, aarch64), macOS (x86_64, aarch64),
  Windows (x86_64)
- Strips binaries and creates archives (.tar.gz for Unix, .zip for Windows)
- Uploads artifacts

### Create Release Job

- Downloads all build artifacts
- Creates GitHub release with auto-generated release notes
- Attaches all binary archives

### Publish Crates Job

- Publishes to crates.io using `cargo publish`
- Users can install with: `cargo install logai`

### Update Homebrew Job

- Calculates SHA256 checksums for all archives
- Updates `scripts/homebrew/logai.rb` with new version and checksums
- Commits and pushes the updated formula

## Manual Steps After Release

### 1. Verify crates.io Publication

Visit <https://crates.io/crates/logai> to confirm the new version is live.

### 2. Test Homebrew Installation

Users can install directly from the main repo:

```bash
brew install https://raw.githubusercontent.com/ranjan-mohanty/logai/main/scripts/homebrew/logai.rb
```

(Optional) Create a tap repository for shorter install command:

- Create repo named `homebrew-logai`
- Copy `scripts/homebrew/logai.rb` to `Formula/logai.rb`
- Users can then: `brew tap ranjan-mohanty/logai && brew install logai`

### 3. Test Installation

Test all installation methods:

```bash
# Cargo
cargo install logai

# Homebrew
brew install https://raw.githubusercontent.com/ranjan-mohanty/logai/main/scripts/homebrew/logai.rb

# Direct download
# Download from GitHub releases and test
```

## Troubleshooting

### crates.io Publication Fails

- Check that CARGO_TOKEN secret is set correctly
- Verify you have publish permissions for the crate
- Check crates.io for any policy violations

### Homebrew Formula Update Fails

- Check GitHub Actions logs for sed errors
- Verify artifact names match expected patterns
- Manually update formula if needed

### Build Failures

- Check Rust version compatibility
- Verify all dependencies are available
- Review GitHub Actions logs for specific errors

## Rollback

If you need to rollback a release:

1. Delete the tag: `git tag -d v0.2.0 && git push origin :refs/tags/v0.2.0`
2. Delete the GitHub release
3. Yank from crates.io: `cargo yank --vers 0.2.0`
