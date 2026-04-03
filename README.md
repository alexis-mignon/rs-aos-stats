# rs-aos-stats

[![CI](https://github.com/alexis-mignon/rs-aos-stats/actions/workflows/ci.yml/badge.svg)](https://github.com/alexis-mignon/rs-aos-stats/actions/workflows/ci.yml)
[![Deploy](https://github.com/alexis-mignon/rs-aos-stats/actions/workflows/deploy.yml/badge.svg)](https://github.com/alexis-mignon/rs-aos-stats/actions/workflows/deploy.yml)

Computing damage statistics for Warhammer Age Of Sigmar

Une bibliothèque Rust pour calculer les probabilités de dégâts dans Warhammer Age of Sigmar, avec des bindings Python et une démo WebAssembly interactive.

## 🎮 Démo en ligne

**Essayez la démo interactive** : [https://alexis-mignon.github.io/rs-aos-stats/](https://alexis-mignon.github.io/rs-aos-stats/)

## Fonctionnalités

- **Calculs de probabilités exacts** : Pas de simulation Monte Carlo, des probabilités précises
- **Règles de combat complètes** : Support des hits, wounds, saves, wards, et règles spéciales
- **Règles de critiques** : Auto-wound, mortal wounds, double hits
- **Bindings Python** : Utilisable depuis Python avec PyO3
- **API REST (FastAPI)** : Endpoint HTTP pour calculer les distributions de dégâts
- **Démo WebAssembly** : Application web interactive pour visualiser les distributions de dégâts

## Utilisation

### Bindings Python

```python
from rs_aos_stats import (
    AttackStats, DefenseStats, CombatConfig, compute_damages,
)

# Configurer les stats
attack_stats = AttackStats(attacks=10, to_hit=3, to_wound=3, rend=1, damage=1)
defense_stats = DefenseStats(save=4, ward=None)
config = CombatConfig(attack_stats, defense_stats, None)

# Calculer les dégâts
damages = compute_damages(config, "normal")
for damage, probability in damages:
    print(f"{damage} dégâts: {probability:.2%}")
```

### Démo WebAssembly

Une démo web interactive est disponible dans le dossier `web/`.

Pour l'essayer :

```bash
# Tout en un: compiler, lancer le serveur et ouvrir le navigateur
make demo

# Ou étape par étape:
make wasm          # Compiler le module WASM
make serve         # Lancer le serveur web
make open          # Ouvrir le navigateur
```

Voir [web/README.md](web/README.md) pour plus de détails.

### API REST

Une API FastAPI est disponible dans le dossier `api/`.

Pour la lancer :

```bash
# Installer les dépendances API
.venv/bin/pip install -r api/requirements.txt

# Construire le module Python Rust
.venv/bin/maturin develop

# Lancer le serveur API
.venv/bin/uvicorn api.main:app --host 0.0.0.0 --port 8001
```

Documentation interactive :
- Swagger UI : `http://localhost:8001/docs`
- ReDoc : `http://localhost:8001/redoc`

Voir [api/README.md](api/README.md) pour la spécification complète de l'API (schémas, validations, exemples `curl`).

## Installation

### Pour Python

```bash
# Installer avec pip (si publié sur PyPI)
pip install rs-aos-stats

# Ou compiler depuis les sources
pip install maturin
maturin develop
```

### Pour WebAssembly

```bash
# Utiliser le Makefile
make demo          # Tout en un
```

## Structure du projet

- `src/probabilities/` : Logique de calcul des probabilités (core)
- `src/python/` : Bindings Python avec PyO3
- `src/wasm/` : Bindings WebAssembly avec wasm-bindgen
- `api/` : API REST FastAPI et documentation OpenAPI
- `web/` : Application web interactive
- `examples/` : Exemples d'utilisation (Jupyter notebooks, scripts Python)

## Vue d'ensemble du moteur de calcul

Le calcul des dégâts suit un pipeline clair :

- **Stats → Config** : les profils d'attaque/défense (AttackStats, DefenseStats, RollModifier) sont combinés dans un `CombatConfig`.
- **Règles** : un `PipelineBuilder` typé enchaîne les règles de transition d'une attaque individuelle (`Initial` → `Hit` → `Wounded` → `Saved` → `Damaged`), avec vérification à la compilation de l’ordre correct.
- **Multiplicité des attaques** : `compute_damages` résout ensuite la caractéristique d'attaques en dehors du pipeline typé et convolue la distribution mono-attaque pour obtenir la distribution finale.
- **Moteur DP** : le module `src/probabilities/compute_engine.rs` maintient une distribution exacte sur les états de combat et applique chaque règle de manière itérative en agrégeant les probabilités (pas de simulation, pas d’arbre explicite).
- **Résultat** : la distribution finale est marginalisée sur le champ `damages` pour produire une distribution `(dégâts, probabilité)`.
- **Bindings** :
    - `src/python/` expose `CombatConfig` + `compute_damages` et des wrappers de règles pour Python / API.
    - `src/wasm/` expose des fonctions WASM utilisées par `web/app.js` pour la démo interactive.

## Développement

### Configuration des pre-commit hooks

Pour assurer la qualité du code, installez les hooks de pre-commit qui exécuteront automatiquement les vérifications avant chaque commit:

```bash
# Installer pre-commit
pip install pre-commit

# Installer les hooks dans le repo
pre-commit install

# (Optionnel) Exécuter sur tous les fichiers
pre-commit run --all-files
```

Les hooks configurés:
- `cargo fmt` : Formatage du code Rust
- `cargo clippy` : Linting du code Rust
- `cargo test` : Exécution des tests
- `ruff` : Linting Python (erreurs de code et problèmes potentiels)
- `mypy` : Vérification statique des types Python
- Vérifications générales : trailing whitespace, end-of-file, YAML, etc.

### Commandes utiles

```bash
# Tests
cargo test

# Tests Python
pytest tests/ -v

# Build Python
maturin develop

# Build WASM
make wasm          # Production (optimisé)
make wasm-dev      # Development (rapide)

# Demo WASM complète
make demo
```

## Licence

(À définir)
