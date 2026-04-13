param(
  [Parameter(Mandatory = $true)]
  [string]$BaseUrl,

  [int]$TimeoutSec = 20
)

$ErrorActionPreference = 'Stop'

function Normalize-BaseUrl {
  param([string]$Url)
  if (-not ($Url -match '^https?://')) {
    $Url = "https://$Url"
  }
  return $Url.TrimEnd('/')
}

function Invoke-Check {
  param(
    [string]$RootUrl,
    [string]$Path,
    [switch]$GetBody
  )

  $uri = "$RootUrl$Path"
  try {
    $response = Invoke-WebRequest -Uri $uri -Method Get -MaximumRedirection 0 -TimeoutSec $TimeoutSec -ErrorAction Stop
    return [pscustomobject]@{
      Path = $Path
      Url = $uri
      StatusCode = [int]$response.StatusCode
      Location = $response.Headers['Location']
      Server = $response.Headers['server']
      Body = if ($GetBody) { [string]$response.Content } else { $null }
      Error = $null
    }
  }
  catch {
    $statusCode = $null
    $location = $null
    $server = $null
    $body = $null

    if ($_.Exception.Response) {
      try { $statusCode = [int]$_.Exception.Response.StatusCode.value__ } catch {}
      try { $location = $_.Exception.Response.Headers['Location'] } catch {}
      try { $server = $_.Exception.Response.Headers['server'] } catch {}
      if ($GetBody) {
        try {
          $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
          $body = $reader.ReadToEnd()
        } catch {}
      }
    }

    return [pscustomobject]@{
      Path = $Path
      Url = $uri
      StatusCode = $statusCode
      Location = $location
      Server = $server
      Body = $body
      Error = $_.Exception.Message
    }
  }
}

function Print-Result {
  param($Item)
  Write-Host "`n[$($Item.Path)]"
  Write-Host "URL: $($Item.Url)"
  Write-Host "Status: $($Item.StatusCode)"
  if ($Item.Location) { Write-Host "Location: $($Item.Location)" }
  if ($Item.Server) { Write-Host "Server: $($Item.Server)" }
  if ($Item.Error -and -not $Item.StatusCode) {
    Write-Host "Error: $($Item.Error)" -ForegroundColor Yellow
  }
}

$root = Normalize-BaseUrl -Url $BaseUrl

Write-Host "Diagnóstico de roteamento para: $root" -ForegroundColor Cyan

$checks = @(
  Invoke-Check -RootUrl $root -Path '/'
  Invoke-Check -RootUrl $root -Path '/intestino'
  Invoke-Check -RootUrl $root -Path '/intestino/' -GetBody
  Invoke-Check -RootUrl $root -Path '/public/intestino/index.html'
)

$checks | ForEach-Object { Print-Result -Item $_ }

$rootPath = $checks | Where-Object { $_.Path -eq '/' } | Select-Object -First 1
$intNoSlash = $checks | Where-Object { $_.Path -eq '/intestino' } | Select-Object -First 1
$intSlash = $checks | Where-Object { $_.Path -eq '/intestino/' } | Select-Object -First 1
$publicIndex = $checks | Where-Object { $_.Path -eq '/public/intestino/index.html' } | Select-Object -First 1

$body = [string]$intSlash.Body
$hasReactRoot = $body -match '<div\s+id="root"\s*>'
$hasLegacyCss = $body -match '/public/css/output.css'

Write-Host "`n=== Inferência ===" -ForegroundColor Green

if ($rootPath.StatusCode -in 301,302,307,308 -and $rootPath.Location -eq '/intestino/' -and $intNoSlash.StatusCode -in 301,302,307,308 -and $intNoSlash.Location -eq '/intestino/' -and $intSlash.StatusCode -eq 200 -and $hasReactRoot) {
  Write-Host "✅ Muito provável que o deploy esteja usando o vercel.json do frontend (app/frontend)." -ForegroundColor Green
}
elseif ($intNoSlash.StatusCode -eq 404 -and $intSlash.StatusCode -eq 200) {
  Write-Host "⚠️ /intestino está 404 e /intestino/ está 200: regra de normalização sem slash não está aplicada no ambiente atual." -ForegroundColor Yellow
}
elseif ($intNoSlash.StatusCode -eq 404 -and $publicIndex.StatusCode -eq 404) {
  Write-Host "⚠️ Sinal de configuração inconsistente: /intestino e /public/intestino/index.html em 404." -ForegroundColor Yellow
  Write-Host "   Revise Root Directory e qual vercel.json está ativo no projeto." -ForegroundColor Yellow
}
else {
  Write-Host "ℹ️ Resultado inconclusivo. Compare os headers/status acima com as regras esperadas." -ForegroundColor Cyan
}

if ($hasLegacyCss) {
  Write-Host "⚠️ Detectado marcador de HTML legado (/public/css/output.css) na resposta de /intestino/." -ForegroundColor Yellow
  Write-Host "   Isso sugere que o deploy pode estar servindo a versão estática da raiz." -ForegroundColor Yellow
}

if ($hasReactRoot) {
  Write-Host "ℹ️ Detectado marcador React (<div id=\"root\">) na resposta de /intestino/." -ForegroundColor Cyan
}

Write-Host "`nPróximo passo recomendado: validar Root Directory no painel da Vercel." -ForegroundColor Magenta
