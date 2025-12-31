# Documentation Assets Makefile
# Generates terminal screenshots and animations for fast-rich docs

# Tool locations
TERMSHOT := ~/go/bin/termshot
ASCIINEMA := asciinema
AGG := agg
SVG_TERM := svg-term
ASSETS_DIR := docs/assets

.PHONY: all screenshots animations docs clean help

all: screenshots animations docs

help:
	@echo "Fast-Rich Documentation Assets"
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  screenshots  Generate PNG screenshots for static features"
	@echo "  animations   Generate GIF animations for dynamic features"
	@echo "  docs         Build MkDocs documentation"
	@echo "  serve        Serve docs locally with live reload"
	@echo "  clean        Remove generated assets"
	@echo ""
	@echo "Prerequisites:"
	@echo "  - termshot: go install github.com/homeport/termshot/cmd/termshot@latest"
	@echo "  - asciinema: brew install asciinema"
	@echo "  - agg: brew install agg"
	@echo "  - svg-term: npm install -g svg-term-cli"
	@echo "  - mkdocs-material: pip install mkdocs-material pymdown-extensions"

# Create assets directory
$(ASSETS_DIR):
	mkdir -p $(ASSETS_DIR)

# ============================================================================
# PNG Screenshots (Static Features)
# ============================================================================

screenshots: $(ASSETS_DIR)
	@echo "Generating PNG screenshots for static features..."
	
	# Hero example (short, concise)
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/hero.png -- \
		cargo run --quiet --example hero
	
	# Full showcase
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/showcase.png -- \
		cargo run --example showcase --features full
	
	# Individual features
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/styles.png -- \
		cargo run --example styles_demo
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/tables.png -- \
		cargo run --example tables_demo
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/tree.png -- \
		cargo run --example tree_view
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/panel.png -- \
		cargo run --example panel
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/logging.png -- \
		cargo run --example logging --features logging
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/traceback.png -- \
		cargo run --example traceback_demo
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/syntax.png -- \
		cargo run --example syntax_highlighting --features syntax
	$(TERMSHOT) --show-cmd --filename $(ASSETS_DIR)/layout.png -- \
		cargo run --example layout_demo
	
	@echo "PNG screenshots generated in $(ASSETS_DIR)/"

# ============================================================================
# SVG Generation (Vector Graphics)
# ============================================================================

svg-hero: $(ASSETS_DIR)
	@echo "Generating SVG hero image..."
	$(ASCIINEMA) rec --command "cargo run --quiet --example hero" \
		--overwrite $(ASSETS_DIR)/hero.cast
	# Convert v3 to v2 manually and generate SVG
	$(SVG_TERM) --in $(ASSETS_DIR)/hero_v2.cast --out $(ASSETS_DIR)/hero.svg \
		--window --width 60 --height 15 --no-cursor
	@echo "SVG hero generated: $(ASSETS_DIR)/hero.svg"

# ============================================================================
# GIF Animations (Professional VHS Tapes)
# ============================================================================

animations: $(ASSETS_DIR)
	@echo "Generating professional GIF animations with VHS..."
	
	# Progress bars
	vhs $(ASSETS_DIR)/progress.tape
	
	# Live display
	vhs $(ASSETS_DIR)/live.tape

	# Hero
	vhs $(ASSETS_DIR)/hero.tape
	
	# Documentation automation
	vhs $(ASSETS_DIR)/make_screenshots.tape

	# Full examples showcase
	vhs $(ASSETS_DIR)/run_all.tape
	
	@echo "Professional GIF animations generated in $(ASSETS_DIR)/"

# ============================================================================
# Documentation Build
# ============================================================================

docs:
	source .venv/bin/activate && mkdocs build

serve:
	source .venv/bin/activate && mkdocs serve

# ============================================================================
# Cleanup
# ============================================================================

clean:
	rm -f $(ASSETS_DIR)/*.png $(ASSETS_DIR)/*.gif $(ASSETS_DIR)/*.svg $(ASSETS_DIR)/*.cast
