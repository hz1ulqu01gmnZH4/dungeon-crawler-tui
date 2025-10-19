# Static Analysis Tools for Rust Code Structure

**Date**: 2025-10-18
**Purpose**: Tools to map functions, structs, dependencies, and call graphs

---

## Available Tools for Rust

### 1. **cargo-modules** ⭐ RECOMMENDED
**Purpose**: Visualize module structure and dependencies

**Installation**:
```bash
cargo install cargo-modules
```

**Usage**:
```bash
# Show module tree
cargo modules generate tree

# Show dependency graph
cargo modules generate graph | dot -Tpng > module_graph.png

# Show orphan modules
cargo modules orphans
```

**Output**: ASCII tree or GraphViz DOT format
**Best For**: Understanding module organization

---

### 2. **cargo-deps** / **cargo-depgraph**
**Purpose**: Dependency graph visualization

**Installation**:
```bash
cargo install cargo-deps
```

**Usage**:
```bash
# Generate dependency graph
cargo deps | dot -Tpng > dependencies.png

# Include all dependencies
cargo deps --all-deps | dot -Tpng > full_deps.png
```

**Output**: GraphViz DOT → PNG/SVG
**Best For**: External dependency analysis

---

### 3. **rust-analyzer** ⭐ RECOMMENDED
**Purpose**: IDE-level code intelligence (already in use)

**Features**:
- Go to definition
- Find references
- Type hierarchy
- Call hierarchy
- Symbol search

**Usage**: Built into VSCode/Neovim/Emacs
**Best For**: Interactive code navigation

---

### 4. **rustdoc**
**Purpose**: Generate documentation with structure

**Installation**: Built-in with Rust

**Usage**:
```bash
# Generate docs
cargo doc --no-deps --open

# With private items
cargo doc --document-private-items --open
```

**Output**: HTML documentation with:
- Struct definitions
- Function signatures
- Type relationships
- Source links

**Best For**: Browsing API structure

---

### 5. **cargo-geiger**
**Purpose**: Find unsafe code

**Installation**:
```bash
cargo install cargo-geiger
```

**Usage**:
```bash
cargo geiger
```

**Output**: Safety statistics
**Best For**: Security audit

---

### 6. **cargo-tree**
**Purpose**: Display dependency tree (built-in since Rust 1.44)

**Installation**: Built-in

**Usage**:
```bash
# Show dependency tree
cargo tree

# Show specific package
cargo tree -p dungeon-clawler-tui

# Show duplicates
cargo tree --duplicates

# Reverse dependencies
cargo tree --invert serde
```

**Output**: ASCII tree
**Best For**: Dependency analysis

---

### 7. **tokei**
**Purpose**: Code statistics

**Installation**:
```bash
cargo install tokei
```

**Usage**:
```bash
tokei
```

**Output**: Lines of code by language/file
**Best For**: Code metrics

---

### 8. **cargo-bloat**
**Purpose**: Find what takes up space in binary

**Installation**:
```bash
cargo install cargo-bloat
```

**Usage**:
```bash
cargo bloat --release
```

**Output**: Binary size breakdown
**Best For**: Size optimization

---

### 9. **cargo-expand**
**Purpose**: Show macro expansions

**Installation**:
```bash
cargo install cargo-expand
```

**Usage**:
```bash
# Expand specific module
cargo expand systems::combat
```

**Output**: Expanded Rust code
**Best For**: Understanding macros

---

### 10. **cargo-call-stack**
**Purpose**: Analyze call stacks

**Installation**:
```bash
cargo install cargo-call-stack
```

**Usage**:
```bash
cargo call-stack --bin dungeon-clawler-tui > callstack.dot
dot -Tpng callstack.dot > callstack.png
```

**Output**: Call graph
**Best For**: Understanding execution flow

---

## Custom Analysis Scripts

### Script 1: Function Counter

```bash
#!/bin/bash
# Count functions by module

echo "Functions by file:"
for file in $(find src -name "*.rs"); do
    count=$(grep -c "fn " "$file")
    if [ $count -gt 0 ]; then
        echo "$file: $count functions"
    fi
done | sort -t: -k2 -rn
```

Save as `count_functions.sh`, then:
```bash
chmod +x count_functions.sh
./count_functions.sh
```

---

### Script 2: Struct/Enum Finder

