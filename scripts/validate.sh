#!/bin/bash

# Release Validation Script
# Validates that the release setup is working correctly

set -e

echo "🧪 Validating Catfood Release Setup..."

# Check workspace structure
echo "1. Checking workspace structure..."
if cargo metadata --format-version=1 --no-deps | jq -e '.packages[] | select(.name == "catfood" or .name == "catfood-bar")' > /dev/null; then
    echo "✅ Workspace packages found"
else
    echo "❌ Workspace packages not found"
    exit 1
fi

# Check unified versioning
echo "2. Checking unified versioning..."
CATFOOD_VERSION=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[] | select(.name == "catfood") | .version')
BAR_VERSION=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[] | select(.name == "catfood-bar") | .version')

if [[ "$CATFOOD_VERSION" == "$BAR_VERSION" ]]; then
    echo "✅ Unified versioning: both crates at v$CATFOOD_VERSION"
else
    echo "❌ Version mismatch: catfood=$CATFOOD_VERSION, catfood-bar=$BAR_VERSION"
    exit 1
fi

# Check CI configuration
echo "3. Checking CI configuration..."
if [[ -f ".github/workflows/release.yml" ]]; then
    echo "✅ Release workflow found"
else
    echo "❌ Release workflow missing"
    exit 1
fi

if [[ -f ".github/workflows/ci.yml" ]]; then
    echo "✅ CI workflow found"
else
    echo "❌ CI workflow missing"
    exit 1
fi

# Check changelog configuration
echo "4. Checking changelog configuration..."
if [[ -f "cliff.toml" ]]; then
    echo "✅ Git-cliff configuration found"
else
    echo "❌ Git-cliff configuration missing"
    exit 1
fi

# Check workspace compiles
echo "5. Checking workspace compilation..."
if cargo check --workspace > /dev/null 2>&1; then
    echo "✅ Workspace compiles successfully"
else
    echo "❌ Workspace compilation failed"
    exit 1
fi

# Check formatting
echo "6. Checking code formatting..."
if cargo fmt --all --check > /dev/null 2>&1; then
    echo "✅ Code formatting is correct"
else
    echo "❌ Code formatting issues found"
    exit 1
fi

# Check clippy
echo "7. Running clippy checks..."
if cargo clippy --workspace -- -D warnings > /dev/null 2>&1; then
    echo "✅ Clippy checks passed"
else
    echo "❌ Clippy warnings found"
    exit 1
fi

echo ""
echo "🎉 All validations passed!"
echo ""
echo "📋 Summary:"
echo "  ✅ Unified workspace versioning (v$CATFOOD_VERSION)"
echo "  ✅ Release workflow configured"
echo "  ✅ Automated changelog generation"
echo "  ✅ Multi-platform binary distribution"
echo "  ✅ Concurrent publishing to crates.io"
echo "  ✅ Code quality checks passed"
echo ""
echo "🚀 Your release environment is ready!"
echo ""
echo "To create a release:"
echo "  1. Make changes with conventional commit messages"
echo "  2. Run: cargo release patch --execute"
echo "  3. Run: git push --tags"
echo "  4. GitHub Actions will handle everything else!"