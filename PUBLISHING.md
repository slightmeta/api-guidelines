# Publishing Guide for api-guidelines

This guide will help you publish your `api-guidelines` crate to crates.io.

## Prerequisites

1. **Crates.io Account**: You need an account on [crates.io](https://crates.io)
2. **API Token**: Get your API token from your crates.io account settings
3. **Cargo Login**: Run `cargo login` with your API token

## Step-by-Step Publishing Process

### 1. Prepare Your Crate

Make sure your crate is ready for publishing:

```bash
# Build the crate to check for errors
cargo build

# Run tests
cargo test

# Check documentation
cargo doc --open

# Run clippy for code quality
cargo clippy

# Check formatting
cargo fmt -- --check
```

### 2. Verify Package Metadata

Your `Cargo.toml` should include:
- `name` - "api-guidelines" ✓
- `version` - "0.1.0" ✓  
- `description` - Added ✓
- `license` - "MIT" ✓
- `repository` - GitHub URL ✓
- `documentation` - docs.rs URL ✓
- `readme` - "README.md" ✓
- `keywords` - Added ✓
- `categories` - Added ✓

### 3. Login to Crates.io

```bash
# Login with your API token
cargo login
```

You'll be prompted to enter your API token from crates.io.

### 4. Dry Run (Optional but Recommended)

```bash
# Check if your package can be published without actually publishing
cargo publish --dry-run
```

This will validate your package without uploading it.

### 5. Publish the Crate

```bash
# Publish to crates.io
cargo publish
```

### 6. Verify Publication

After publishing:
1. Visit https://crates.io/crates/api-guidelines
2. Check that your documentation appears at https://docs.rs/api-guidelines

## Post-Publishing Tasks

### Version Management

After publishing, you cannot modify or delete published versions. To make changes:

1. **Bug Fixes**: Bump patch version (0.1.0 → 0.1.1)
2. **New Features**: Bump minor version (0.1.0 → 0.2.0)
3. **Breaking Changes**: Bump major version (0.1.0 → 1.0.0)

### Git Tagging

It's good practice to tag releases:

```bash
git tag v0.1.0
git push origin v0.1.0
```

## Common Issues and Solutions

### Name Already Taken
If "api-guidelines" is taken, you'll need to choose a different name in Cargo.toml.

### Missing Metadata
Ensure all required fields in Cargo.toml are filled.

### Documentation Build Failures
Make sure your code compiles and all doc tests pass.

### License File Issues
Ensure LICENSE file exists and matches the license specified in Cargo.toml.

## Best Practices

1. **Semantic Versioning**: Follow semver.org for versioning
2. **Changelog**: Consider adding a CHANGELOG.md file
3. **CI/CD**: Set up GitHub Actions for automated testing and publishing
4. **Documentation**: Keep documentation up to date
5. **Dependencies**: Keep dependencies minimal and up to date

## Next Steps

After successful publication:

1. Share your crate with the Rust community
2. Monitor downloads and issues
3. Consider adding more API guideline categories
4. Add utility functions for working with the guidelines
5. Create examples and tutorials

## Useful Commands

```bash
# Check crate info
cargo package --list

# Update dependencies
cargo update

# Check for outdated dependencies  
cargo outdated

# Generate lockfile (if needed)
cargo generate-lockfile
```

## Support

If you encounter issues:
- Check [Cargo documentation](https://doc.rust-lang.org/cargo/)
- Visit [crates.io help](https://crates.io/help)
