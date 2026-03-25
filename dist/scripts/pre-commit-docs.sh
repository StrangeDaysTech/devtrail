#!/bin/bash

# =============================================================================
# DevTrail - Pre-commit Hook for Documentation Validation
# https://strangedays.tech
# =============================================================================
#
# Installation:
#   cp scripts/pre-commit-docs.sh .git/hooks/pre-commit
#   chmod +x .git/hooks/pre-commit
#
# Or with npm/husky:
#   npx husky add .husky/pre-commit "bash scripts/pre-commit-docs.sh"
#
# =============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Error counters
ERRORS=0
WARNINGS=0

echo "🔍 Validating documentation..."
echo ""

# =============================================================================
# 1. Get staged documentation files
# =============================================================================

STAGED_DOCS=$(git diff --cached --name-only --diff-filter=ACM | grep -E "^\.devtrail/.*\.md$" || true)

if [ -z "$STAGED_DOCS" ]; then
    echo -e "${GREEN}✓ No staged markdown documents to validate${NC}"
    exit 0
fi

echo "Files to validate:"
echo "$STAGED_DOCS"
echo ""

# =============================================================================
# 2. Validate file naming convention
# =============================================================================

echo "📋 Validating file naming convention..."

# Valid pattern: TYPE-YYYY-MM-DD-NNN-description.md
# Allowed types: ADR, REQ, TES, OPS, INC, TDE, AILOG, AIDEC, ETH, DOC, SEC, MCARD, SBOM, DPIA
VALID_PATTERN="^(ADR|REQ|TES|OPS|INC|TDE|AILOG|AIDEC|ETH|DOC|SEC|MCARD|SBOM|DPIA)-[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]{3}-[a-z0-9-]+\.md$"

# Files excluded from naming validation
EXCLUDED_FILES="PRINCIPLES.md|DOCUMENTATION-POLICY.md|AGENT-RULES.md|TEMPLATE-.*\.md|README.md|QUICK-REFERENCE.md|INDEX.md|\.gitkeep"

for file in $STAGED_DOCS; do
    filename=$(basename "$file")

    # Skip excluded files
    if echo "$filename" | grep -qE "$EXCLUDED_FILES"; then
        echo -e "  ${YELLOW}⊘ Excluded: $filename${NC}"
        continue
    fi

    # Validate naming convention
    if ! echo "$filename" | grep -qE "$VALID_PATTERN"; then
        echo -e "  ${RED}✗ Invalid naming: $filename${NC}"
        echo -e "    Expected: [TYPE]-[YYYY-MM-DD]-[NNN]-[description].md"
        echo -e "    Valid types: ADR, REQ, TES, OPS, INC, TDE, AILOG, AIDEC, ETH, DOC, SEC, MCARD, SBOM, DPIA"
        ((ERRORS++))
    else
        echo -e "  ${GREEN}✓ $filename${NC}"
    fi
done

echo ""

# =============================================================================
# 3. Validate YAML front-matter
# =============================================================================

echo "📋 Validating metadata (front-matter)..."

REQUIRED_FIELDS="id|title|status|created"

for file in $STAGED_DOCS; do
    filename=$(basename "$file")

    # Skip excluded files
    if echo "$filename" | grep -qE "$EXCLUDED_FILES"; then
        continue
    fi

    # Verify front-matter exists
    if ! head -1 "$file" | grep -q "^---"; then
        echo -e "  ${RED}✗ Missing YAML front-matter: $filename${NC}"
        ((ERRORS++))
        continue
    fi

    # Extract front-matter (between --- and ---)
    FRONTMATTER=$(sed -n '/^---$/,/^---$/p' "$file" | sed '1d;$d')

    if [ -z "$FRONTMATTER" ]; then
        echo -e "  ${RED}✗ Empty front-matter: $filename${NC}"
        ((ERRORS++))
        continue
    fi

    # Verify required fields
    MISSING_FIELDS=""
    for field in $(echo $REQUIRED_FIELDS | tr '|' ' '); do
        if ! echo "$FRONTMATTER" | grep -q "^$field:"; then
            MISSING_FIELDS="$MISSING_FIELDS $field"
        fi
    done

    if [ -n "$MISSING_FIELDS" ]; then
        echo -e "  ${RED}✗ Missing fields in $filename:$MISSING_FIELDS${NC}"
        ((ERRORS++))
    else
        echo -e "  ${GREEN}✓ $filename - metadata complete${NC}"
    fi
