#!/bin/bash

# Performance Gate Validation Script for Linux/Mac
# Ensures performance budgets are met before allowing commits

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
CONFIG_FILE="$SCRIPT_DIR/../../.gitmeta/config/performance-budgets.json"

# Parse command line arguments
SKIP_STARTUP=false
SKIP_MEMORY=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-startup)
            SKIP_STARTUP=true
            shift
            ;;
        --skip-memory)
            SKIP_MEMORY=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Load configuration using Python (cross-platform JSON parsing)
read_config() {
    python3 -c "
import json
import sys
with open('$CONFIG_FILE', 'r') as f:
    config = json.load(f)
    if '$1' == 'startup_max':
        print(config['startup']['max_ms'])
    elif '$1' == 'startup_warn':
        print(config['startup']['warn_ms'])
    elif '$1' == 'memory_max':
        print(config['memory']['max_mb'])
    elif '$1' == 'memory_warn':
        print(config['memory']['warn_mb'])
"
}

# Function to measure startup time
test_startup_performance() {
    if [ "$SKIP_STARTUP" = true ]; then
        echo -e "${YELLOW}⏭️  Skipping startup performance check${NC}"
        return 0
    fi

    echo -e "${YELLOW}⏱️  Measuring startup performance...${NC}"
    
    PROJECT_PATH="$SCRIPT_DIR/../../apps/desktop/MultiControllerApp.csproj"
    
    if [ ! -f "$PROJECT_PATH" ]; then
        echo -e "${YELLOW}⚠️  Desktop project not found, skipping startup check${NC}"
        return 0
    fi
    
    # Build in Release mode first
    echo "Building in Release mode..."
    dotnet build "$PROJECT_PATH" -c Release > /dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Build failed, cannot measure startup performance${NC}"
        return 1
    fi
    
    # Measure startup time (average of 3 runs)
    MEASUREMENTS=()
    for i in 1 2 3; do
        echo "  Run $i/3..."
        
        # Start timing
        START_TIME=$(date +%s%3N)
        
        # Start the app in background
        dotnet run --project "$PROJECT_PATH" --configuration Release --no-build > /dev/null 2>&1 &
        APP_PID=$!
        
        # Wait a bit for startup
        sleep 0.5
        
        # Stop timing
        END_TIME=$(date +%s%3N)
        ELAPSED=$((END_TIME - START_TIME))
        MEASUREMENTS+=($ELAPSED)
        
        # Kill the process
        kill $APP_PID 2>/dev/null
        wait $APP_PID 2>/dev/null
        
        sleep 0.2
    done
    
    # Calculate average
    TOTAL=0
    for m in "${MEASUREMENTS[@]}"; do
        TOTAL=$((TOTAL + m))
    done
    AVG=$((TOTAL / 3))
    
    MAX_ALLOWED=$(read_config "startup_max")
    WARN_THRESHOLD=$(read_config "startup_warn")
    
    echo "  Measurements: ${MEASUREMENTS[0]}ms, ${MEASUREMENTS[1]}ms, ${MEASUREMENTS[2]}ms"
    echo "  Average: ${AVG}ms"
    
    if [ $AVG -gt $MAX_ALLOWED ]; then
        echo -e "${RED}❌ Startup time (${AVG}ms) exceeds budget (${MAX_ALLOWED}ms)${NC}"
        return 1
    elif [ $AVG -gt $WARN_THRESHOLD ]; then
        echo -e "${YELLOW}⚠️  Startup time (${AVG}ms) approaching limit (max: ${MAX_ALLOWED}ms)${NC}"
    else
        echo -e "${GREEN}✅ Startup time OK: ${AVG}ms (budget: ${MAX_ALLOWED}ms)${NC}"
    fi
    
    return 0
}

# Function to check memory usage
test_memory_usage() {
    if [ "$SKIP_MEMORY" = true ]; then
        echo -e "${YELLOW}⏭️  Skipping memory check${NC}"
        return 0
    fi

    echo -e "${YELLOW}💾 Checking memory usage...${NC}"
    
    # For now, check binary size as proxy for memory
    BIN_PATH="$SCRIPT_DIR/../../apps/desktop/bin/Release/net8.0"
    
    if [ -d "$BIN_PATH" ]; then
        # Get total size in MB
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS
            TOTAL_SIZE=$(find "$BIN_PATH" -type f -exec stat -f%z {} + | awk '{s+=$1} END {printf "%.2f", s/1048576}')
        else
            # Linux
            TOTAL_SIZE=$(find "$BIN_PATH" -type f -exec stat --format=%s {} + | awk '{s+=$1} END {printf "%.2f", s/1048576}')
        fi
        
        echo "  Binary size: ${TOTAL_SIZE}MB"
        
        # Rough estimate: binary size * 3 for runtime memory
        ESTIMATED_MEMORY=$(echo "$TOTAL_SIZE * 3" | bc)
        MAX_ALLOWED=$(read_config "memory_max")
        
        if (( $(echo "$ESTIMATED_MEMORY > $MAX_ALLOWED" | bc -l) )); then
            echo -e "${YELLOW}⚠️  Estimated memory usage high: ~${ESTIMATED_MEMORY}MB${NC}"
        else
            echo -e "${GREEN}✅ Memory estimate OK: ~${ESTIMATED_MEMORY}MB (budget: ${MAX_ALLOWED}MB)${NC}"
        fi
    else
        echo -e "${YELLOW}⚠️  No release build found for memory estimation${NC}"
    fi
    
    return 0
}

# Function to check code changes
test_code_changes() {
    echo -e "${YELLOW}📝 Checking for performance-critical changes...${NC}"
    
    # Get list of staged files
    STAGED_FILES=$(git diff --cached --name-only)
    
    CRITICAL_FILES=("Program.cs" "App.xaml.cs" "MainWindow.xaml.cs")
    HAS_CRITICAL=false
    
    for file in $STAGED_FILES; do
        for critical in "${CRITICAL_FILES[@]}"; do
            if [[ "$file" == *"$critical"* ]]; then
                echo -e "  ${YELLOW}⚠️  Critical file changed: $file${NC}"
                HAS_CRITICAL=true
            fi
        done
    done
    
    if [ "$HAS_CRITICAL" = true ]; then
        echo -e "  ${YELLOW}⚠️  Performance-critical files changed - extra validation recommended${NC}"
    else
        echo -e "  ${GREEN}✅ No performance-critical files in this commit${NC}"
    fi
    
    return 0
}

# Main execution
echo -e "\n${YELLOW}🚦 Performance Gate Validation${NC}"
echo -e "${YELLOW}================================${NC}"

ALL_PASSED=true

# Run tests
test_code_changes || ALL_PASSED=false
test_startup_performance || ALL_PASSED=false
test_memory_usage || ALL_PASSED=false

echo -e "${YELLOW}================================${NC}"

if [ "$ALL_PASSED" = true ]; then
    echo -e "${GREEN}✅ All performance checks passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Performance validation failed!${NC}"
    echo -e "\n${YELLOW}To bypass (use with caution):${NC}"
    echo "  git commit --no-verify"
    echo -e "\n${YELLOW}To fix performance issues:${NC}"
    echo "  1. Review recent changes for performance impact"
    echo "  2. Profile the application"
    echo "  3. Consider using lazy loading or async initialization"
    exit 1
fi