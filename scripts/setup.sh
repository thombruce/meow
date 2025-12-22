#!/bin/bash

# Release Setup Script for Catfood
# This script sets up the release dependencies and configuration

set -e

echo "🚀 Setting up Catfood Release Environment..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust first."
    exit 1
fi

# Install required tools for releases
echo "📦 Installing cargo-release..."
cargo install cargo-release --quiet

echo "📝 Installing git-cliff for changelog generation..."
cargo install git-cliff --quiet

echo "🔧 Setting up git hooks..."
# Ensure we have git hooks directory
mkdir -p .git/hooks

# Create pre-push hook to check formatting and clippy
cat > .git/hooks/pre-push << 'EOF'
#!/bin/bash

echo "🔍 Running pre-push checks..."

# Check formatting
echo "Checking formatting..."
if ! cargo fmt --all --check; then
    echo "❌ Code formatting issues found. Run 'cargo fmt --all' to fix."
    exit 1
fi

# Run clippy
echo "Running clippy..."
if ! cargo clippy --workspace -- -D warnings; then
    echo "❌ Clippy warnings found. Please fix them before pushing."
    exit 1
fi

echo "✅ Pre-push checks passed!"
EOF

chmod +x .git/hooks/pre-push

echo "✅ Setup complete!"

# Test installation
echo "🧪 Testing setup..."
cargo release --version
git-cliff --version

echo ""
echo "🎉 Catfood release environment is ready!"
echo ""
echo "Next steps:"
echo "1. Make your code changes using conventional commit messages"
echo "2. Run 'cargo release patch --execute' to bump version"
echo "3. Push and tag: 'git push --tags'"
echo "4. GitHub Actions will handle the rest!"
echo ""
echo "Examples of conventional commits:"
echo "  feat: add new weather component"
echo "  fix: memory leak in CPU monitoring"
echo "  docs: update configuration examples"
echo "  refactor: improve component performance"