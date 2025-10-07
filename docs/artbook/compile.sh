#!/bin/bash

# Compilation script for The Unraveling of Kándavael Artbook
# Handles font fallbacks and missing packages gracefully

echo "═══════════════════════════════════════════"
echo "  The Unraveling of Kándavael - Artbook"
echo "═══════════════════════════════════════════"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check for LuaLaTeX
if ! command -v lualatex &> /dev/null; then
    echo -e "${RED}Error: LuaLaTeX not found!${NC}"
    echo "Please install texlive-luatex or equivalent package."
    exit 1
fi

# Create necessary directories
mkdir -p build
mkdir -p images

# Create a minimal version without special fonts if they're missing
create_minimal_version() {
    echo -e "${YELLOW}Creating minimal version without special fonts...${NC}"
    cat > artbook_minimal.tex << 'EOF'
\documentclass[11pt,a4paper,twoside]{book}
\usepackage[margin=2.5cm]{geometry}
\usepackage{fontspec}
\usepackage{graphicx}
\usepackage{xcolor}
\usepackage{tikz}
\usepackage{tcolorbox}
\usepackage{fancyhdr}
\usepackage{lettrine}
\usepackage{hyperref}

% Fallback fonts
\setmainfont{Latin Modern Roman}
\newfontfamily\displayfont{Latin Modern Roman}[
    Letters=UppercaseSmallCaps
]

% Color definitions
\definecolor{dawn}{RGB}{255, 220, 150}
\definecolor{dusk}{RGB}{120, 80, 140}
\definecolor{blood}{RGB}{139, 0, 0}
\definecolor{void}{RGB}{20, 0, 30}

\title{The Unraveling of Kándavael}
\author{A Linguistic Horror Artbook}
\date{}

\begin{document}
\maketitle

\chapter{Introduction}

This is a minimal version of the artbook compiled without special fonts.

The full version includes:
\begin{itemize}
    \item EB Garamond font for main text
    \item Cinzel font for display text
    \item UnifrakturMaguntia for horror text
    \item Advanced typography and effects
\end{itemize}

\chapter{The Language System}

\section{High Vaelic to Under-Vêlth}

The game features a constructed language that transforms from noble to horrific:

\begin{itemize}
    \item \textbf{Kándavael} → \textbf{Khândvêl} (Candle Vale → Seared Gullet)
    \item \textbf{Dael Tríthae} → \textbf{Dæl Trith} (Dawn Trinity → Draining Throats)
    \item \textbf{Vardain} → \textbf{Vârð} (Wardens → Sutures)
\end{itemize}

\end{document}
EOF
    lualatex -output-directory=build artbook_minimal.tex
    mv build/artbook_minimal.pdf .
    echo -e "${GREEN}Minimal version created: artbook_minimal.pdf${NC}"
}

# Try to compile the full version
echo "Attempting to compile full artbook..."
if lualatex -interaction=nonstopmode -output-directory=build artbook.tex; then
    # Second pass for references
    lualatex -interaction=nonstopmode -output-directory=build artbook.tex
    mv build/artbook.pdf .
    echo -e "${GREEN}✓ Full artbook compiled successfully!${NC}"
    echo "Output: artbook.pdf"
else
    echo -e "${YELLOW}Warning: Full compilation failed.${NC}"
    echo "This might be due to missing fonts or packages."
    echo ""
    read -p "Create minimal version instead? (y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        create_minimal_version
    fi
fi

# Clean up auxiliary files
echo "Cleaning auxiliary files..."
rm -f *.aux *.log *.out *.toc *.synctex.gz
rm -rf build/*.aux build/*.log build/*.out build/*.toc

echo ""
echo "═══════════════════════════════════════════"
echo "  Compilation complete!"
echo "═══════════════════════════════════════════"