# Release Checklist

Use this checklist when preparing a new release of LogAI.

## Pre-Release (1-2 weeks before)

### Code Preparation
- [ ] All planned features merged to `main`
- [ ] All tests passing (`make check`)
- [ ] No critical bugs in issue tracker
- [ ] Code review completed for all PRs
- [ ] Performance benchmarks run and documented

### Documentation
- [ ] Update CHANGELOG.md with all changes
- [ ] Update version in Cargo.toml
- [ ] Update version in README.md badges
- [ ] Review and update all documentation
- [ ] Update API documentation if needed
- [ ] Add migration guide if breaking changes

### Testing
- [ ] Run full test suite: `cargo test --all-features`
- [ ] Test on all platforms (macOS, Linux, Windows)
- [ ] Test with all AI providers (OpenAI, Claude, Gemini, Ollama)
- [ ] Test installation methods (cargo, homebrew, binary)
- [ ] Manual smoke testing of key features
- [ ] Performance regression testing

## Release Day

### Version Bump
- [ ] Update version in `Cargo.toml`
- [ ] Update version in `CITATION.cff`
- [ ] Update version in documentation
- [ ] Commit version bump: `git commit -m "chore: bump version to X.Y.Z"`

### Create Release
- [ ] Create git tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] Wait for CI to build release artifacts
- [ ] Verify release artifacts on GitHub

### Publish
- [ ] Publish to crates.io: `cargo publish`
- [ ] Verify crates.io listing
- [ ] Update Homebrew formula (if applicable)
- [ ] Update installation scripts

### Announcement
- [ ] Create GitHub release with notes
- [ ] Post announcement in GitHub Discussions
- [ ] Update project website (if applicable)
- [ ] Share on social media (optional)
- [ ] Notify major users/contributors

## Post-Release

### Verification
- [ ] Test installation from crates.io
- [ ] Test installation from GitHub releases
- [ ] Verify documentation links work
- [ ] Monitor for issues in first 24 hours

### Cleanup
- [ ] Close milestone (if used)
- [ ] Update project board
- [ ] Archive old release notes
- [ ] Plan next release

### Communication
- [ ] Respond to release feedback
- [ ] Update roadmap if needed
- [ ] Thank contributors

## Release Types

### Patch Release (X.Y.Z)
- Bug fixes only
- No breaking changes
- No new features
- Quick turnaround (1-2 weeks)

### Minor Release (X.Y.0)
- New features
- Backward compatible
- May include deprecations
- Regular cadence (1-2 months)

### Major Release (X.0.0)
- Breaking changes
- Major new features
- API changes
- Longer cycle (3-6 months)

## Emergency Release

For critical security or bug fixes:

1. Create hotfix branch from release tag
2. Fix issue with minimal changes
3. Fast-track testing
4. Release immediately
5. Backport to main if needed

## Rollback Procedure

If a release has critical issues:

1. Yank from crates.io: `cargo yank --vers X.Y.Z`
2. Mark GitHub release as pre-release
3. Post announcement about issue
4. Prepare hotfix release
5. Document what went wrong

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

## Release Notes Template

```markdown
# LogAI vX.Y.Z

Released: YYYY-MM-DD

## Highlights

- Major feature 1
- Major feature 2
- Important fix

## Added
- New feature descriptions

## Changed
- Modified behavior descriptions

## Fixed
- Bug fix descriptions

## Security
- Security fix descriptions (if any)

## Breaking Changes
- Breaking change descriptions (if any)

## Upgrade Guide
- Steps to upgrade from previous version

## Contributors
- @username1
- @username2

## Full Changelog
https://github.com/ranjan-mohanty/logai/compare/vX.Y.Z-1...vX.Y.Z
```

## Automation

Consider automating:
- Version bumping
- Changelog generation
- Release note creation
- Artifact building
- Publishing to crates.io

## Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Cargo Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)
