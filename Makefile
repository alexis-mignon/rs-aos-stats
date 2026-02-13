.PHONY: help wasm wasm-dev serve open demo clean

# Default target
help:
	@echo "Available targets:"
	@echo "  make wasm      - Build WASM module for production"
	@echo "  make wasm-dev  - Build WASM module for development (faster)"
	@echo "  make serve     - Start HTTP server on port 8888"
	@echo "  make open      - Open browser to http://localhost:8888"
	@echo "  make demo      - Build WASM, start server, and open browser"
	@echo "  make clean     - Remove generated WASM files"

# Check if wasm-pack is installed
check-wasm-pack:
	@which wasm-pack > /dev/null || (echo "Installing wasm-pack..." && cargo install wasm-pack)

# Build WASM for production (optimized)
wasm: check-wasm-pack
	@echo "🚀 Building WASM module (production)..."
	wasm-pack build --target web --out-dir web/pkg --no-default-features --features wasm
	@echo "✅ Build complete!"

# Build WASM for development (faster builds)
wasm-dev: check-wasm-pack
	@echo "🔧 Building WASM module (development)..."
	wasm-pack build --target web --out-dir web/pkg --no-default-features --features wasm --dev
	@echo "✅ Build complete!"

# Start HTTP server
serve:
	@echo "🌐 Starting HTTP server on http://localhost:8888"
	@cd web && python3 -m http.server 8888

# Open browser
open:
	@echo "🌍 Opening browser..."
	@which xdg-open > /dev/null && xdg-open http://localhost:8888 || \
	 which open > /dev/null && open http://localhost:8888 || \
	 echo "Please open http://localhost:8888 in your browser"

# Full demo: build, serve, and open
demo: wasm
	@echo "🎮 Launching demo..."
	@(sleep 2 && $(MAKE) open) & $(MAKE) serve

# Clean generated files
clean:
	@echo "🧹 Cleaning generated files..."
	@rm -rf web/pkg
	@echo "✅ Clean complete!"
