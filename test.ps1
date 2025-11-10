Get-ChildItem .\tests\*.rcc | ForEach-Object {
  $out = cargo run -- $_.FullName 2>&1
  if ($out -match 'error') {
    Write-Host "Falló: $($_.Name)"
  }
}
