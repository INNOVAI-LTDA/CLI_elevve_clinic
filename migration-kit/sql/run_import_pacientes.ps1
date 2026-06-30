# Sobe o backend FastAPI e importa pacientes do CSV
# Salve como run_import_pacientes.ps1 e execute no PowerShell

# Caminho do backend
$backendPath = "migration-kit/backend-integration"
$importScript = "migration-kit/sql/import_pacientes.py"

# Ativa ambiente virtual se existir
if (Test-Path "$backendPath/.venv/Scripts/Activate.ps1") {
    Write-Host "Ativando ambiente virtual..."
    . "$backendPath/.venv/Scripts/Activate.ps1"
}

# Sobe o backend em background
Write-Host "Iniciando backend FastAPI..."
$backendProc = Start-Process -FilePath "python" -ArgumentList "-m uvicorn admin_api:app --reload --host 127.0.0.1 --port 8000" -WorkingDirectory $backendPath -PassThru

# Aguarda backend subir
Start-Sleep -Seconds 5

# Executa o script de importação
Write-Host "Executando importação do CSV..."
python $importScript

# Finaliza backend
Write-Host "Finalizando backend..."
Stop-Process -Id $backendProc.Id
Write-Host "Processo concluído."
