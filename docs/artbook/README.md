# Dungeon Clawler TUI - Art Book & Settings Compendium

This directory contains the LaTeX source for "The Unraveling of Kándavael" art book and settings documentation for the Dungeon Clawler TUI game.

## Contents

- `dungeon-clawler-artbook.tex` - Main LaTeX document
- `images/` - Generated concept art images
- `Makefile` - Build automation

## Building the PDF

### Prerequisites

You need a LaTeX distribution installed:

**Ubuntu/Debian:**
```bash
sudo apt-get install texlive-full
```

**Fedora:**
```bash
sudo dnf install texlive-scheme-full
```

**macOS:**
```bash
brew install --cask mactex
```

**Windows:**
Download and install MiKTeX from https://miktex.org/

### Compilation

Once LaTeX is installed, compile the document:

```bash
make
```

Or specifically with LuaLaTeX (recommended):
```bash
make lualatex
```

### Viewing

To open the generated PDF:
```bash
make view
```

### Cleaning

Remove auxiliary files:
```bash
make clean
```

Remove all generated files including PDF:
```bash
make cleanall
```

## Document Structure

The art book includes:

1. **World Lore** - The backstory of Kándavael and the dungeon
2. **Bestiary** - Detailed enemy descriptions with stats and artwork
3. **Artifacts** - Legendary items and their properties
4. **Dungeon Architecture** - Level generation and special rooms
5. **Game Mechanics** - Combat system, progression, and skills
6. **Deep Lore** - Prophecies and hidden truths
7. **Appendices** - Commands, ASCII art, and configuration

## Image Generation

All concept art was generated using AI image generation tools and styled to match the terminal-based aesthetic of the game while providing high-quality fantasy artwork.

## Notes

- The document uses custom color schemes matching the terminal aesthetic
- Special boxes highlight lore entries and game mechanics
- ASCII art examples are preserved in verbatim environments
- The document is designed for both digital viewing and printing