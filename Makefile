.PHONY: all build-all clean build-example package-example help

# Get all example names from examples directory
EXAMPLES := $(patsubst examples/%.rs,%,$(wildcard examples/*.rs))
OUT_DIR := ./out
BUILD_DIR := ./build
WASM_TARGET := wasm32-unknown-unknown
RUSTFLAGS := --cfg=web_sys_unstable_apis
WEBGL2_FEATURES := webgl2
WEBGPU_FEATURES := webgpu

help:
	@echo "Available targets:"
	@echo "  make all          - Build and package all examples (WebGL2 and WebGPU)"
	@echo "  make build-all    - Build WASM for all examples (WebGL2 and WebGPU)"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make build-<name> - Build specific example (e.g., make build-simple)"
	@echo ""
	@echo "Available examples: $(EXAMPLES)"

all: $(addprefix package-,$(EXAMPLES))

build-all: $(addprefix build-,$(EXAMPLES))

# Pattern rule for building individual examples (both WebGL2 and WebGPU)
build-%: build-%-webgl2 build-%-webgpu
	@true

# Build WebGL2 version
build-%-webgl2:
	@echo "=========================================="
	@echo "Build $* WebGL2"
	@echo "=========================================="
	RUSTFLAGS="$(RUSTFLAGS)" cargo build --release --example $* --target $(WASM_TARGET) --no-default-features --features $(WEBGL2_FEATURES)
	
	@echo "=========================================="
	@echo "wasm-bindgen $* WebGL2"
	@echo "=========================================="
	mkdir -p $(OUT_DIR)
	wasm-bindgen --out-name $*_webgl2 --out-dir $(OUT_DIR) --no-typescript --target web \
		./target/$(WASM_TARGET)/release/examples/$*.wasm
	
	@echo "=========================================="
	@echo "Optimize WASM $* WebGL2"
	@echo "=========================================="
	wasm-opt --strip-debug --vacuum -Oz -o $(OUT_DIR)/$*_webgl2.wasm \
		$(OUT_DIR)/$*_webgl2_bg.wasm
	@rm -f $(OUT_DIR)/$*_webgl2_bg.wasm
	@sed -i 's/$*_webgl2_bg\.wasm/$*_webgl2.wasm/g' $(OUT_DIR)/$*_webgl2.js

# Build WebGPU version
build-%-webgpu:
	@echo "=========================================="
	@echo "Build $* WebGPU"
	@echo "=========================================="
	RUSTFLAGS="$(RUSTFLAGS)" cargo build --release --example $* --target $(WASM_TARGET) --no-default-features --features $(WEBGPU_FEATURES)
	
	@echo "=========================================="
	@echo "wasm-bindgen $* WebGPU"
	@echo "=========================================="
	mkdir -p $(OUT_DIR)
	wasm-bindgen --out-name $*_webgpu --out-dir $(OUT_DIR) --no-typescript --target web \
		./target/$(WASM_TARGET)/release/examples/$*.wasm
	
	@echo "=========================================="
	@echo "Optimize WASM $* WebGPU"
	@echo "=========================================="
	wasm-opt --strip-debug --vacuum -Oz -o $(OUT_DIR)/$*_webgpu.wasm \
		$(OUT_DIR)/$*_webgpu_bg.wasm
	@rm -f $(OUT_DIR)/$*_webgpu_bg.wasm
	@sed -i 's/$*_webgpu_bg\.wasm/$*_webgpu.wasm/g' $(OUT_DIR)/$*_webgpu.js

# Pattern rule for packaging examples
package-%: build-%
	@echo "=========================================="
	@echo "Package $* WebGL2"
	@echo "=========================================="
	@mkdir -p $(BUILD_DIR)/$*_webgl2
	@mkdir -p $(BUILD_DIR)/$*_webgl2/assets
	
	@# Copy WebGL2 WASM files
	@cp $(OUT_DIR)/$*_webgl2.js $(BUILD_DIR)/$*_webgl2/
	@cp $(OUT_DIR)/$*_webgl2.wasm $(BUILD_DIR)/$*_webgl2/
	
	@# Copy assets
	@if [ -d assets ]; then cp -r assets/* $(BUILD_DIR)/$*_webgl2/assets/ 2>/dev/null || true; fi
	
	@# Create index.html for WebGL2 from template
	@sed -e 's|<title>.*</title>|<title>Bevy 2D Dissolve - $* (WebGL2)</title>|' \
		-e 's|import init from ".*";|import init from "./$*_webgl2.js";|' \
		index.html > $(BUILD_DIR)/$*_webgl2/index.html
	
	@# Create zip archive for WebGL2
	@cd $(BUILD_DIR) && zip -r $*_webgl2.zip $*_webgl2/ -q
	@echo "Created archive: $(BUILD_DIR)/$*_webgl2.zip"
	
	@echo "=========================================="
	@echo "Package $* WebGPU"
	@echo "=========================================="
	@mkdir -p $(BUILD_DIR)/$*_webgpu
	@mkdir -p $(BUILD_DIR)/$*_webgpu/assets
	
	@# Copy WebGPU WASM files
	@cp $(OUT_DIR)/$*_webgpu.js $(BUILD_DIR)/$*_webgpu/
	@cp $(OUT_DIR)/$*_webgpu.wasm $(BUILD_DIR)/$*_webgpu/
	
	@# Copy assets
	@if [ -d assets ]; then cp -r assets/* $(BUILD_DIR)/$*_webgpu/assets/ 2>/dev/null || true; fi
	
	@# Create index.html for WebGPU from template
	@sed -e 's|<title>.*</title>|<title>Bevy 2D Dissolve - $* (WebGPU)</title>|' \
		-e 's|import init from ".*";|import init from "./$*_webgpu.js";|' \
		index.html > $(BUILD_DIR)/$*_webgpu/index.html
	
	@# Create zip archive for WebGPU
	@cd $(BUILD_DIR) && zip -r $*_webgpu.zip $*_webgpu/ -q
	@echo "Created archive: $(BUILD_DIR)/$*_webgpu.zip"
	
	@# Clean up intermediate files
	@rm -f $(OUT_DIR)/$*_webgl2.js $(OUT_DIR)/$*_webgl2.wasm
	@rm -f $(OUT_DIR)/$*_webgpu.js $(OUT_DIR)/$*_webgpu.wasm

clean:
	@echo "Cleaning build artifacts..."
	@rm -rf $(OUT_DIR)
	@rm -rf $(BUILD_DIR)
	@cargo clean
	@echo "Clean complete"

.SILENT: package-%
