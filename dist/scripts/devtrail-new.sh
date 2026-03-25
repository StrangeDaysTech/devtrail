#!/bin/bash
# devtrail-new.sh - Create DevTrail documentation manually
# Usage: ./scripts/devtrail-new.sh [type] [title] [slug]
# Types: ailog, aidec, adr, eth, req, tes, inc, tde, sec, mcard, sbom, dpia
#
# Examples:
#   ./scripts/devtrail-new.sh                              # fully interactive
#   ./scripts/devtrail-new.sh ailog                        # asks for title and slug
#   ./scripts/devtrail-new.sh ailog "Implement auth"       # slug derived from title
#   ./scripts/devtrail-new.sh ailog "Implement auth" "implement-auth"  # no prompts

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DEVTRAIL_DIR=".devtrail"
CONFIG_FILE="$DEVTRAIL_DIR/config.yml"
TODAY=$(date +%Y-%m-%d)

# Document locations
declare -A DOC_PATHS=(
    ["ailog"]="$DEVTRAIL_DIR/07-ai-audit/agent-logs"
    ["aidec"]="$DEVTRAIL_DIR/07-ai-audit/decisions"
    ["eth"]="$DEVTRAIL_DIR/07-ai-audit/ethical-reviews"
    ["adr"]="$DEVTRAIL_DIR/02-design/decisions"
    ["req"]="$DEVTRAIL_DIR/01-requirements"
    ["tes"]="$DEVTRAIL_DIR/04-testing"
    ["inc"]="$DEVTRAIL_DIR/05-operations/incidents"
    ["tde"]="$DEVTRAIL_DIR/06-evolution/technical-debt"
    ["sec"]="$DEVTRAIL_DIR/08-security"
    ["mcard"]="$DEVTRAIL_DIR/09-ai-models"
    ["sbom"]="$DEVTRAIL_DIR/07-ai-audit"
    ["dpia"]="$DEVTRAIL_DIR/07-ai-audit/ethical-reviews"
)

declare -A DOC_NAMES=(
    ["ailog"]="AI Action Log"
    ["aidec"]="AI Decision"
    ["eth"]="Ethical Review"
    ["adr"]="Architecture Decision Record"
    ["req"]="Requirement"
    ["tes"]="Test Plan"
    ["inc"]="Incident Post-mortem"
    ["tde"]="Technical Debt"
    ["sec"]="Security Assessment"
    ["mcard"]="Model/System Card"
    ["sbom"]="Software Bill of Materials"
    ["dpia"]="Data Protection Impact Assessment"
)

# Check if .devtrail exists
if [ ! -d "$DEVTRAIL_DIR" ]; then
    echo -e "${RED}Error: .devtrail directory not found.${NC}"
    echo "Run this script from the project root with DevTrail installed."
    exit 1
fi

# Get language from config
get_language() {
    if [ -f "$CONFIG_FILE" ]; then
        lang=$(grep "^language:" "$CONFIG_FILE" | cut -d: -f2 | tr -d ' ')
        echo "${lang:-en}"
    else
        echo "en"
    fi
}

# Get template path
get_template_path() {
    local type=$1
    local lang=$(get_language)
    local type_upper=$(echo "$type" | tr '[:lower:]' '[:upper:]')
    
    if [ "$lang" = "es" ]; then
        echo "$DEVTRAIL_DIR/templates/i18n/es/TEMPLATE-$type_upper.md"
    else
        echo "$DEVTRAIL_DIR/templates/TEMPLATE-$type_upper.md"
    fi
}

# Get next sequence number
get_next_seq() {
    local type=$1
    local path="${DOC_PATHS[$type]}"
    local type_upper=$(echo "$type" | tr '[:lower:]' '[:upper:]')
    
    local count=$(ls "$path"/${type_upper}-${TODAY}-*.md 2>/dev/null | wc -l)
    printf "%03d" $((count + 1))
}

# Show header
show_header() {
    echo ""
    echo -e "${BLUE}╔══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║  DevTrail New                                                    ║${NC}"
    echo -e "${BLUE}╠══════════════════════════════════════════════════════════════════╣${NC}"
}

# Show git status summary
show_git_status() {
    echo -e "${BLUE}║                                                                   ║${NC}"
    echo -e "${BLUE}║  📊 Git Status:                                                   ║${NC}"
    
    if git rev-parse --git-dir > /dev/null 2>&1; then
        local modified=$(git diff --name-only 2>/dev/null | wc -l)
        local staged=$(git diff --cached --name-only 2>/dev/null | wc -l)
        echo -e "${BLUE}║     • Modified files: $modified                                     ║${NC}"
        echo -e "${BLUE}║     • Staged files: $staged                                         ║${NC}"
    else
        echo -e "${BLUE}║     • Not a git repository                                       ║${NC}"
    fi
}

