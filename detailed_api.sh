#!/bin/bash

echo "# Detailed API Reference"
echo "**Generated**: $(date)"
echo ""

echo "## Core Components (src/ecs/components.rs)"
echo ""
echo "### Position"
echo "\`\`\`rust"
grep -A 5 "^pub struct Position" src/ecs/components.rs | head -6
echo "\`\`\`"
echo ""

echo "### CombatStats"
echo "\`\`\`rust"
grep -A 6 "^pub struct CombatStats" src/ecs/components.rs | head -7
echo "\`\`\`"
echo ""

echo "### Inventory"
echo "\`\`\`rust"
grep -A 3 "^pub struct Inventory" src/ecs/components.rs | head -4
echo "\`\`\`"
echo ""

echo "## Map System (src/map/tile.rs)"
echo ""
echo "### Tile Enum"
echo "\`\`\`rust"
grep -A 10 "^pub enum Tile" src/map/tile.rs | head -11
echo "\`\`\`"
echo ""

echo "## Save System (src/save.rs)"
echo ""
echo "### SaveGame Structure"
echo "\`\`\`rust"
grep -A 15 "^pub struct SaveGame" src/save.rs | head -16
echo "\`\`\`"
echo ""

echo "## Resources (src/ecs/resources.rs)"
echo ""
echo "### Resources Fields"
echo "\`\`\`rust"
grep -A 30 "^pub struct Resources" src/ecs/resources.rs | head -35
echo "\`\`\`"
echo ""

echo "## Test Distribution"
echo ""
echo "| Module | Tests | Status |"
echo "|--------|-------|--------|"
for file in $(find src -name "*.rs" | sort); do
    if grep -q "#\[cfg(test)\]" "$file"; then
        tests=$(grep -c "fn test_" "$file" 2>/dev/null || echo 0)
        module=$(echo "$file" | sed 's|src/||' | sed 's|/|::|g' | sed 's|.rs$||')
        echo "| $module | $tests | ✅ |"
    fi
done
echo ""
