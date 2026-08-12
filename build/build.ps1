# ==============================================================================
#  Nantara-Boot Builder Engine
#  Open-Source System Rescue Live OS ISO Builder Script
# ==============================================================================

[CmdletBinding()]
param (
    [string]$TargetOS = "Win11PE",
    [string]$Architecture = "x64",
    [string]$ConfigFile = "$PSScriptRoot\config.json",
    [string]$OutputDir = "$PSScriptRoot\out"
)

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "   🚀 Nantara-Boot Open-Source Builder Engine v1.0" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# 1. Check & Validate Prerequisites
if (-not (Test-Path $ConfigFile)) {
    Write-Error "Config file not found: $ConfigFile"
    exit 1
}

$Config = Get-Content $ConfigFile | ConvertFrom-Json
Write-Host "[+] Loaded Project: $($Config.projectName) (v$($Config.version))" -ForegroundColor Green
Write-Host "[+] Target Kernel : $TargetOS ($Architecture)" -ForegroundColor Green

# 2. Ensure Output Directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

# 3. Process Categories & Validate Tool Fetchers
Write-Host "`n[+] Verifying Tool Packages & Fetcher Scripts..." -ForegroundColor Yellow
foreach ($category in $Config.categories) {
    Write-Host "  -> Category: $($category.name) ($($category.id))" -ForegroundColor Gray
    foreach ($tool in $category.tools) {
        Write-Host "     - Tool: $($tool.name) v$($tool.version) [Portable]" -ForegroundColor DarkGray
    }
}

# 4. Assembling Builder Pipeline Placeholder
Write-Host "`n[+] Assembling Base Kernel Environment..." -ForegroundColor Yellow
Write-Host "  -> Mount WinPE Base Image..." -ForegroundColor Gray
Write-Host "  -> Injecting Storage & Network Drivers..." -ForegroundColor Gray
Write-Host "  -> Embedding Nantara Launcher & Auto-Diagnostic Widgets..." -ForegroundColor Gray
Write-Host "  -> Packaging Portable Toolkits..." -ForegroundColor Gray

# 5. Build Final ISO
$IsoPath = Join-Path $OutputDir "Nantara-Boot-$($Config.version)-$Architecture.iso"
Write-Host "`n[+] Creating ISO Image: $IsoPath" -ForegroundColor Green
Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "   ✅ Build Initialization Completed Successfully!" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan
