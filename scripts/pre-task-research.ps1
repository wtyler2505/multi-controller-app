# Pre-Task Research Enforcement Script
# Ensures research is conducted before task implementation

param(
    [Parameter(Mandatory=$true)]
    [string]$TaskId,
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipCheck
)

Write-Host "[PRE-TASK] Checking research for task $TaskId" -ForegroundColor Cyan

if ($SkipCheck) {
    Write-Host "[WARNING] Skipping research check (not recommended)" -ForegroundColor Yellow
    exit 0
}

# Check for research evidence in memory
$searchQuery = "task $TaskId research"
Write-Host "[CHECKING] Searching memory for: $searchQuery" -ForegroundColor Gray

# Use cipher to check for research
$memoryCheck = & cipher memory:search $searchQuery 2>&1
if ($LASTEXITCODE -eq 0 -and $memoryCheck) {
    Write-Host "[OK] Research found for task $TaskId" -ForegroundColor Green
    exit 0
}

# If no research found, provide guidance
Write-Host "" -ForegroundColor Red
Write-Host "[BLOCKED] No research found for task $TaskId!" -ForegroundColor Red
Write-Host "" -ForegroundColor Red
Write-Host "REQUIRED STEPS BEFORE IMPLEMENTATION:" -ForegroundColor Yellow
Write-Host "1. Run sequentialthinking to break down the task" -ForegroundColor White
Write-Host "2. Use get-library-docs for official documentation" -ForegroundColor White
Write-Host "3. Query perplexity_ask for best practices" -ForegroundColor White
Write-Host "4. Check ref/ folder for existing patterns" -ForegroundColor White
Write-Host "5. Store findings with cipher_extract_and_operate_memory" -ForegroundColor White
Write-Host ""
Write-Host "After research, run this script again to proceed." -ForegroundColor Cyan

exit 1