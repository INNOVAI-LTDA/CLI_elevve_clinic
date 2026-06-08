#Requires -Version 5.1
<#
.SYNOPSIS
    Smoke test do Commit 2 - BI Radar de Longevidade (deva_elevveclinic_bi_*).
.DESCRIPTION
    Wrapper PowerShell que aplica as migrations 010/011 (idempotente),
    roda 12 checks de schema/seed/sanidade, valida os 5 runtime-stores
    JSON, calcula os scores esperados para os 2 pacientes demo e
    verifica idempotencia do seed.

    A logica de teste fica em test_bi_radar.py (mesmo diretorio);
    este .ps1 e' um wrapper fino que localiza o python, valida os
    arquivos esperados e propaga o exit code.

    Codigos de saida:
      0  = todos os checks passaram
      1  = algum check falhou
      2  = erro de setup (python ausente, arquivo faltando, etc.)
.PARAMETER DbPath
    Caminho do SQLite de referencia. Default: elevve_clinic_deva_db.db
    ao lado deste script.
.PARAMETER NoApply
    Pula a aplicacao das migrations (util se voce acabou de aplicar e
    quer so revalidar).
.EXAMPLE
    .\test_bi_radar.ps1
.EXAMPLE
    .\test_bi_radar.ps1 -NoApply
.EXAMPLE
    .\test_bi_radar.ps1 -DbPath "C:\temp\outro.db"
#>
param(
    [string]$DbPath = (Join-Path $PSScriptRoot 'elevve_clinic_deva_db.db'),
    [switch]$NoApply
)

$ErrorActionPreference = 'Stop'

# --- localizar python (sintaxe compativel com Windows PowerShell 5.1) ---
$pythonCmd = Get-Command python -ErrorAction SilentlyContinue
$python = $null
if ($pythonCmd) { $python = $pythonCmd.Source }
if ([string]::IsNullOrEmpty($python)) {
    Write-Host "ERRO: python nao encontrado no PATH." -ForegroundColor Red
    Write-Host "Instale Python 3 (stdlib sqlite3 ja basta) e tente de novo." -ForegroundColor Red
    exit 2
}

$ScriptPath = Join-Path $PSScriptRoot 'test_bi_radar.py'
$MigrationsDir = Join-Path $PSScriptRoot 'migrations'
$RuntimeStoresDir = Join-Path $PSScriptRoot 'runtime-stores'

# --- conferir arquivos esperados ---
$missing = @()
foreach ($f in @(
    $ScriptPath,
    (Join-Path $MigrationsDir '010_deva_elevveclinic_bi_radar.sql'),
    (Join-Path $MigrationsDir '011_deva_elevveclinic_bi_radar_seed.sql'),
    (Join-Path $RuntimeStoresDir 'bi_radar_pillars.json'),
    (Join-Path $RuntimeStoresDir 'bi_radar_questions.json'),
    (Join-Path $RuntimeStoresDir 'bi_radar_clusters.json'),
    (Join-Path $RuntimeStoresDir 'bi_radar_responses.json'),
    (Join-Path $RuntimeStoresDir 'bi_radar_interpretation_ranges.json')
)) {
    if (-not (Test-Path -LiteralPath $f)) { $missing += $f }
}
if ($missing.Count -gt 0) {
    Write-Host "ERRO: arquivos ausentes:" -ForegroundColor Red
    $missing | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
    exit 2
}

# --- passar paths via env ---
$env:BI_RADAR_DB             = $DbPath
$env:BI_RADAR_MIGRATIONS     = $MigrationsDir
$env:BI_RADAR_RUNTIME_STORES = $RuntimeStoresDir
$env:BI_RADAR_SKIP_APPLY     = if ($NoApply) { '1' } else { '0' }

# --- rodar o python e propagar o exit code ---
& $python $ScriptPath
$LAST = $LASTEXITCODE
if ($LAST -ne $null) { exit $LAST } else { exit 1 }