```bash
#!/bin/bash
# Find all structs and enums

echo "=== Structs ==="
grep -rn "^pub struct\|^struct" src --include="*.rs" | sed 's/:/ | /'

echo -e "\n=== Enums ==="
grep -rn "^pub enum\|^enum" src --include="*.rs" | sed 's/:/ | /'
```

---

### Script 3: Test Coverage Map

```bash
#!/bin/bash
# Show which files have tests

echo "Files with tests:"
for file in $(find src -name "*.rs"); do
    if grep -q "#\[cfg(test)\]" "$file"; then
        tests=$(grep -c "fn test_" "$file" 2>/dev/null || echo 0)
        echo "✅ $file ($tests tests)"
    else
        echo "❌ $file (no tests)"
    fi
done
```

---

## Practical Usage for This Project

### Generate Module Tree

```bash
cargo install cargo-modules
cargo modules generate tree > module_structure.txt
```

**Sample Output**:
```
dungeon-clawler-tui
├── ecs
│   ├── components
│   └── resources
├── game
│   ├── app
│   ├── state
│   └── world
├── map
│   ├── chunks
│   ├── fov
│   ├── generator
│   └── tile
├── systems
│   ├── ai
│   ├── combat
│   ├── fov
│   ├── input
│   ├── inventory
│   └── movement
├── ui
│   ├── character_screen
│   ├── examine_renderer
│   ├── inventory_renderer
│   ├── main_menu
│   ├── minimap
│   ├── overmap_renderer
│   └── renderer
└── world
    ├── building
    ├── generator
    ├── overmap
    ├── placement
    ├── poi
    ├── roads
    ├── settlement
    ├── settlement_gen
    ├── time
    ├── travel_events
    └── weather
```

---

### Generate Dependency Graph

```bash
cargo install cargo-deps
cargo deps | dot -Tpng > project_deps.png
```

**Creates**: Visual graph of crate dependencies

---

### Generate Documentation

```bash
cargo doc --document-private-items --no-deps --open
```

**Creates**: Interactive HTML docs at `target/doc/dungeon_clawler_tui/index.html`

**Features**:
- Click through struct definitions
- See function signatures
- View type relationships
- Browse source code

---

### View Dependency Tree

```bash
cargo tree | head -50
```

**Sample Output**:
```
dungeon-clawler-tui v0.1.0
├── anyhow v1.0.75
├── crossterm v0.27.0
│   ├── bitflags v2.4.0
│   ├── libc v0.2.150
│   ├── parking_lot v0.12.1
│   └── ...
├── hecs v0.10.3
│   └── hashbrown v0.14.2
├── rand v0.8.5
│   ├── rand_core v0.6.4
│   └── ...
├── ratatui v0.24.0
│   ├── crossterm v0.27.0 (*)
│   └── ...
└── serde v1.0.193
    └── serde_derive v1.0.193
```

---

### Code Statistics

```bash
cargo install tokei
tokei
```

**Sample Output**:
```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Rust                   50         8450         7200          350          900
 Markdown                8         2100         1600            0          500
===============================================================================
 Total                  58        10550         8800          350         1400
===============================================================================
```

---

## GraphViz Visualization

### Install GraphViz

```bash
# Ubuntu/Debian
sudo apt install graphviz

# macOS
brew install graphviz

# Arch
sudo pacman -S graphviz
```

### Generate Call Graph

```bash
cargo install cargo-call-stack
cargo call-stack --bin dungeon-clawler-tui > callgraph.dot
dot -Tsvg callgraph.dot > callgraph.svg
```

---

## Quick Analysis Commands

### 1. Find All Public Functions
```bash
grep -rn "pub fn" src --include="*.rs" | wc -l
```

### 2. Find All Structs
```bash
grep -rn "struct" src --include="*.rs" | grep -v "//" | wc -l
```

### 3. Find All Tests
```bash
grep -rn "#\[test\]" src --include="*.rs" | wc -l
```

### 4. Find Unsafe Code
```bash
grep -rn "unsafe" src --include="*.rs"
```

### 5. Find TODOs
```bash
grep -rn "TODO\|FIXME\|XXX" src --include="*.rs"
```

### 6. Function Length Analysis
```bash
# Find long functions (>50 lines)
for file in $(find src -name "*.rs"); do
    awk '/fn /{p=1; name=$0; line=NR; count=0}
         p && /}/{count++; if(count==1 && NR-line>50)
         print FILENAME":"line": "name" ("NR-line" lines)"}' "$file"
done
```

---

## IDE Integration

### VSCode Extensions

