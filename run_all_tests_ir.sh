#!/bin/bash

# Script to run all tests with IR mode enabled
# Usage: ./run_all_tests_ir.sh

RACCOON="./target/release/raccoon"
TEST_DIR="tests"
FAILED_TESTS=""
PASSED_TESTS=0
FAILED_COUNT=0
TOTAL_TESTS=0

echo "Running all tests with IR mode enabled..."
echo "==========================================="
echo ""

for test_file in "$TEST_DIR"/*.rcc; do
    if [ -f "$test_file" ]; then
        TOTAL_TESTS=$((TOTAL_TESTS + 1))
        test_name=$(basename "$test_file")

        echo -n "Testing: $test_name ... "

        # Run the test with --use-ir flag and capture output
        if timeout 10 "$RACCOON" --use-ir "$test_file" > /tmp/test_output_$$.txt 2>&1; then
            echo "✓ PASSED"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo "✗ FAILED"
            FAILED_TESTS="$FAILED_TESTS\n  - $test_name"
            FAILED_COUNT=$((FAILED_COUNT + 1))

            # Show error output
            echo "    Error output:"
            head -5 /tmp/test_output_$$.txt | sed 's/^/    /'
        fi

        rm -f /tmp/test_output_$$.txt
    fi
done

echo ""
echo "==========================================="
echo "Test Results:"
echo "  Total tests: $TOTAL_TESTS"
echo "  Passed: $PASSED_TESTS"
echo "  Failed: $FAILED_COUNT"
echo ""

if [ $FAILED_COUNT -gt 0 ]; then
    echo "Failed tests:"
    echo -e "$FAILED_TESTS"
    exit 1
else
    echo "All tests passed! ✓"
    exit 0
fi