# Type selection
select_type() {
    echo -e "${BLUE}║                                                                   ║${NC}"
    echo -e "${BLUE}║  📋 Available document types:                                     ║${NC}"
    echo -e "${BLUE}║     1. ailog - AI Action Log                                      ║${NC}"
    echo -e "${BLUE}║     2. aidec - AI Decision                                        ║${NC}"
    echo -e "${BLUE}║     3. adr   - Architecture Decision Record                       ║${NC}"
    echo -e "${BLUE}║     4. eth   - Ethical Review                                     ║${NC}"
    echo -e "${BLUE}║     5. req   - Requirement                                        ║${NC}"
    echo -e "${BLUE}║     6. tes   - Test Plan                                          ║${NC}"
    echo -e "${BLUE}║     7. inc   - Incident Post-mortem                               ║${NC}"
    echo -e "${BLUE}║     8. tde   - Technical Debt                                     ║${NC}"
    echo -e "${BLUE}║     9. sec   - Security Assessment                                ║${NC}"
    echo -e "${BLUE}║    10. mcard - Model/System Card                                  ║${NC}"
    echo -e "${BLUE}║    11. sbom  - Software Bill of Materials                         ║${NC}"
    echo -e "${BLUE}║    12. dpia  - Data Protection Impact Assessment                  ║${NC}"
    echo -e "${BLUE}║                                                                   ║${NC}"
    echo -e "${BLUE}╚══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    read -p "Select type (1-12 or name): " type_input

    case $type_input in
        1|ailog) echo "ailog" ;;
        2|aidec) echo "aidec" ;;
        3|adr) echo "adr" ;;
        4|eth) echo "eth" ;;
        5|req) echo "req" ;;
        6|tes) echo "tes" ;;
        7|inc) echo "inc" ;;
        8|tde) echo "tde" ;;
        9|sec) echo "sec" ;;
        10|mcard) echo "mcard" ;;
        11|sbom) echo "sbom" ;;
        12|dpia) echo "dpia" ;;
        *) echo "" ;;
    esac
}

# Create document
create_document() {
    local type=$1
    local title=$2
    local description=$3
    
    local type_upper=$(echo "$type" | tr '[:lower:]' '[:upper:]')
    local seq=$(get_next_seq "$type")
    local path="${DOC_PATHS[$type]}"
    local template=$(get_template_path "$type")
    
    # Create description slug
    local slug=$(echo "$description" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | tr -cd '[:alnum:]-')
    slug=${slug:0:50}  # Limit length
    
    local filename="${type_upper}-${TODAY}-${seq}-${slug}.md"
    local filepath="$path/$filename"
    
    # Check template exists
    if [ ! -f "$template" ]; then
        echo -e "${RED}Error: Template not found: $template${NC}"
        exit 1
    fi
    
    # Create document from template
    mkdir -p "$path"
    
    # Replace placeholders
    sed -e "s/YYYY-MM-DD-NNN/${TODAY}-${seq}/g" \
        -e "s/YYYY-MM-DD/${TODAY}/g" \
        -e "s/\[Descriptive title.*\]/${title}/g" \
        -e "s/\[Decision title\]/${title}/g" \
        -e "s/\[Architectural decision title\]/${title}/g" \
        -e "s/\[Title\]/${title}/g" \
        -e "s/\[agent-name-v1.0\]/manual-user/g" \
        "$template" > "$filepath"
    
    echo ""
    echo -e "${GREEN}✅ Document created:${NC}"
    echo "   $filepath"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo "   1. Edit the document to fill in details"
    echo "   2. Review and update risk_level if needed"
    echo "   3. Commit with: git add $filepath"
}

# Main
main() {
    local args_provided=$#
    local type=$1
    local title=$2
    local description=$3

    show_header
    show_git_status

    # If type not provided, ask
    if [ -z "$type" ]; then
        type=$(select_type)
    fi

    # Validate type
    if [ -z "${DOC_PATHS[$type]}" ]; then
        echo -e "${RED}Error: Invalid type '$type'${NC}"
        echo "Valid types: ailog, aidec, adr, eth, req, tes, inc, tde, sec, mcard, sbom, dpia"
        exit 1
    fi

    echo ""
    echo -e "Creating: ${GREEN}${DOC_NAMES[$type]}${NC}"
    echo ""

    # Get title - ask only if not provided as argument
    if [ -z "$title" ]; then
        read -p "Title (brief description): " title
        if [ -z "$title" ]; then
            echo -e "${RED}Error: Title is required${NC}"
            exit 1
        fi
    fi

    # Get description for filename - ask only if not provided as argument
    if [ -z "$description" ]; then
        if [ "$args_provided" -ge 2 ]; then
            # Title was passed as arg but no slug; derive from title
            description="$title"
        else
            read -p "Filename slug (short, lowercase): " description
            if [ -z "$description" ]; then
                description="$title"
            fi
        fi
    fi

    # Confirm - skip when user provided args (they already know what they want)
    if [ "$args_provided" -lt 2 ]; then
        echo ""
        read -p "Create document? [Y/n]: " confirm
        if [[ "$confirm" =~ ^[Nn] ]]; then
            echo "Cancelled."
            exit 0
        fi
    fi

    create_document "$type" "$title" "$description"
}

main "$@"
