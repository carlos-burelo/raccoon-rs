Get-ChildItem .\tests\*.rcc | ForEach-Object {
  $output = cargo run -- $_.FullName 2>&1
  $errors = $output | Where-Object { $_ -is [System.Management.Automation.ErrorRecord] }
  if ($errors) {
    $errors | Out-Host
  }
}
