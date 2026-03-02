# =============================================================================
# DevTrail - Documentation Validation Script (PowerShell)
# https://strangedays.tech
# =============================================================================
#
# Usage:
#   .\scripts\validate-docs.ps1
#   .\scripts\validate-docs.ps1 -Path ".devtrail/07-ai-audit"
#   .\scripts\validate-docs.ps1 -Verbose
#
# =============================================================================

param(
    [string]$Path = ".devtrail",
    [switch]$Verbose
)

$ErrorCount = 0
$WarningCount = 0

Write-Host "🔍 Validating documentation in: $Path" -ForegroundColor Cyan
Write-Host ""

# =============================================================================
# 1. Get markdown files
# =============================================================================

$MarkdownFiles = Get-ChildItem -Path $Path -Filter "*.md" -Recurse -ErrorAction SilentlyContinue

if ($MarkdownFiles.Count -eq 0) {
    Write-Host "✓ No markdown files to validate" -ForegroundColor Green
    exit 0
}

Write-Host "Files found: $($MarkdownFiles.Count)"
Write-Host ""

# =============================================================================
# 2. Define patterns and exclusions
# =============================================================================

$ValidTypes = @("ADR", "REQ", "TES", "OPS", "INC", "TDE", "AILOG", "AIDEC", "ETH", "DOC")
$ExcludedFiles = @("PRINCIPLES.md", "DOCUMENTATION-POLICY.md", "AGENT-RULES.md", "README.md", "QUICK-REFERENCE.md", "INDEX.md", ".gitkeep")
$ExcludedPatterns = @("TEMPLATE-.*\.md")

$ValidStatuses = @(
    "draft", "proposed", "accepted", "deprecated", "superseded",
    "investigating", "identified", "monitoring", "resolved", "closed",
    "under_review", "approved", "rejected", "requires_changes",
    "implemented", "in_progress", "wont_fix"
)

$SensitivePatterns = @("password", "api_key", "apikey", "secret", "token", "private_key", "credentials", "passwd")

# =============================================================================
# 3. Function to check if file is excluded
# =============================================================================

function Test-ExcludedFile {
    param([string]$FileName)

    if ($ExcludedFiles -contains $FileName) {
        return $true
    }

    foreach ($pattern in $ExcludedPatterns) {
        if ($FileName -match $pattern) {
            return $true
        }
    }

    return $false
}

# =============================================================================
# 4. Validate naming convention
# =============================================================================

Write-Host "📋 Validating file naming convention..." -ForegroundColor Cyan

foreach ($file in $MarkdownFiles) {
    $fileName = $file.Name

    if (Test-ExcludedFile -FileName $fileName) {
        if ($Verbose) {
            Write-Host "  ⊘ Excluded: $fileName" -ForegroundColor Yellow
        }
        continue
    }

    # Pattern: TYPE-YYYY-MM-DD-NNN-description.md
    $pattern = "^($($ValidTypes -join '|'))-\d{4}-\d{2}-\d{2}-\d{3}-[a-z0-9-]+\.md$"

    if ($fileName -notmatch $pattern) {
        Write-Host "  ✗ Invalid naming: $fileName" -ForegroundColor Red
        Write-Host "    Expected: [TYPE]-[YYYY-MM-DD]-[NNN]-[description].md" -ForegroundColor Gray
        $ErrorCount++
    } else {
        Write-Host "  ✓ $fileName" -ForegroundColor Green
    }
}

Write-Host ""

# =============================================================================
# 5. Validate front-matter
# =============================================================================

Write-Host "📋 Validating metadata (front-matter)..." -ForegroundColor Cyan

$RequiredFields = @("id", "title", "status", "created")

foreach ($file in $MarkdownFiles) {
    $fileName = $file.Name

    if (Test-ExcludedFile -FileName $fileName) {
        continue
    }

    $content = Get-Content -Path $file.FullName -Raw -ErrorAction SilentlyContinue

    if (-not $content) {
        Write-Host "  ✗ Cannot read: $fileName" -ForegroundColor Red
        $ErrorCount++
        continue
    }

    # Verify front-matter exists
    if ($content -notmatch "^---") {
        Write-Host "  ✗ Missing YAML front-matter: $fileName" -ForegroundColor Red
        $ErrorCount++
        continue
    }

    # Extract front-matter
    if ($content -match "(?s)^---\r?\n(.+?)\r?\n---") {
        $frontmatter = $Matches[1]

        $missingFields = @()
        foreach ($field in $RequiredFields) {
            if ($frontmatter -notmatch "(?m)^$field`:") {
                $missingFields += $field
            }
        }

        if ($missingFields.Count -gt 0) {
            Write-Host "  ✗ Missing fields in $fileName`: $($missingFields -join ', ')" -ForegroundColor Red
            $ErrorCount++
        } else {
            Write-Host "  ✓ $fileName - metadata complete" -ForegroundColor Green
        }
    } else {
        Write-Host "  ✗ Malformed front-matter: $fileName" -ForegroundColor Red
        $ErrorCount++
    }
}

Write-Host ""

# =============================================================================
# 6. Validate sensitive information
# =============================================================================

Write-Host "🔒 Checking for sensitive information..." -ForegroundColor Cyan

foreach ($file in $MarkdownFiles) {
    $content = Get-Content -Path $file.FullName -Raw -ErrorAction SilentlyContinue

    $foundSensitive = $false
    foreach ($pattern in $SensitivePatterns) {
        if ($content -match "(?i)$pattern") {
            if (-not $foundSensitive) {
                Write-Host "  ⚠ Possible sensitive information in $($file.Name):" -ForegroundColor Yellow
                $foundSensitive = $true
            }
            Write-Host "    - Pattern detected: $pattern" -ForegroundColor Yellow
            $WarningCount++
        }
    }
}

if ($WarningCount -eq 0) {
    Write-Host "  ✓ No sensitive information detected" -ForegroundColor Green
}

Write-Host ""

# =============================================================================
# 7. Validate statuses
# =============================================================================

Write-Host "📋 Validating document statuses..." -ForegroundColor Cyan

foreach ($file in $MarkdownFiles) {
    $fileName = $file.Name

    if (Test-ExcludedFile -FileName $fileName) {
        continue
    }

    $content = Get-Content -Path $file.FullName -Raw -ErrorAction SilentlyContinue

    if ($content -match "(?m)^status:\s*(.+)$") {
        $status = $Matches[1].Trim()

        if ($ValidStatuses -notcontains $status) {
            Write-Host "  ✗ Invalid status in $fileName`: '$status'" -ForegroundColor Red
            Write-Host "    Valid statuses: $($ValidStatuses -join ', ')" -ForegroundColor Gray
            $ErrorCount++
        } else {
            Write-Host "  ✓ $fileName - status: $status" -ForegroundColor Green
        }
    }
}

Write-Host ""

# =============================================================================
# 8. Summary
# =============================================================================

Write-Host "═══════════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "📊 Validation summary" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan

if ($ErrorCount -gt 0) {
    Write-Host "✗ Errors found: $ErrorCount" -ForegroundColor Red
}

if ($WarningCount -gt 0) {
    Write-Host "⚠ Warnings: $WarningCount" -ForegroundColor Yellow
}

if ($ErrorCount -eq 0 -and $WarningCount -eq 0) {
    Write-Host "✓ All validations passed" -ForegroundColor Green
}

Write-Host ""

if ($ErrorCount -gt 0) {
    Write-Host "❌ Validation failed" -ForegroundColor Red
    exit 1
} else {
    Write-Host "✅ Validation completed" -ForegroundColor Green
    exit 0
}
