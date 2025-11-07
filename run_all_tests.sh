#!/bin/bash

# Script para ejecutar todos los tests .rcc y resumir los resultados

echo "=========================================="
echo "EJECUTANDO TODOS LOS TESTS DE RACCOON"
echo "=========================================="
echo ""

TOTAL=0
PASSED=0
FAILED=0

# Función para ejecutar un test
run_test() {
    local file="$1"
    TOTAL=$((TOTAL + 1))

    echo "[$TOTAL] Testing: $file"

    if timeout 30 cargo run -- run "$file" > /tmp/test_output.txt 2>&1; then
        if grep -q -i "error\|failed\|panic" /tmp/test_output.txt; then
            echo "    ❌ FAILED (with errors in output)"
            FAILED=$((FAILED + 1))
            echo "    Error details:"
            grep -i "error\|failed" /tmp/test_output.txt | head -3 | sed 's/^/      /'
        else
            echo "    ✅ PASSED"
            PASSED=$((PASSED + 1))
        fi
    else
        EXIT_CODE=$?
        if [ $EXIT_CODE -eq 124 ]; then
            echo "    ⏱️  TIMEOUT (>30s)"
        else
            echo "    ❌ FAILED (exit code: $EXIT_CODE)"
        fi
        FAILED=$((FAILED + 1))
    fi
    echo ""
}

# Ejecutar tests en el directorio principal
for file in tests/*.rcc; do
    if [ -f "$file" ]; then
        run_test "$file"
    fi
done

# Ejecutar tests en modules/
for file in tests/modules/*.rcc; do
    if [ -f "$file" ]; then
        run_test "$file"
    fi
done

# Resumen final
echo "=========================================="
echo "RESUMEN DE TESTS"
echo "=========================================="
echo "Total:   $TOTAL tests"
echo "Passed:  $PASSED tests ✅"
echo "Failed:  $FAILED tests ❌"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "🎉 TODOS LOS TESTS PASARON!"
    exit 0
else
    echo "⚠️  Algunos tests fallaron"
    exit 1
fi
