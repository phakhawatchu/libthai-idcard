# Contributing to libthai-idcard

Thank you for considering contributing! This document outlines the guidelines we follow.

## Code of Conduct

Please be respectful and constructive in all interactions. This project aims to be a welcoming space for contributors of all backgrounds.

## How to Contribute

### Report Bugs

Open an issue with:
- A clear title and description
- Your environment (OS, Rust version, card reader model)
- Steps to reproduce, if applicable

### Suggest Features

Open an issue describing the feature, its use case, and any relevant implementation ideas.

### Submit Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Make your changes
4. Run the checks locally:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   cargo check --examples
   ```
5. Commit with a clear message
6. Push and open a PR against `main`

## Development Setup

```bash
# Build all targets
make build

# Run linting
make vet

# Format code
make fmt
```

## Project Structure

All Rust source lives under `src/`. Examples for each supported language are in `examples/`. The cross-compilation Docker setup is in `Dockerfile.build`.

## Licensing

By contributing, you agree that your contributions will be licensed under the MIT and Apache-2.0 licenses as specified in the project.