done

echo ""

# =============================================================================
# 4. Validate no sensitive information
# =============================================================================

echo "🔒 Checking for sensitive information..."

SENSITIVE_PATTERNS="password|api_key|apikey|secret|token|private_key|credentials|passwd"

for file in $STAGED_DOCS; do
    # Search for sensitive patterns (case insensitive)
    MATCHES=$(grep -inE "$SENSITIVE_PATTERNS" "$file" 2>/dev/null | head -5 || true)

    if [ -n "$MATCHES" ]; then
        echo -e "  ${YELLOW}⚠ Possible sensitive information in $file:${NC}"
        echo "$MATCHES" | while read line; do
            echo -e "    ${YELLOW}$line${NC}"
        done
        ((WARNINGS++))
    fi
done

if [ $WARNINGS -eq 0 ]; then
    echo -e "  ${GREEN}✓ No sensitive information detected${NC}"
fi

echo ""

# =============================================================================
# 5. Validate valid status
# =============================================================================

echo "📋 Validating document statuses..."

VALID_STATUSES="draft|proposed|accepted|deprecated|superseded|investigating|identified|monitoring|resolved|closed|under_review|approved|rejected|requires_changes|implemented|in_progress|wont_fix"

for file in $STAGED_DOCS; do
    filename=$(basename "$file")

    # Skip excluded files
    if echo "$filename" | grep -qE "$EXCLUDED_FILES"; then
        continue
    fi

    # Extract status from front-matter
    STATUS=$(grep "^status:" "$file" 2>/dev/null | head -1 | sed 's/status: *//' | tr -d '\r' || true)

    if [ -n "$STATUS" ]; then
        # Validate status is valid
        if ! echo "$STATUS" | grep -qE "^($VALID_STATUSES)$"; then
            echo -e "  ${RED}✗ Invalid status in $filename: '$STATUS'${NC}"
            echo -e "    Valid statuses: draft, proposed, accepted, deprecated, superseded"
            ((ERRORS++))
        else
            echo -e "  ${GREEN}✓ $filename - status: $STATUS${NC}"
        fi
    fi
done

echo ""

# =============================================================================
# 6. Validate risk_level / review_required cross-check
# =============================================================================

echo "📋 Validating risk_level and review_required..."

for file in $STAGED_DOCS; do
    filename=$(basename "$file")

    # Skip excluded files
    if echo "$filename" | grep -qE "$EXCLUDED_FILES"; then
        continue
    fi

    FRONTMATTER=$(sed -n '/^---$/,/^---$/p' "$file" | sed '1d;$d')

    if [ -z "$FRONTMATTER" ]; then
        continue
    fi

    # Check risk_level high/critical requires review_required: true
    RISK_LEVEL=$(echo "$FRONTMATTER" | grep "^risk_level:" | head -1 | sed 's/risk_level: *//' | tr -d '\r' || true)
    REVIEW_REQUIRED=$(echo "$FRONTMATTER" | grep "^review_required:" | head -1 | sed 's/review_required: *//' | tr -d '\r' || true)

    if [ "$RISK_LEVEL" = "high" ] || [ "$RISK_LEVEL" = "critical" ]; then
        if [ "$REVIEW_REQUIRED" != "true" ]; then
            echo -e "  ${RED}✗ $filename: risk_level is '$RISK_LEVEL' but review_required is not true${NC}"
            ((ERRORS++))
        fi
    fi
done

