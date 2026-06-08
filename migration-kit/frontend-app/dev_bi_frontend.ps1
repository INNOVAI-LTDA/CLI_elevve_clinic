#Requires -Version 5.1
<#
.SYNOPSIS
    Sobe o frontend SPA do BI Elevve em modo dev.
.DESCRIPTION
    Wrapper PowerShell que:
    - checa se o .env existe; se nao, copia do .env.example
    - valida que VITE_DEPLOY_TARGET esta definido (caso contrario aborta)
    - executa `npm run dev` no diretorio do frontend

    O backend (uvicorn admin_api:app) NAO e' iniciado por este
    script -- rode separadamente se quiser a API junto:

        cd ..\backend-integration
        python -m uvicorn admin_api:app --host 127.0.0.1 --port 8000

.EXAMPLE
    .\dev_bi_frontend.ps1
.NOTES
    Pare com Ctrl+C. O dev server fica ouvindo ate interrupcao.
#>
param()

$ErrorActionPreference = 'Stop'

$FrontendRoot = $PSScriptRoot
$EnvFile = Join-Path $FrontendRoot '.env'
$EnvExample = Join-Path $FrontendRoot '.env.example'
$PackageJson = Join-Path $FrontendRoot 'package.json'

# --- conferir arquivos esperados ---
$missing = @()
foreach ($f in @($PackageJson)) {
    if (-not (Test-Path -LiteralPath $f)) { $missing += $f }
}
if ($missing.Count -gt 0) {
    Write-Host "ERRO: arquivos ausentes:" -ForegroundColor Red
    $missing | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
    exit 2
}

# --- garantir que node_modules esta instalado ---
if (-not (Test-Path -LiteralPath (Join-Path $FrontendRoot 'node_modules'))) {
    Write-Host "node_modules nao encontrado. Rode 'npm install' antes de subir o dev server." -ForegroundColor Red
    exit 2
}

# --- copiar .env do .env.example se necessario ---
if (-not (Test-Path -LiteralPath $EnvFile)) {
    if (Test-Path -LiteralPath $EnvExample) {
        Write-Host "Criando .env a partir de .env.example..." -ForegroundColor Cyan
        Copy-Item -LiteralPath $EnvExample -Destination $EnvFile
    } else {
        Write-Host "ERRO: nem .env nem .env.example encontrados em $FrontendRoot" -ForegroundColor Red
        exit 2
    }
}

# --- checar VITE_DEPLOY_TARGET ---
$EnvContent = Get-Content -LiteralPath $EnvFile -Raw
if ($EnvContent -notmatch '(?m)^VITE_DEPLOY_TARGET\s*=') {
    Write-Host "ERRO: VITE_DEPLOY_TARGET nao definido em .env" -ForegroundColor Red
    Write-Host "Adicione a linha 'VITE_DEPLOY_TARGET=local' no .env" -ForegroundColor Red
    exit 2
}

Write-Host "Subindo o dev server do frontend (Ctrl+C para parar)..." -ForegroundColor Green
Write-Host "  URL: http://127.0.0.1:5173" -ForegroundColor Green
Write-Host ""

& npm run dev
exit $LASTEXITCODE
