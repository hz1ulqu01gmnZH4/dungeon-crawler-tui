#!/bin/bash
# Code Structure Analysis Script

echo "# Code Structure Analysis Report"
echo "**Generated**: $(date)"
echo ""

echo "## Code Statistics"
echo ""
echo "\`\`\`"
tokei src
echo "\`\`\`"
echo ""

echo "## Structs by Module"
echo ""
for file in $(find src -name "*.rs" | sort); do
    structs=$(grep -c "^pub struct\|^struct" "$file" 2>/dev/null || echo 0)
    if [ $structs -gt 0 ]; then
        echo "- \`$file\`: $structs structs"
    fi
done
echo ""

echo "## Functions by Module"
echo ""
for file in $(find src -name "*.rs" | sort); do
    funcs=$(grep -c "fn " "$file" 2>/dev/null || echo 0)
    if [ $funcs -gt 0 ]; then
        echo "- \`$file\`: $funcs functions"
    fi
done
echo ""

echo "## Test Coverage by Module"
echo ""
for file in $(find src -name "*.rs" | sort); do
    if grep -q "#\[cfg(test)\]" "$file"; then
        tests=$(grep -c "fn test_" "$file" 2>/dev/null || echo 0)
        echo "- ✅ \`$file\`: $tests tests"
    fi
done
echo ""

echo "## Module Organization"
echo ""
echo "\`\`\`"
tree -d -L 3 src 2>/dev/null || find src -type d | sort | sed 's|^src|.|'
echo "\`\`\`"
