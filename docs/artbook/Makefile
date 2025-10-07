# Makefile for The Unraveling of Kándavael Artbook

# LaTeX compiler
LATEX = lualatex
LATEXFLAGS = -shell-escape -interaction=nonstopmode

# Main documents
MAIN = artbook_expanded
MAIN_ALT = artbook
MAIN_SIMPLE = dungeon-clawler-artbook
SOURCES = $(MAIN).tex
PDF = $(MAIN).pdf

# Image directories
IMGDIR = images
IMAGES = $(wildcard $(IMGDIR)/*.png) $(wildcard $(IMGDIR)/*.jpg)

# Build directory
BUILDDIR = build

.PHONY: all clean view install-deps create-dirs expanded simple original

# Default target (builds expanded version)
all: create-dirs $(PDF)

# Create necessary directories
create-dirs:
	@mkdir -p $(BUILDDIR)
	@mkdir -p $(IMGDIR)

# Compile the expanded PDF (default)
$(PDF): $(SOURCES) $(IMAGES)
	@echo "Compiling artbook (expanded version)..."
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN)
	# Run twice for proper references
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN)
	@mv $(BUILDDIR)/$(PDF) .
	@echo "Artbook compiled successfully!"

# Build expanded version explicitly
expanded: create-dirs
	@echo "Compiling expanded artbook..."
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN)
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN)
	@mv $(BUILDDIR)/$(MAIN).pdf .

# Build standard version
standard: create-dirs
	@echo "Compiling standard artbook..."
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN_ALT)
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN_ALT)
	@mv $(BUILDDIR)/$(MAIN_ALT).pdf .

# Build simple version
simple: create-dirs
	@echo "Compiling simple artbook..."
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN_SIMPLE)
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN_SIMPLE)
	@mv $(BUILDDIR)/$(MAIN_SIMPLE).pdf .

# Build all versions
all-versions: expanded standard simple
	@echo "All artbook versions compiled!"

# Clean build files
clean:
	@echo "Cleaning build files..."
	@rm -rf $(BUILDDIR)
	@rm -f *.aux *.log *.out *.toc *.synctex.gz
	@echo "Clean complete!"

# Clean all including PDFs
cleanall: clean
	@rm -f artbook_expanded.pdf artbook.pdf dungeon-clawler-artbook.pdf
	@echo "All files cleaned!"

# View the PDF
view: $(PDF)
	@if command -v xdg-open > /dev/null; then \
		xdg-open $(PDF); \
	elif command -v open > /dev/null; then \
		open $(PDF); \
	else \
		echo "Please open $(PDF) manually"; \
	fi

# Install LaTeX dependencies (Ubuntu/Debian)
install-deps:
	@echo "Installing LaTeX dependencies..."
	sudo apt-get update
	sudo apt-get install -y \
		texlive-full \
		texlive-luatex \
		texlive-fonts-extra \
		texlive-latex-extra \
		fonts-ebgaramond \
		fonts-cinzel

# Install LaTeX dependencies (Arch)
install-deps-arch:
	@echo "Installing LaTeX dependencies for Arch..."
	sudo pacman -S \
		texlive-most \
		texlive-luatex \
		texlive-fontsextra

# Install LaTeX dependencies (Fedora)
install-deps-fedora:
	@echo "Installing LaTeX dependencies for Fedora..."
	sudo dnf install -y \
		texlive-scheme-full \
		texlive-luatex

# Install LaTeX dependencies (macOS)
install-deps-mac:
	@echo "Installing LaTeX dependencies for macOS..."
	@echo "Please install MacTeX from https://www.tug.org/mactex/"
	@echo "Or use: brew install --cask mactex"

# Generate placeholder images
generate-placeholders:
	@echo "Generating placeholder images..."
	@mkdir -p $(IMGDIR)
	# Create placeholder with ImageMagick if available
	@if command -v convert > /dev/null; then \
		convert -size 800x600 xc:gray -pointsize 48 -gravity center \
			-annotate +0+0 "Trinity Transformation" $(IMGDIR)/trinity_transformation.png; \
		convert -size 400x600 xc:gray -pointsize 36 -gravity center \
			-annotate +0+0 "Priestess Portrait" $(IMGDIR)/priestess_portrait.png; \
	else \
		echo "ImageMagick not found. Please create images manually."; \
	fi

# Quick compile (single pass, faster)
quick: create-dirs
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN)
	@mv $(BUILDDIR)/$(PDF) .

# Watch for changes and recompile
watch:
	@echo "Watching for changes..."
	@while true; do \
		inotifywait -e modify $(SOURCES); \
		make quick; \
	done

help:
	@echo "Artbook Makefile - The Unraveling of Kándavael"
	@echo "=============================================="
	@echo "Available targets:"
	@echo "  make              - Compile the expanded artbook PDF (default)"
	@echo "  make expanded     - Compile expanded version (1124 lines)"
	@echo "  make standard     - Compile standard version (497 lines)"
	@echo "  make simple       - Compile simple version (421 lines)"
	@echo "  make all-versions - Compile all three versions"
	@echo "  make clean        - Remove build files"
	@echo "  make cleanall     - Remove build files and PDFs"
	@echo "  make view         - Compile and view the PDF"
	@echo "  make quick        - Quick compile (single pass)"
	@echo "  make watch        - Auto-recompile on changes (requires inotify-tools)"
	@echo ""
	@echo "Install dependencies:"
	@echo "  make install-deps       - Ubuntu/Debian"
	@echo "  make install-deps-arch  - Arch Linux"
	@echo "  make install-deps-fedora- Fedora/RHEL"
	@echo "  make install-deps-mac   - macOS"
	@echo ""
	@echo "  make generate-placeholders - Create placeholder images"
