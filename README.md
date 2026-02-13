# rs-aos-stats

[![CI](https://github.com/YOUR_USERNAME/rs-aos-stats/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/rs-aos-stats/actions/workflows/ci.yml)
[![Deploy](https://github.com/YOUR_USERNAME/rs-aos-stats/actions/workflows/deploy.yml/badge.svg)](https://github.com/YOUR_USERNAME/rs-aos-stats/actions/workflows/deploy.yml)

Computing damage statistics for Warhammer Age Of Sigmar

Une bibliothèque Rust pour calculer les probabilités de dégâts dans Warhammer Age of Sigmar, avec des bindings Python et une démo WebAssembly interactive.

## 🎮 Démo en ligne

**Essayez la démo interactive** : [https://YOUR_USERNAME.github.io/rs-aos-stats/](https://YOUR_USERNAME.github.io/rs-aos-stats/)

> Remplacez `YOUR_USERNAME` par votre nom d'utilisateur GitHub après avoir activé GitHub Pages

## Fonctionnalités

- **Calculs de probabilités exacts** : Pas de simulation Monte Carlo, des probabilités précises
- **Règles de combat complètes** : Support des hits, wounds, saves, wards, et règles spéciales
- **Règles de critiques** : Auto-wound, mortal wounds, double hits
- **Bindings Python** : Utilisable depuis Python avec PyO3
- **Démo WebAssembly** : Application web interactive pour visualiser les distributions de dégâts

## Utilisation

### Bindings Python

```python
from rs_aos_stats import (
    AttackStats, DefenseStats, CombatConfig, compute_damages,
    HitRule, WoundRule, SaveRule, DamagesRule, WardRule
)

# Configurer les stats
attack_stats = AttackStats(attacks=10, to_hit=3, to_wound=3, rend=1, damage=1)
defense_stats = DefenseStats(save=4, ward=None)
config = CombatConfig(attack_stats, defense_stats, None)

# Définir la séquence de règles
sequence = [
    AttackCharacteristicRule(),
    HitRule(),
    WoundRule(),
    SaveRule(),
    DamagesRule(),
]

# Calculer les dégâts
damages = compute_damages(config, sequence)
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
- `web/` : Application web interactive
- `examples/` : Exemples d'utilisation (Jupyter notebooks, scripts Python)

## Développement

```bash
# Tests
cargo test

# Tests Python
cd examples
python test_*.py

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

