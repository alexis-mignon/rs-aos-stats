# GitHub Actions Workflows

This directory contains the CI/CD workflows for the rs-aos-stats project.

## 📋 Available Workflows

### 1. CI (`ci.yml`)

Runs on every push and pull request targeting `main`.

**Jobs:**
- **test-rust**: Rust unit tests
- **check-rust**: formatting checks (`rustfmt`) and linting (`clippy`)
- **build-wasm**: WebAssembly build verification
- **test-python**: Python binding tests on Linux, macOS, and Windows with Python 3.9-3.13
- **python-quality**: Python quality checks (`ruff` + `mypy`)

### 2. Deploy (`deploy.yml`)

Automatically deploys the WASM demo to GitHub Pages on every push to `main`.

**Required configuration:**

1. Go to `Settings > Pages` in your GitHub repository
2. Set the source to `GitHub Actions`
3. The demo will be available at `https://<username>.github.io/<repo>/`

### 3. Release (`release.yml`)

Creates automated releases when a tag is pushed, for example `v0.1.0`.

**Jobs:**
- Build Python wheels for Linux, macOS, and Windows (Python 3.9-3.13)
- Build the WASM archive
- Create the GitHub release automatically with all assets
- Optionally publish to PyPI

**Required configuration for PyPI:**

1. Create a PyPI API token: https://pypi.org/manage/account/token/
2. Add the secret in GitHub:
   - Go to `Settings > Secrets and variables > Actions`
   - Create a new secret named `PYPI_API_TOKEN`
   - Set its value to your PyPI token

**Create a release:**

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release will be created automatically with:
- auto-generated release notes
- Python wheels for all platforms
- the complete WASM archive

## 🔧 Status Badges

Add these badges to your README.md:

```markdown
[![CI](https://github.com/<username>/<repo>/actions/workflows/ci.yml/badge.svg)](https://github.com/<username>/<repo>/actions/workflows/ci.yml)
[![Deploy](https://github.com/<username>/<repo>/actions/workflows/deploy.yml/badge.svg)](https://github.com/<username>/<repo>/actions/workflows/deploy.yml)
```

## 🚀 Manual Actions

Some workflows can be triggered manually from the GitHub "Actions" tab, including `deploy.yml`.

## 📊 Cache

The workflows use GitHub Actions caching to speed up builds:
- Cargo registry and index cache
- Rust `target` directory cache
- `wasm-pack` cache

## ⚠️ Important Notes

1. **Python tests**: the `test-python` job runs the Python tests in `tests/` with pytest

2. **GitHub Pages**: make sure GitHub Pages is enabled in your repository settings

3. **PyPI**: the release workflow will publish to PyPI only if `PYPI_API_TOKEN` is configured

4. **Build time**: the first builds can be slow (10-15 min), but cached follow-up builds are typically much faster (2-5 min)
