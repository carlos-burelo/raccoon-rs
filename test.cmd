@echo off
setlocal enabledelayedexpansion

for %%f in ("tests\*.rcc") do (
  echo Running test: %%f
  cargo run -- run %%f --interpret
)