echo ""

# =============================================================================
# 7. Validate id matches filename prefix
# =============================================================================

echo "📋 Validating id matches filename..."

for file in $STAGED_DOCS; do
    filename=$(basename "$file")

    # Skip excluded files
    if echo "$filename" | grep -qE "$EXCLUDED_FILES"; then
        continue
    fi

    FRONTMATTER=$(sed -n '/^---$/,/^---$/p' "$file" | sed '1d;$d')

    if [ -z "$FRONTMATTER" ]; then
        continue
    fi

    # Extract id from frontmatter
    DOC_ID=$(echo "$FRONTMATTER" | grep "^id:" | head -1 | sed 's/id: *//' | tr -d '\r' || true)

    if [ -n "$DOC_ID" ]; then
        # Expected: filename starts with the id value
        EXPECTED_PREFIX="$DOC_ID"
        FILENAME_PREFIX=$(echo "$filename" | sed 's/\(.*-[0-9]\{3\}\).*/\1/')

        if [ "$DOC_ID" != "$FILENAME_PREFIX" ]; then
            echo -e "  ${RED}✗ $filename: id '$DOC_ID' does not match filename prefix '$FILENAME_PREFIX'${NC}"
            ((ERRORS++))
        else
            echo -e "  ${GREEN}✓ $filename - id matches filename${NC}"
        fi
    fi
done

echo ""

# =============================================================================
# 8. Validate review_required for specific types
# =============================================================================

echo "📋 Validating review_required for governance types..."

REVIEW_REQUIRED_TYPES="^(ETH|ADR|SEC|MCARD|DPIA)-"

for file in $STAGED_DOCS; do
    filename=$(basename "$file")

    # Skip excluded files
    if echo "$filename" | grep -qE "$EXCLUDED_FILES"; then
        continue
    fi

    # Only check specific types
    if ! echo "$filename" | grep -qE "$REVIEW_REQUIRED_TYPES"; then
        continue
    fi

    FRONTMATTER=$(sed -n '/^---$/,/^---$/p' "$file" | sed '1d;$d')

    if [ -z "$FRONTMATTER" ]; then
        continue
    fi

    REVIEW_REQUIRED=$(echo "$FRONTMATTER" | grep "^review_required:" | head -1 | sed 's/review_required: *//' | tr -d '\r' || true)

    if [ "$REVIEW_REQUIRED" != "true" ]; then
        DOC_TYPE=$(echo "$filename" | sed 's/-.*//')
        echo -e "  ${RED}✗ $filename: type '$DOC_TYPE' requires review_required: true${NC}"
        ((ERRORS++))
    else
        echo -e "  ${GREEN}✓ $filename - review_required is set${NC}"
    fi
done

echo ""

# =============================================================================
# 9. Validate observability tag has matching section
# =============================================================================

echo "📋 Validating observability tag consistency..."

for file in $STAGED_DOCS; do
    filename=$(basename "$file")

    # Skip excluded files
    if echo "$filename" | grep -qE "$EXCLUDED_FILES"; then
        continue
    fi

    FRONTMATTER=$(sed -n '/^---$/,/^---$/p' "$file" | sed '1d;$d')

    if [ -z "$FRONTMATTER" ]; then
        continue
    fi

    # Check if tags contain observabilidad
    if echo "$FRONTMATTER" | grep -qi "observabilidad"; then
        # Check body (after frontmatter) for an Observab header
        BODY=$(sed -n '/^---$/,/^---$/!p' "$file" | tail -n +2)
        if ! echo "$BODY" | grep -qiE "^#+.*Observab"; then
            echo -e "  ${YELLOW}⚠ $filename: has tag 'observabilidad' but no Observability section header${NC}"
            ((WARNINGS++))
        fi
    fi
done

echo ""

# =============================================================================
# 10. Validate related: references exist
# =============================================================================

echo "📋 Validating related document references..."

