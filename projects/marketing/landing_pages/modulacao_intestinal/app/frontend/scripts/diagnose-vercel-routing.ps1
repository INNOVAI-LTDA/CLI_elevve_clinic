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

function Location-Is {
  param(
    [string]$Actual,
    [string]$Expected
  )

  if ([string]::IsNullOrWhiteSpace($Actual)) {
    return $false
  }

  return $Actual -eq $Expected -or $Actual.EndsWith($Expected)
}

function Invoke-Check {
  param(
    [string]$RootUrl,
    [string]$Path,
    [switch]$GetBody
  )

  $uri = "$RootUrl$Path"
  $request = [System.Net.HttpWebRequest]::Create($uri)
  $request.Method = 'GET'
  $request.AllowAutoRedirect = $false
  $request.Timeout = $TimeoutSec * 1000
  $request.ReadWriteTimeout = $TimeoutSec * 1000
  $request.UserAgent = 'routing-diagnose/1.0'

  $response = $null
  $statusCode = $null
  $location = $null
  $server = $null
  $body = $null
  $errorMessage = $null

  try {
    $response = [System.Net.HttpWebResponse]$request.GetResponse()
  }
  catch [System.Net.WebException] {
    if ($_.Exception.Response) {
      $response = [System.Net.HttpWebResponse]$_.Exception.Response
      $errorMessage = $_.Exception.Message
    }
    else {
      $errorMessage = $_.Exception.Message
    }
  }
  catch {
    $errorMessage = $_.Exception.Message
  }

  if ($response) {
    try {
      $statusCode = [int]$response.StatusCode
      $location = [string]$response.Headers['Location']
      $server = [string]$response.Headers['Server']

      if ($GetBody) {
        $stream = $response.GetResponseStream()
        if ($stream) {
          $reader = New-Object System.IO.StreamReader($stream)
          $body = $reader.ReadToEnd()
          $reader.Dispose()
          $stream.Dispose()
        }
      }
    }
    finally {
      $response.Close()
    }
  }

  return [pscustomobject]@{
    Path = $Path
    Url = $uri
    StatusCode = $statusCode
    Location = $location
    Server = $server
    Body = $body
    Error = $errorMessage
  }
}

function Print-Result {
  param($Item)

  Write-Host "`n[$($Item.Path)]"
  Write-Host "URL: $($Item.Url)"
  Write-Host "Status: $($Item.StatusCode)"
  if ($Item.Location) { Write-Host "Location: $($Item.Location)" }
  if ($Item.Server) { Write-Host "Server: $($Item.Server)" }
  if ($Item.Error) {
    Write-Host "Error: $($Item.Error)" -ForegroundColor Yellow
  }
}

$root = Normalize-BaseUrl -Url $BaseUrl

Write-Host "Diagnostico de roteamento para: $root" -ForegroundColor Cyan

$checks = @(
  Invoke-Check -RootUrl $root -Path '/'
  Invoke-Check -RootUrl $root -Path '/intestino' -GetBody
  Invoke-Check -RootUrl $root -Path '/intestino/' -GetBody
  Invoke-Check -RootUrl $root -Path '/public/intestino/index.html'
)

$checks | ForEach-Object { Print-Result -Item $_ }

$rootPath = $checks | Where-Object { $_.Path -eq '/' } | Select-Object -First 1
$intNoSlash = $checks | Where-Object { $_.Path -eq '/intestino' } | Select-Object -First 1
$intSlash = $checks | Where-Object { $_.Path -eq '/intestino/' } | Select-Object -First 1
$publicIndex = $checks | Where-Object { $_.Path -eq '/public/intestino/index.html' } | Select-Object -First 1

$body = if (-not [string]::IsNullOrWhiteSpace([string]$intNoSlash.Body)) {
  [string]$intNoSlash.Body
}
else {
  [string]$intSlash.Body
}
$hasReactRoot = $body -match '<div\s+id="root"\s*>'
$hasLegacyCss = $body -match '/public/css/output.css'

Write-Host "`n=== Inferencia ===" -ForegroundColor Green

$allUnauthorized = ($checks | Where-Object { $_.StatusCode -eq 401 }).Count -eq $checks.Count
$hasRedirectLoop = (
  ($intNoSlash.StatusCode -in 301, 302, 307, 308) -and
  (Location-Is -Actual $intNoSlash.Location -Expected '/intestino/') -and
  ($intSlash.StatusCode -in 301, 302, 307, 308) -and
  (Location-Is -Actual $intSlash.Location -Expected '/intestino')
)

if ($allUnauthorized) {
  Write-Host "ALERTA: Todas as rotas retornaram 401." -ForegroundColor Yellow
  Write-Host "Isso indica protecao de deployment ativa (SSO/password) no preview da Vercel." -ForegroundColor Yellow
}
elseif ($hasRedirectLoop) {
  Write-Host "ERRO: Loop de redirect detectado entre /intestino e /intestino/." -ForegroundColor Red
  Write-Host "Causa comum: redirect forcando slash e trailingSlash=false ao mesmo tempo." -ForegroundColor Red
}
elseif (
  $rootPath.StatusCode -in 301, 302, 307, 308 -and
  (Location-Is -Actual $rootPath.Location -Expected '/intestino/') -and
  $intNoSlash.StatusCode -in 301, 302, 307, 308 -and
  (Location-Is -Actual $intNoSlash.Location -Expected '/intestino/') -and
  $intSlash.StatusCode -eq 200 -and
  $hasReactRoot
) {
  Write-Host "OK: Muito provavel que o deploy esteja usando o vercel.json do frontend (app/frontend)." -ForegroundColor Green
}
elseif (
  $rootPath.StatusCode -in 301, 302, 307, 308 -and
  (Location-Is -Actual $rootPath.Location -Expected '/intestino') -and
  $intNoSlash.StatusCode -eq 200 -and
  $intSlash.StatusCode -in 301, 302, 307, 308 -and
  (Location-Is -Actual $intSlash.Location -Expected '/intestino') -and
  $hasReactRoot
) {
  Write-Host "OK: Roteamento estavel com canonical sem slash (/intestino)." -ForegroundColor Green
}
elseif ($intNoSlash.StatusCode -eq 404 -and $intSlash.StatusCode -eq 200) {
  Write-Host "ALERTA: /intestino esta 404 e /intestino/ esta 200." -ForegroundColor Yellow
  Write-Host "A regra de normalizacao sem slash nao esta aplicada neste ambiente." -ForegroundColor Yellow
}
elseif ($intNoSlash.StatusCode -eq 404 -and $publicIndex.StatusCode -eq 404) {
  Write-Host "ALERTA: /intestino e /public/intestino/index.html retornaram 404." -ForegroundColor Yellow
  Write-Host "Revise Root Directory e qual vercel.json esta ativo no projeto." -ForegroundColor Yellow
}
else {
  Write-Host "INFO: Resultado inconclusivo. Compare os headers/status acima com as regras esperadas." -ForegroundColor Cyan
}

if ($hasLegacyCss) {
  Write-Host "ALERTA: Detectado marcador de HTML legado (/public/css/output.css) em /intestino/." -ForegroundColor Yellow
  Write-Host "Isso sugere que o deploy pode estar servindo uma versao estatica da raiz." -ForegroundColor Yellow
}

if ($hasReactRoot) {
  Write-Host 'INFO: Detectado marcador React (<div id="root">) em /intestino/.' -ForegroundColor Cyan
}

Write-Host "`nProximo passo recomendado: validar Root Directory e regras de redirect no painel da Vercel." -ForegroundColor Magenta
