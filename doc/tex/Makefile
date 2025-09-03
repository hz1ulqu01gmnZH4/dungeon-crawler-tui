# Makefile for Dungeon Clawler Art Book
# Supports multiple LaTeX compilers

MAIN = dungeon-clawler-artbook
LATEX = pdflatex
LUALATEX = lualatex
XELATEX = xelatex

# Default target
all: pdf

# Try to compile with available compiler
pdf:
	@echo "Attempting to compile $(MAIN).tex..."
	@if command -v lualatex > /dev/null 2>&1; then \
		echo "Using LuaLaTeX..."; \
		$(LUALATEX) $(MAIN).tex; \
		$(LUALATEX) $(MAIN).tex; \
	elif command -v xelatex > /dev/null 2>&1; then \
		echo "Using XeLaTeX..."; \
		$(XELATEX) $(MAIN).tex; \
		$(XELATEX) $(MAIN).tex; \
	elif command -v pdflatex > /dev/null 2>&1; then \
		echo "Using PDFLaTeX..."; \
		$(LATEX) $(MAIN).tex; \
		$(LATEX) $(MAIN).tex; \
	else \
		echo "Error: No LaTeX compiler found!"; \
		echo "Please install texlive-full or miktex"; \
		exit 1; \
	fi

# LuaLaTeX compilation (recommended)
lualatex:
	$(LUALATEX) $(MAIN).tex
	$(LUALATEX) $(MAIN).tex

# XeLaTeX compilation
xelatex:
	$(XELATEX) $(MAIN).tex
	$(XELATEX) $(MAIN).tex

# PDFLaTeX compilation
pdflatex:
	$(LATEX) $(MAIN).tex
	$(LATEX) $(MAIN).tex

# Clean auxiliary files
clean:
	rm -f *.aux *.log *.out *.toc *.lof *.lot *.bbl *.blg *.nav *.snm *.vrb

# Clean all including PDF
cleanall: clean
	rm -f $(MAIN).pdf

# View the PDF
view:
	@if [ -f $(MAIN).pdf ]; then \
		if command -v xdg-open > /dev/null 2>&1; then \
			xdg-open $(MAIN).pdf; \
		elif command -v open > /dev/null 2>&1; then \
			open $(MAIN).pdf; \
		else \
			echo "PDF viewer not found. Please open $(MAIN).pdf manually."; \
		fi \
	else \
		echo "PDF not found. Run 'make' first."; \
	fi

.PHONY: all pdf lualatex xelatex pdflatex clean cleanall view