1. **rust-analyzer**
   - Call hierarchy: Right-click → "Show Call Hierarchy"
   - Type hierarchy: Right-click → "Show Type Hierarchy"
   - Symbol search: Ctrl+T

2. **CodeLLDB** (for debugging)
   - Stack traces
   - Variable inspection

3. **Better TOML**
   - Cargo.toml analysis

---

## Custom Analysis for This Project

### Generate Complete Map

```bash
#!/bin/bash
# generate_code_map.sh

echo "# Code Structure Map" > CODE_MAP.md
echo "**Generated**: $(date)" >> CODE_MAP.md
echo "" >> CODE_MAP.md

echo "## Module Statistics" >> CODE_MAP.md
echo "\`\`\`" >> CODE_MAP.md
tokei src >> CODE_MAP.md
echo "\`\`\`" >> CODE_MAP.md
echo "" >> CODE_MAP.md

echo "## Structs by Module" >> CODE_MAP.md
for file in $(find src -name "*.rs" | sort); do
    structs=$(grep -c "^pub struct\|^struct" "$file" 2>/dev/null || echo 0)
    if [ $structs -gt 0 ]; then
        echo "- \`$file\`: $structs structs" >> CODE_MAP.md
    fi
done
echo "" >> CODE_MAP.md

echo "## Functions by Module" >> CODE_MAP.md
for file in $(find src -name "*.rs" | sort); do
    funcs=$(grep -c "fn " "$file" 2>/dev/null || echo 0)
    if [ $funcs -gt 0 ]; then
        echo "- \`$file\`: $funcs functions" >> CODE_MAP.md
    fi
done
echo "" >> CODE_MAP.md

echo "## Test Coverage" >> CODE_MAP.md
for file in $(find src -name "*.rs" | sort); do
    if grep -q "#\[cfg(test)\]" "$file"; then
        tests=$(grep -c "fn test_" "$file" 2>/dev/null || echo 0)
        echo "- ✅ \`$file\`: $tests tests" >> CODE_MAP.md
    fi
done

echo "Code map generated: CODE_MAP.md"
```

---

## Recommended Setup

### For This Project

1. **Install Essential Tools**:
```bash
cargo install cargo-modules
cargo install tokei
cargo install cargo-expand
```

2. **Generate Documentation**:
```bash
cargo doc --document-private-items --no-deps
```

3. **View Module Structure**:
```bash
cargo modules generate tree
```

4. **Get Code Stats**:
```bash
tokei
```

---

## Advanced: AST Analysis

### Using syn (programmatic)

```rust
// analyze.rs
use syn::{File, Item};
use std::fs;

fn main() {
    let code = fs::read_to_string("src/main.rs").unwrap();
    let ast: File = syn::parse_file(&code).unwrap();

    for item in ast.items {
        match item {
            Item::Fn(func) => println!("Function: {}", func.sig.ident),
            Item::Struct(s) => println!("Struct: {}", s.ident),
            Item::Enum(e) => println!("Enum: {}", e.ident),
            _ => {}
        }
    }
}
```

---

## Summary

### Quick Start (3 commands)
```bash
# 1. Install basics
cargo install cargo-modules tokei

# 2. View structure
cargo modules generate tree

# 3. Get statistics
tokei
```

### For Deep Analysis
```bash
# Install full suite
cargo install cargo-modules cargo-deps cargo-expand tokei cargo-bloat

# Generate all artifacts
cargo doc --document-private-items --no-deps --open
cargo modules generate tree > structure.txt
cargo tree > dependencies.txt
tokei > statistics.txt
```

### Best Tools by Use Case

| Use Case | Tool | Command |
|----------|------|---------|
| Module structure | cargo-modules | `cargo modules generate tree` |
| Dependencies | cargo-tree | `cargo tree` |
| Documentation | rustdoc | `cargo doc --open` |
| Code stats | tokei | `tokei` |
| Call graphs | cargo-call-stack | `cargo call-stack` |
| IDE navigation | rust-analyzer | Built-in VSCode |
| Binary size | cargo-bloat | `cargo bloat --release` |
| Macro expansion | cargo-expand | `cargo expand` |

---

**For this project, I recommend**:
1. ✅ `cargo doc --document-private-items --open` - Immediate, built-in
2. ✅ `cargo install cargo-modules` - Best structure visualization
3. ✅ `cargo install tokei` - Quick statistics

These three give you 90% of what you need for code navigation and understanding.

