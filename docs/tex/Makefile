# Makefile for The Unraveling of Kándavael Artbook

# LaTeX compiler
LATEX = lualatex
LATEXFLAGS = -shell-escape -interaction=nonstopmode

# Main document
MAIN = artbook
SOURCES = $(MAIN).tex
PDF = $(MAIN).pdf

# Image directories
IMGDIR = images
IMAGES = $(wildcard $(IMGDIR)/*.png) $(wildcard $(IMGDIR)/*.jpg)

# Build directory
BUILDDIR = build

.PHONY: all clean view install-deps create-dirs

# Default target
all: create-dirs $(PDF)

# Create necessary directories
create-dirs:
	@mkdir -p $(BUILDDIR)
	@mkdir -p $(IMGDIR)

# Compile the PDF
$(PDF): $(SOURCES) $(IMAGES)
	@echo "Compiling artbook..."
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN)
	# Run twice for proper references
	$(LATEX) $(LATEXFLAGS) -output-directory=$(BUILDDIR) $(MAIN)
	@mv $(BUILDDIR)/$(PDF) .
	@echo "Artbook compiled successfully!"

# Clean build files
clean:
	@echo "Cleaning build files..."
	@rm -rf $(BUILDDIR)
	@rm -f $(PDF)
	@rm -f *.aux *.log *.out *.toc *.synctex.gz
	@echo "Clean complete!"

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
	@echo "Artbook Makefile"
	@echo "================"
	@echo "Available targets:"
	@echo "  make              - Compile the artbook PDF"
	@echo "  make clean        - Remove build files"
	@echo "  make view         - Compile and view the PDF"
	@echo "  make quick        - Quick compile (single pass)"
	@echo "  make watch        - Auto-recompile on changes"
	@echo "  make install-deps - Install LaTeX dependencies (Ubuntu/Debian)"
	@echo "  make generate-placeholders - Create placeholder images"