for file in $STAGED_DOCS; do
    filename=$(basename "$file")

    # Skip excluded files
    if echo "$filename" | grep -qE "$EXCLUDED_FILES"; then
        continue
    fi

    FRONTMATTER=$(sed -n '/^---$/,/^---$/p' "$file" | sed '1d;$d')

    if [ -z "$FRONTMATTER" ]; then
        continue
    fi

    # Extract related items (lines starting with " - " after "related:")
    RELATED=$(echo "$FRONTMATTER" | sed -n '/^related:/,/^[a-z]/p' | grep "^ *-" | sed 's/^ *- *//' | tr -d '\r')

    for ref in $RELATED; do
        # Skip empty refs
        if [ -z "$ref" ] || [ "$ref" = "[]" ]; then
            continue
        fi

        # Search for a file matching this reference
        FOUND=$(find .devtrail -name "${ref}*" -not -path "*/templates/*" 2>/dev/null | head -1)
        if [ -z "$FOUND" ]; then
            echo -e "  ${YELLOW}⚠ $filename: related document '$ref' not found in .devtrail/${NC}"
            ((WARNINGS++))
        fi
    done
done

echo ""

# =============================================================================
# 11. Detect code changes without same-day AILOG (warning)
# =============================================================================

echo "📋 Checking for code changes without AILOG..."

TODAY=$(date +%Y-%m-%d)

# Check if there are staged code files (non-.devtrail, non-.md)
STAGED_CODE=$(git diff --cached --name-only --diff-filter=ACM | grep -vE "^\.devtrail/|\.md$|\.yml$|\.json$|\.gitkeep$" || true)

if [ -n "$STAGED_CODE" ]; then
    # Check if there's an AILOG for today
    TODAY_AILOG=$(find .devtrail -name "AILOG-${TODAY}-*.md" -not -path "*/templates/*" 2>/dev/null | head -1)

    if [ -z "$TODAY_AILOG" ]; then
        # Also check staged files for an AILOG
        STAGED_AILOG=$(echo "$STAGED_DOCS" | grep "AILOG-${TODAY}" || true)

        if [ -z "$STAGED_AILOG" ]; then
            echo -e "  ${YELLOW}⚠ Code changes detected but no AILOG for today ($TODAY)${NC}"
            echo -e "    Consider creating one with: ./scripts/devtrail-new.sh ailog"
            ((WARNINGS++))
        fi
    fi
fi

echo ""

# =============================================================================
# 12. Run markdownlint if available
# =============================================================================

if command -v markdownlint &> /dev/null; then
    echo "📝 Running markdownlint..."

    for file in $STAGED_DOCS; do
        if ! markdownlint "$file" 2>/dev/null; then
            echo -e "  ${YELLOW}⚠ markdownlint warnings in $file${NC}"
            ((WARNINGS++))
        fi
    done
else
    echo -e "${YELLOW}ℹ markdownlint not installed, skipping format validation${NC}"
    echo -e "  Install with: npm install -g markdownlint-cli"
fi

echo ""

# =============================================================================
# 13. Summary and result
# =============================================================================

echo "═══════════════════════════════════════════════════════════════════════════"
echo "📊 Validation summary"
echo "═══════════════════════════════════════════════════════════════════════════"

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}✗ Errors found: $ERRORS${NC}"
fi

if [ $WARNINGS -gt 0 ]; then
    echo -e "${YELLOW}⚠ Warnings: $WARNINGS${NC}"
fi

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✓ All validations passed${NC}"
fi

echo ""

# Fail if there are errors
if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}❌ Commit blocked due to documentation errors${NC}"
    echo -e "   Fix the errors before committing"
    exit 1
fi

# Warn but allow if there are only warnings
if [ $WARNINGS -gt 0 ]; then
    echo -e "${YELLOW}⚠ Commit allowed with warnings${NC}"
    echo -e "   Consider reviewing the warnings"
fi

echo -e "${GREEN}✅ Documentation validation completed${NC}"
exit 0
