# AoS Stats Calculator - WebAssembly Demo

An interactive web application for computing and visualizing Warhammer Age of Sigmar damage probabilities, compiled to WebAssembly for performance.

## ✨ Features

- **Real-time probability calculation**: instantly visualize damage distributions
- **Interactive UI**: sliders make it easy to adjust all stats
- **Multiple critical hit rules**:
  - Normal hit
  - Critical auto-wound
  - Critical mortal wound
  - Critical double hit
- **Dynamic charts**: clear visualization with Chart.js
- **WASM performance**: fast calculations powered by Rust compiled to WebAssembly

## 🚀 Installation and Usage

### Prerequisites

- Rust and Cargo installed
- `wasm-pack` (installed automatically by the Makefile if missing)

### Build and Run

From the project root:

```bash
# All in one: build, start the server, and open the browser
make demo

# Or step by step:
make wasm          # Build the WASM module (production, optimized)
make wasm-dev      # Build the WASM module (development, faster)
make serve         # Start the web server on http://localhost:8080
make open          # Open the browser
make clean         # Clean generated files
make help          # Show all available targets
```

Manual alternative:
```bash
# Build WASM
wasm-pack build --target web --out-dir web/pkg --features wasm

# Start the server
cd web
python3 -m http.server 8080
```

## 🎮 Usage Guide

### Attack Stats

- **Attacks**: number of attacks (1-30)
- **To Hit (X+)**: hit threshold (2+ to 6+)
- **To Wound (X+)**: wound threshold (2+ to 6+)
- **Rend**: armor penetration value (0-5)
- **Damage**: damage per successful attack (1-6)

### Defense Stats

- **Save (X+)**: save threshold (2+ to 6+)
- **Ward (X+)**: optional ward save (2+ to 7, where 7 means none)

### Hit Rules

- **Normal**: standard hit, critical 6s count as 1 hit
- **Critical Auto-Wound**: critical 6s automatically wound
- **Critical Mortal Wound**: critical 6s inflict mortal wounds
- **Critical Double Hit**: critical 6s count as 2 hits

## 📊 Displayed Results

- **Mean Damage**: expected damage
- **Max Damage**: maximum possible damage
- **Probability distribution**: bar chart showing the probability for each damage value

## 🔧 Technical Architecture

```
web/
├── index.html      # User interface
├── app.js          # JavaScript logic and WASM integration
└── pkg/            # Generated WASM module (after build)
    ├── rs_aos_stats.js
    ├── rs_aos_stats_bg.wasm
    └── ...
```

The WASM module primarily exposes:
- `compute_combat_damage()`: computes the damage distribution for a given configuration

## 🎨 Customization

The interface uses a modern design with:
- burgundy gradient styling
- responsive layout for desktop and mobile
- interactive Chart.js visualizations
- real-time feedback

You can customize colors and styling by editing the CSS in `index.html`.

## 📝 Notes

- Calculations are performed in Rust and compiled to WebAssembly for performance
- All calculations are exact; there is no Monte Carlo simulation
- The library also supports random dice characteristics (D3, D6) for attacks and damage

## 🐛 Troubleshooting

**WASM does not load**:
- Make sure you are using an HTTP server, not `file://`
- Check the browser console for errors

**Calculations do not update**:
- Reload the page
- Verify that all files in `web/pkg/` were generated

**Build error**:
- Make sure `wasm-pack` is installed: `cargo install wasm-pack`
- Verify that the `wasm` feature is enabled in `Cargo.toml`
