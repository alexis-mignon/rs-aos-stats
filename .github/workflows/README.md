# GitHub Actions Workflows

Ce dossier contient les workflows CI/CD pour le projet rs-aos-stats.

## 📋 Workflows disponibles

### 1. CI (`ci.yml`)

Exécuté sur chaque push et pull request vers `main`.

**Jobs :**
- **test-rust** : Tests unitaires Rust
- **check-rust** : Vérification du formatage (rustfmt) et linting (clippy)
- **build-wasm** : Compilation WebAssembly pour vérifier que ça build
- **test-python** : Tests des bindings Python sur Linux, macOS, Windows avec Python 3.9-3.12

### 2. Deploy (`deploy.yml`)

Déploie automatiquement la démo WASM sur GitHub Pages à chaque push sur `main`.

**Configuration nécessaire :**

1. Aller dans `Settings > Pages` de votre repo GitHub
2. Source : `GitHub Actions`
3. La démo sera disponible à : `https://<username>.github.io/<repo>/`

### 3. Release (`release.yml`)

Crée des releases automatiques lors de la création d'un tag (ex: `v0.1.0`).

**Jobs :**
- Build des wheels Python pour Linux, macOS, Windows (Python 3.9-3.12)
- Build de l'archive WASM
- Création automatique de la release GitHub avec tous les assets
- Publication optionnelle sur PyPI

**Configuration nécessaire pour PyPI :**

1. Créer un token API sur PyPI : https://pypi.org/manage/account/token/
2. Ajouter le secret dans GitHub :
   - Aller dans `Settings > Secrets and variables > Actions`
   - Créer un nouveau secret : `PYPI_API_TOKEN`
   - Valeur : votre token PyPI

**Créer une release :**

```bash
git tag v0.1.0
git push origin v0.1.0
```

La release sera créée automatiquement avec :
- Notes de version auto-générées
- Wheels Python pour toutes les plateformes
- Archive WASM complète

## 🔧 Badge de statut

Ajoutez ces badges dans votre README.md :

```markdown
[![CI](https://github.com/<username>/<repo>/actions/workflows/ci.yml/badge.svg)](https://github.com/<username>/<repo>/actions/workflows/ci.yml)
[![Deploy](https://github.com/<username>/<repo>/actions/workflows/deploy.yml/badge.svg)](https://github.com/<username>/<repo>/actions/workflows/deploy.yml)
```

## 🚀 Actions manuelles

Les workflows peuvent être déclenchés manuellement via l'onglet "Actions" sur GitHub (pour `deploy.yml`).

## 📊 Cache

Les workflows utilisent le cache GitHub Actions pour accélérer les builds :
- Cache Cargo registry et index
- Cache du dossier `target` de Rust
- Cache de wasm-pack

## ⚠️  Notes importantes

1. **Test Python** : Le job `test-python` exécute les tests Python du dossier `tests/` avec pytest

2. **GitHub Pages** : Assurez-vous que GitHub Pages est activé dans les paramètres de votre repo

3. **PyPI** : Le workflow de release ne publiera sur PyPI que si le secret `PYPI_API_TOKEN` est configuré

4. **Temps de build** : Les premiers builds peuvent être lents (10-15 min), mais grâce au cache les suivants seront plus rapides (2-5 min)
