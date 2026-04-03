# rs-aos-stats

[![CI](https://github.com/alexis-mignon/rs-aos-stats/actions/workflows/ci.yml/badge.svg)](https://github.com/alexis-mignon/rs-aos-stats/actions/workflows/ci.yml)
[![Deploy](https://github.com/alexis-mignon/rs-aos-stats/actions/workflows/deploy.yml/badge.svg)](https://github.com/alexis-mignon/rs-aos-stats/actions/workflows/deploy.yml)

Computing damage statistics for Warhammer Age Of Sigmar

A Rust library for computing exact damage probabilities in Warhammer Age of Sigmar, with Python bindings, a REST API, and an interactive WebAssembly demo.

## 🎮 Live Demo

**Try the interactive demo**: [https://alexis-mignon.github.io/rs-aos-stats/index.html](https://alexis-mignon.github.io/rs-aos-stats/index.html)

## Features

- **Exact probability calculations**: no Monte Carlo simulation, only precise distributions
- **Complete combat rules**: supports hits, wounds, saves, wards, and special rules
- **Critical hit rules**: auto-wound, mortal wounds, double hits
- **Python bindings**: usable from Python via PyO3
- **REST API (FastAPI)**: HTTP endpoint for computing damage distributions
- **WebAssembly demo**: interactive web app for visualizing damage distributions

## Usage

### Bindings Python

```python
from rs_aos_stats import (
    AttackStats, DefenseStats, CombatConfig, compute_damages,
)

# Configure the stats
attack_stats = AttackStats(attacks=10, to_hit=3, to_wound=3, rend=1, damage=1)
defense_stats = DefenseStats(save=4, ward=None)
config = CombatConfig(attack_stats, defense_stats, None)

# Compute damage
damages = compute_damages(config, "normal")
for damage, probability in damages:
    print(f"{damage} damage: {probability:.2%}")
```

### WebAssembly Demo

An interactive web demo is available in the `web/` directory.

To run it:

```bash
# All in one: build, start the server, and open the browser
make demo

# Or step by step:
make wasm          # Build the WASM module
make serve         # Start the web server
make open          # Open the browser
```

See [web/README.md](web/README.md) for more details.

### API REST

A FastAPI-based REST API is available in the `api/` directory.

To start it:

```bash
# Install API dependencies
.venv/bin/pip install -r api/requirements.txt

# Build the Rust-backed Python module
.venv/bin/maturin develop

# Start the API server
.venv/bin/uvicorn api.main:app --host 0.0.0.0 --port 8001
```

Interactive documentation:
- Swagger UI: `http://localhost:8001/docs`
- ReDoc: `http://localhost:8001/redoc`

See [api/README.md](api/README.md) for the full API specification, validation rules, and `curl` examples.

## Installation

### Python

```bash
# Install with pip (if published to PyPI)
pip install rs-aos-stats

# Or build from source
pip install maturin
maturin develop
```

### WebAssembly

```bash
# Use the Makefile
make demo          # All in one
```

## Project Structure

- `src/probabilities/`: core probability engine
- `src/python/`: Python bindings via PyO3
- `src/wasm/`: WebAssembly bindings via wasm-bindgen
- `api/`: FastAPI REST API and OpenAPI docs
- `web/`: interactive web app
- `examples/`: usage examples, including Jupyter notebooks

## Calculation Engine Overview

Damage computation follows a clear pipeline:

- **Stats → Config**: attack and defense profiles (`AttackStats`, `DefenseStats`, `RollModifier`) are combined into a `CombatConfig`.
- **Rules**: a typed `PipelineBuilder` chains the transitions for a single attack (`Initial` → `Hit` → `Wounded` → `Saved` → `Damaged`) with compile-time ordering guarantees.
- **Attack multiplicity**: `compute_damages` resolves the attacks characteristic outside the typed pipeline and convolves the single-attack distribution into the final result.
- **DP engine**: `src/probabilities/compute_engine.rs` maintains an exact probability distribution over combat states and applies each rule iteratively, aggregating probabilities without simulation or an explicit tree.
- **Result**: the final distribution is marginalized on the `damages` field to produce `(damage, probability)` pairs.
- **Bindings**:
  - `src/python/` exposes `CombatConfig` and `compute_damages`, plus rule wrappers used by Python and the API.
  - `src/wasm/` exposes WASM functions used by `web/app.js` for the interactive demo.

## Development

### Pre-commit Hooks

To enforce code quality, install the pre-commit hooks so checks run automatically before each commit:

```bash
# Install pre-commit
pip install pre-commit

# Install the repository hooks
pre-commit install

# Optional: run on all files
pre-commit run --all-files
```

Configured hooks:
- `cargo fmt`: Rust formatting
- `cargo clippy`: Rust linting
- `cargo test`: test execution
- `ruff`: Python linting
- `mypy`: static type checking for Python
- General hygiene checks: trailing whitespace, end-of-file, YAML, etc.

### Useful Commands

```bash
# Tests
cargo test

# Tests Python
pytest tests/ -v

# Build Python
maturin develop

# Build WASM
make wasm          # Production build
make wasm-dev      # Faster development build

# Full WASM demo
make demo
```

## License

(To be defined)
