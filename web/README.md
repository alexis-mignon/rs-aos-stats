# AoS Stats Calculator - WebAssembly Demo

Une application web interactive pour calculer et visualiser les probabilités de dégâts dans Warhammer Age of Sigmar, compilée en WebAssembly pour des performances optimales.

## ✨ Fonctionnalités

- **Calcul de probabilités en temps réel** : Visualisez instantanément les distributions de dégâts
- **Interface interactive** : Sliders pour ajuster facilement toutes les statistiques
- **Règles de critiques multiples** :
  - Hit normale
  - Critique Auto-Wound
  - Critique Mortal Wound
  - Critique Double Hit
- **Graphiques dynamiques** : Visualisation claire avec Chart.js
- **Performance WASM** : Calculs rapides grâce à Rust compilé en WebAssembly

## 🚀 Installation et utilisation

### Prérequis

- Rust et Cargo installés
- `wasm-pack` (sera installé automatiquement par le Makefile si absent)

### Construction et lancement

Depuis la racine du projet :

```bash
# Tout en un: compiler, lancer le serveur et ouvrir le navigateur
make demo

# Ou étape par étape:
make wasm          # Compiler le module WASM (production, optimisé)
make wasm-dev      # Compiler le module WASM (dev, plus rapide)
make serve         # Lancer le serveur web sur http://localhost:8080
make open          # Ouvrir le navigateur
make clean         # Nettoyer les fichiers générés
make help          # Voir toutes les cibles disponibles
```

Alternative manuelle :
```bash
# Compiler le WASM
wasm-pack build --target web --out-dir web/pkg --features wasm

# Lancer le serveur
cd web
python3 -m http.server 8080
```

## 🎮 Guide d'utilisation

### Statistiques d'attaque

- **Attacks** : Nombre d'attaques (1-30)
- **To Hit (X+)** : Seuil de réussite pour toucher (2+ à 6+)
- **To Wound (X+)** : Seuil de réussite pour blesser (2+ à 6+)
- **Rend** : Valeur de perforation d'armure (0-5)
- **Damage** : Dégâts par attaque réussie (1-6)

### Statistiques de défense

- **Save (X+)** : Seuil de réussite de sauvegarde (2+ à 6+)
- **Ward (X+)** : Sauvegarde invulnérable optionnelle (2+ à 7, où 7 = aucune)

### Règles de Hit

- **Normal** : Hit standard, les 6 critiques comptent comme 1 hit
- **Critical Auto-Wound** : Les 6 critiques blessent automatiquement
- **Critical Mortal Wound** : Les 6 critiques infligent des blessures mortelles
- **Critical Double Hit** : Les 6 critiques comptent comme 2 hits

## 📊 Résultats affichés

- **Mean Damage** : Dégâts moyens attendus
- **Max Damage** : Dégâts maximum possibles
- **Distribution de probabilité** : Graphique à barres montrant la probabilité de chaque valeur de dégât

## 🔧 Architecture technique

```
web/
├── index.html      # Interface utilisateur
├── app.js          # Logique JavaScript et interaction WASM
└── pkg/            # Module WASM généré (après build)
    ├── rs_aos_stats.js
    ├── rs_aos_stats_bg.wasm
    └── ...
```

Le module WASM expose principalement :
- `compute_combat_damage()` : Calcule la distribution de dégâts pour une configuration donnée

## 🎨 Personnalisation

L'interface utilise un design moderne avec :
- Gradient violet/bleu
- Responsive design pour mobile et desktop
- Visualisation interactive avec Chart.js
- Feedback en temps réel

Vous pouvez personnaliser les couleurs et le style en modifiant le CSS dans `index.html`.

## 📝 Notes

- Les calculs sont effectués en Rust et compilés en WebAssembly pour des performances optimales
- Tous les calculs sont exacts (pas de simulation Monte Carlo), utilisant des probabilités précises
- La bibliothèque supporte également les jets de dés aléatoires (D3, D6) pour les attaques et dégâts

## 🐛 Dépannage

**Le WASM ne se charge pas** :
- Assurez-vous d'utiliser un serveur HTTP (pas `file://`)
- Vérifiez la console du navigateur pour les erreurs

**Les calculs ne se mettent pas à jour** :
- Rechargez la page
- Vérifiez que tous les fichiers dans `web/pkg/` ont été générés

**Erreur de build** :
- Assurez-vous que `wasm-pack` est installé : `cargo install wasm-pack`
- Vérifiez que la feature `wasm` est activée dans `Cargo.toml`
