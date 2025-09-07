#!/bin/bash

# Install git hooks for Multi-Controller App

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     Git Hooks Installation Script    ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"

# Get the repository root
REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Not in a git repository${NC}"
    exit 1
fi

cd "$REPO_ROOT"

# Check if hooks directory exists
HOOKS_SOURCE="scripts/git-automation/hooks"
HOOKS_DEST=".git/hooks"

if [ ! -d "$HOOKS_SOURCE" ]; then
    echo -e "${RED}❌ Hooks source directory not found: $HOOKS_SOURCE${NC}"
    exit 1
fi

# Function to install a hook
install_hook() {
    local hook_name=$1
    local source_file="$HOOKS_SOURCE/$hook_name"
    local dest_file="$HOOKS_DEST/$hook_name"
    
    if [ ! -f "$source_file" ]; then
        echo -e "${YELLOW}⚠️  Hook not found: $hook_name${NC}"
        return 1
    fi
    
    # Backup existing hook if it exists
    if [ -f "$dest_file" ]; then
        if [ ! -f "$dest_file.backup" ]; then
            cp "$dest_file" "$dest_file.backup"
            echo -e "${YELLOW}   Backed up existing $hook_name to $hook_name.backup${NC}"
        fi
    fi
    
    # Copy and make executable
    cp "$source_file" "$dest_file"
    chmod +x "$dest_file"
    echo -e "${GREEN}✅ Installed: $hook_name${NC}"
    return 0
}

# Install hooks
echo -e "\n${BLUE}Installing hooks...${NC}"
install_hook "pre-commit"
install_hook "commit-msg"
install_hook "pre-push"

# Create symlinks for Windows compatibility
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo -e "\n${BLUE}Setting up Windows compatibility...${NC}"
    
    # Create .bat wrappers for Windows
    for hook in pre-commit commit-msg pre-push; do
        if [ -f "$HOOKS_DEST/$hook" ]; then
            cat > "$HOOKS_DEST/$hook.bat" << EOF
@echo off
bash "%~dp0$hook" %*
EOF
            echo -e "${GREEN}✅ Created Windows wrapper: $hook.bat${NC}"
        fi
    done
fi

# Verify installation
echo -e "\n${BLUE}Verifying installation...${NC}"
INSTALLED_COUNT=0
for hook in pre-commit commit-msg pre-push; do
    if [ -f "$HOOKS_DEST/$hook" ] && [ -x "$HOOKS_DEST/$hook" ]; then
        INSTALLED_COUNT=$((INSTALLED_COUNT + 1))
    fi
done

echo -e "${GREEN}✅ $INSTALLED_COUNT/3 hooks installed successfully${NC}"

# Configuration check
echo -e "\n${BLUE}Checking configuration files...${NC}"
if [ -f ".gitmeta/config/secrets-patterns.json" ]; then
    echo -e "${GREEN}✅ Secrets patterns configured${NC}"
else
    echo -e "${YELLOW}⚠️  Secrets patterns not found${NC}"
fi

if [ -f ".gitmeta/config/performance-budgets.json" ]; then
    echo -e "${GREEN}✅ Performance budgets configured${NC}"
else
    echo -e "${YELLOW}⚠️  Performance budgets not found${NC}"
fi

# Final instructions
echo -e "\n${BLUE}════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Git hooks installation complete!${NC}"
echo -e "\n${BLUE}Hooks installed:${NC}"
echo "  • pre-commit  - Runs secrets scanner and performance checks"
echo "  • commit-msg  - Enforces conventional commits and task references"
echo "  • pre-push    - Final validation before pushing to remote"
echo -e "\n${BLUE}To bypass hooks (use sparingly):${NC}"
echo "  git commit --no-verify"
echo "  git push --no-verify"
echo -e "\n${BLUE}To uninstall hooks:${NC}"
echo "  rm .git/hooks/{pre-commit,commit-msg,pre-push}"
echo -e "${BLUE}════════════════════════════════════════${NC}\n"