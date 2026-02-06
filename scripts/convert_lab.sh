#!/bin/bash

###############################################################################
# Lab Conversion Script
#
# Usage: ./scripts/convert_lab.sh labs/NN-lab-name
#
# This script helps bootstrap converting an incomplete lab (just main.rs)
# into a complete teaching lab (lib.rs, solution.rs, tests, enhanced README)
#
# What it does:
# 1. Checks the lab structure
# 2. Creates placeholder files (lib.rs, solution.rs, integration_test.rs)
# 3. Copies the existing main.rs content to solution.rs for reference
# 4. Guides you through manual conversion steps
#
# What you need to do:
# 1. Edit lib.rs - extract function signatures and add todos
# 2. Edit solution.rs - keep implementations, add exhaustive docs
# 3. Edit tests/integration_test.rs - write comprehensive tests
# 4. Edit README.md - add pedagogical structure
###############################################################################

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEMPLATES_DIR="${REPO_ROOT}/templates"
SCRIPTS_DIR="${REPO_ROOT}/scripts"

# Helpers
print_header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Main logic
if [ $# -eq 0 ]; then
    print_error "Usage: $0 labs/NN-lab-name"
    echo ""
    echo "Example:"
    echo "  $0 labs/11-control-flow"
    exit 1
fi

LAB_PATH="$1"
LAB_NAME=$(basename "$LAB_PATH")
LAB_NUM="${LAB_NAME%%-*}"

if [ ! -d "$LAB_PATH" ]; then
    print_error "Lab directory not found: $LAB_PATH"
    exit 1
fi

if [ ! -f "$LAB_PATH/src/main.rs" ]; then
    print_error "No main.rs found in $LAB_PATH/src/"
    exit 1
fi

print_header "Lab Conversion Bootstrap: $LAB_NAME"
echo ""

# Step 1: Analyze existing code
print_info "Step 1: Analyzing existing main.rs..."
MAIN_LINES=$(wc -l < "$LAB_PATH/src/main.rs")
MAIN_FUNCS=$(grep -c "^fn " "$LAB_PATH/src/main.rs" 2>/dev/null || echo "0")
print_success "Found main.rs ($MAIN_LINES lines, ~$MAIN_FUNCS functions)"
echo ""

# Step 2: Create lib.rs if it doesn't exist
if [ -f "$LAB_PATH/src/lib.rs" ]; then
    print_warning "lib.rs already exists, skipping creation"
else
    print_info "Step 2: Creating lib.rs template..."
    cat "$TEMPLATES_DIR/lib.rs.template" > "$LAB_PATH/src/lib.rs"
    print_success "Created lib.rs - you need to:"
    echo "  1. Replace [Topic Name] with actual topic"
    echo "  2. Extract function signatures from main.rs"
    echo "  3. Add doc comments and todo! placeholders"
    echo ""
fi

# Step 3: Create solution.rs if it doesn't exist
if [ -f "$LAB_PATH/src/solution.rs" ]; then
    print_warning "solution.rs already exists, skipping creation"
else
    print_info "Step 3: Creating solution.rs template..."
    cp "$TEMPLATES_DIR/solution.rs.template" "$LAB_PATH/src/solution.rs"
    print_success "Created solution.rs - you need to:"
    echo "  1. Copy implementations from main.rs"
    echo "  2. Add exhaustive function documentation"
    echo "  3. Include ownership & borrowing analysis"
    echo "  4. Add memory layout diagrams"
    echo ""
fi

# Step 4: Create tests directory if it doesn't exist
if [ ! -d "$LAB_PATH/tests" ]; then
    mkdir -p "$LAB_PATH/tests"
    print_success "Created tests/ directory"
fi

# Step 5: Create integration_test.rs if it doesn't exist
if [ -f "$LAB_PATH/tests/integration_test.rs" ]; then
    print_warning "integration_test.rs already exists, skipping creation"
else
    print_info "Step 4: Creating tests/integration_test.rs template..."
    cp "$TEMPLATES_DIR/integration_test.rs.template" "$LAB_PATH/tests/integration_test.rs"
    print_success "Created integration_test.rs - you need to:"
    echo "  1. Write unit tests for each function (happy path + edge cases)"
    echo "  2. Write integration tests (functions working together)"
    echo "  3. Write property-based tests (invariants)"
    echo "  4. Add explanatory comments to each test"
    echo ""
fi

# Step 6: Create lib module file to make main.rs a module
if ! grep -q "^mod solution;" "$LAB_PATH/src/lib.rs"; then
    echo "" >> "$LAB_PATH/src/lib.rs"
    echo "pub mod solution;" >> "$LAB_PATH/src/lib.rs"
    print_success "Added 'pub mod solution;' to lib.rs"
fi

# Step 7: Summary and next steps
print_header "Conversion Bootstrap Complete!"
echo ""
print_success "Files created in $LAB_PATH:"
echo "  ✅ src/lib.rs (exercise scaffolding)"
echo "  ✅ src/solution.rs (exhaustive teaching)"
echo "  ✅ tests/integration_test.rs (comprehensive tests)"
echo "  ✅ src/main.rs (already exists - keep it)"
echo "  ✅ README.md (already exists - enhance it)"
echo ""

print_header "Your Next Steps (in order):"
echo ""
echo "1️⃣  Edit src/lib.rs:"
echo "   nano $LAB_PATH/src/lib.rs"
echo "   - Replace template placeholders"
echo "   - Extract function signatures from main.rs"
echo "   - Add doc comments with hints"
echo "   - Add todo!() placeholders"
echo ""

echo "2️⃣  Edit src/solution.rs:"
echo "   nano $LAB_PATH/src/solution.rs"
echo "   - Copy working implementations from main.rs"
echo "   - Add exhaustive documentation:"
echo "     * What this function does"
echo "     * Parameter explanations (with symbol breakdown)"
echo "     * Return value explanation"
echo "     * Examples"
echo "     * Ownership & borrowing analysis (if relevant)"
echo "     * Memory layout diagrams (if complex)"
echo "     * Complexity analysis"
echo ""

echo "3️⃣  Edit tests/integration_test.rs:"
echo "   nano $LAB_PATH/tests/integration_test.rs"
echo "   - Write 20-40 comprehensive tests"
echo "   - Include edge cases, boundaries, properties"
echo "   - Add explanatory comments to each test"
echo ""

echo "4️⃣  Enhance README.md:"
echo "   nano $LAB_PATH/README.md"
echo "   - Use templates/README.md.template as reference"
echo "   - Add plain English explanation"
echo "   - Add concept descriptions"
echo "   - Add syntax examples"
echo "   - Add comparisons to other languages"
echo "   - Add common mistakes"
echo ""

echo "5️⃣  Test your work:"
echo "   cd $LAB_PATH && cargo test"
echo "   cd $LAB_PATH && cargo run"
echo "   cd $LAB_PATH && cargo check"
echo ""

print_info "Reference templates available in:"
echo "   templates/lib.rs.template"
echo "   templates/solution.rs.template"
echo "   templates/integration_test.rs.template"
echo "   templates/README.md.template"
echo ""

print_info "Full conversion guide available in:"
echo "   LAB_CONVERSION_GUIDE.md"
echo ""

print_success "Ready to start converting! Good luck! 🦀"
