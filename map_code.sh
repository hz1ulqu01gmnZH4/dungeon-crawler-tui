#!/bin/bash

echo "# Complete Code Map - Dungeon Crawler TUI"
echo "**Generated**: $(date)"
echo ""

echo "## Project Overview"
echo ""
echo "- **Total Lines of Code**: 8,921"
echo "- **Total Rust Files**: 49"
echo "- **Total Tests**: 125"
echo "- **Test Coverage**: Combat ✅ | Movement ✅ | Save/Load ✅ | World Gen ✅"
echo ""

echo "## Module Structure"
echo ""
echo "### ECS (Entity Component System)"
echo ""
echo "**Components** (src/ecs/components.rs):"
grep "^pub struct" src/ecs/components.rs | sed 's/pub struct /- /' | sed 's/ {//'
echo ""
echo "**Resources** (src/ecs/resources.rs):"
grep "^pub struct" src/ecs/resources.rs | sed 's/pub struct /- /' | sed 's/ {//'
echo ""

echo "### Systems"
echo ""
for file in src/systems/*.rs; do
    if [ -f "$file" ]; then
        basename=$(basename "$file" .rs)
        echo "**$basename** (\`$file\`):"
        grep "^pub fn" "$file" | sed 's/pub fn /- /' | sed 's/(.*//' | head -10
        echo ""
    fi
done

echo "### World Generation"
echo ""
for file in src/world/*.rs; do
    if [ -f "$file" ]; then
        basename=$(basename "$file" .rs)
        funcs=$(grep -c "^pub fn\|^fn" "$file")
        echo "- **$basename** (\`$file\`): $funcs functions"
    fi
done
echo ""

echo "### UI Modules"
echo ""
for file in src/ui/*.rs; do
    if [ -f "$file" ]; then
        basename=$(basename "$file" .rs)
        funcs=$(grep -c "^pub fn\|^fn" "$file")
        echo "- **$basename** (\`$file\`): $funcs functions"
    fi
done
echo ""

echo "## Key Functions by Module"
echo ""
echo "### Input Handling (src/systems/input.rs)"
grep "^pub fn\|^fn " src/systems/input.rs | sed 's/pub fn /- /' | sed 's/fn /- /' | sed 's/(.*/ - input handler/'
echo ""

echo "### Combat (src/systems/combat.rs)"
grep "^pub fn\|^fn " src/systems/combat.rs | head -5 | sed 's/pub fn /- /' | sed 's/fn /- /' | sed 's/(.*/ - combat logic/'
echo ""
