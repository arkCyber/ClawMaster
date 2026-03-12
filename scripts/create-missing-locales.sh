#!/bin/bash
# Script to create missing locale directories and files for 16 language support

set -e

LOCALES_DIR="/Users/arksong/ClawMaster/crates/web/src/assets/js/locales"
EXISTING_LOCALES=("en" "fr" "zh")
NEW_LOCALES=("es" "de" "ja" "ko" "ru" "pt" "it" "ar" "hi" "tr" "nl" "pl" "vi")

echo "🌐 Creating missing locale directories and files..."

# Get list of all translation files from English
cd "$LOCALES_DIR/en"
FILES=(*.js)

for locale in "${NEW_LOCALES[@]}"; do
    echo ""
    echo "📁 Creating locale: $locale"
    
    # Create directory if it doesn't exist
    mkdir -p "$LOCALES_DIR/$locale"
    
    # Copy each file from English as template
    for file in "${FILES[@]}"; do
        target_file="$LOCALES_DIR/$locale/$file"
        
        if [ -f "$target_file" ]; then
            echo "   ✓ $file already exists"
        else
            # Copy from English and add a comment
            echo "   ✅ Creating $file"
            cat > "$target_file" << 'EOF'
// ── Placeholder translations ────────────────────────────────
// TODO: Translate to native language
// This file uses English as fallback until proper translations are added

EOF
            # Append the English content
            cat "$LOCALES_DIR/en/$file" >> "$target_file"
        fi
    done
done

echo ""
echo "✅ All locale directories created!"
echo ""
echo "📝 Note: New locales use English as fallback."
echo "   Proper translations should be added by native speakers."
