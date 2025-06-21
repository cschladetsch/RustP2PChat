#!/bin/bash
# Setup script to ensure clean commits without AI mentions

echo "🔧 Setting up clean commit environment..."

# 1. Create global hooks directory
mkdir -p ~/.config/git/hooks

# 2. Create prepare-commit-msg hook
cat > ~/.config/git/hooks/prepare-commit-msg << 'EOF'
#!/bin/bash
# Remove AI mentions from commit messages
COMMIT_MSG_FILE=$1
if [ -f "$COMMIT_MSG_FILE" ]; then
    sed -i '/Co-Authored-By:.*Claude/d' "$COMMIT_MSG_FILE"
    sed -i '/Co-Authored-By:.*anthropic/d' "$COMMIT_MSG_FILE"
    sed -i '/Generated with.*Claude/d' "$COMMIT_MSG_FILE"
    sed -i 's/with Claude//gi' "$COMMIT_MSG_FILE"
    sed -i 's/by Claude//gi' "$COMMIT_MSG_FILE"
fi
exit 0
EOF

# 3. Create commit-msg hook
cat > ~/.config/git/hooks/commit-msg << 'EOF'
#!/bin/bash
# Final check for AI mentions
COMMIT_MSG_FILE=$1
if grep -iE "(claude|anthropic|co-authored-by:.*claude|ai assistant)" "$COMMIT_MSG_FILE" > /dev/null 2>&1; then
    echo "❌ Error: Commit message contains AI references. Please remove them."
    exit 1
fi
exit 0
EOF

# 4. Make hooks executable
chmod +x ~/.config/git/hooks/*

# 5. Set global hooks path
git config --global core.hooksPath ~/.config/git/hooks

# 6. Create safe commit alias
git config --global alias.safe-commit '!f() { git commit -m "$(echo "$1" | sed -E "s/(claude|anthropic|ai assistant)//gi")" "${@:2}"; }; f'

echo "✅ Setup complete! Your commits will now be automatically cleaned."
echo ""
echo "Usage:"
echo "  - Regular commits will auto-clean AI mentions"
echo "  - Use 'git safe-commit' for extra safety"
echo "  - Commits with AI mentions will be rejected"