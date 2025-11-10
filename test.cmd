@echo off
setlocal enabledelayedexpansion

for %%f in (".\tests\*.rcc") do (
  echo Running test: %%f
  cargo run -- %%f | findstr /i "error"
  if not errorlevel 1 echo Falló: %%f
)
