#!/bin/bash
# Script to replace "Moltis" branding with "ClawMaster" in user-visible files
# This script only replaces display text, NOT code identifiers or package names

set -e

echo "🔄 Replacing Moltis branding with ClawMaster..."

# Function to replace in a file
replace_in_file() {
    local file="$1"
    if [ ! -f "$file" ]; then
        echo "⚠️  File not found: $file"
        return
    fi
    
    echo "📝 Processing: $file"
    
    # Create backup
    cp "$file" "$file.bak"
    
    # Replace user-visible text (NOT package names or code identifiers)
    # Only replace standalone "Moltis" or "clawmaster" that are display text
    sed -i.tmp \
        -e 's/\bMoltis — A Rust-native claw you can trust\b/ClawMaster — A Rust-native claw you can trust/g' \
        -e 's/\bMoltis recently hit\b/ClawMaster recently hit/g' \
        -e 's/making Moltis excellent/making ClawMaster excellent/g' \
        -e 's/\bGet your AI assistant at\b.*clawmaster\.org/Get your AI assistant at clawmaster.org/g' \
        -e 's/alt="Moltis icon"/alt="ClawMaster icon"/g' \
        -e 's/Restart Moltis to apply/Restart ClawMaster to apply/g' \
        -e 's/New message from clawmaster/New message from ClawMaster/g' \
        -e 's/Service Worker for clawmaster PWA/Service Worker for ClawMaster PWA/g' \
        "$file"
    
    rm -f "$file.tmp"
    
    # Check if file changed
    if diff -q "$file" "$file.bak" > /dev/null 2>&1; then
        echo "   ✓ No changes needed"
        rm "$file.bak"
    else
        echo "   ✅ Updated"
    fi
}

# Main documentation files (user-facing)
echo ""
echo "📚 Updating main documentation..."
replace_in_file "README.md"
replace_in_file "RUNNING_GUIDE.md"
replace_in_file "CERTIFICATE_GUIDE.md"
replace_in_file "AUTO_EXECUTION_GUIDE.md"
replace_in_file "AUDIT_REPORT.md"
replace_in_file "CLAWHUB_FIX_REPORT.md"
replace_in_file "CLAWHUB_COMPLETE.md"

# Web UI files
echo ""
echo "🌐 Updating Web UI files..."
replace_in_file "crates/web/src/templates/index.html"
replace_in_file "crates/web/src/assets/sw.js"
replace_in_file "crates/web/src/assets/share.html"
replace_in_file "crates/web/src/assets/js/app.js"
replace_in_file "crates/web/src/assets/js/page-images.js"
replace_in_file "crates/web/src/assets/js/page-hooks.js"

echo ""
echo "✅ Branding replacement complete!"
echo ""
echo "📋 Summary:"
echo "   - Replaced user-visible 'Moltis' with 'ClawMaster'"
echo "   - Preserved package names (clawmaster-tools, etc.)"
echo "   - Preserved code identifiers and paths"
echo ""
echo "🔍 To verify changes:"
echo "   git diff"
echo ""
echo "♻️  To restore backups:"
echo "   find . -name '*.bak' -exec sh -c 'mv \"\$1\" \"\${1%.bak}\"' _ {} \\;"
