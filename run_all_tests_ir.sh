#!/bin/bash

# Script to run all tests with IR mode enabled with detailed error reporting
# Auto-compiles raccoon if needed (uses debug by default for faster compilation)
# Usage: ./run_all_tests_ir.sh [--verbose] [--debug] [--release]
# Options:
#   --verbose, -v: Show full error output for each test
#   --debug, -d:   Save error logs for further analysis
#   --release, -r: Use release binary (requires full compilation)

TEST_DIR="tests"
LOG_DIR="test_logs"
FAILED_TESTS=""
PASSED_TESTS=0
FAILED_COUNT=0
TOTAL_TESTS=0
TIMEOUT_COUNT=0
VERBOSE=0
DEBUG=0
RELEASE=0

# Parse arguments
for arg in "$@"; do
    case $arg in
        --verbose|-v)
            VERBOSE=1
            ;;
        --debug|-d)
            DEBUG=1
            ;;
        --release|-r)
            RELEASE=1
            ;;
        *)
            echo "Unknown option: $arg"
            exit 1
            ;;
    esac
done

# Determine which binary to use
if [ $RELEASE -eq 1 ]; then
    RACCOON="./target/release/raccoon"
    BUILD_DIR="release"
else
    RACCOON="./target/debug/raccoon"
    BUILD_DIR="debug"
fi

# Check if binary exists, if not compile it
if [ ! -f "$RACCOON" ]; then
    echo "Building raccoon ($BUILD_DIR)..."
    if cargo build $([ $RELEASE -eq 1 ] && echo "--release"); then
        echo "✓ Build successful"
    else
        echo "✗ Build failed"
        exit 1
    fi
    echo ""
fi

# Create log directory if debug mode is enabled
if [ $DEBUG -eq 1 ]; then
    mkdir -p "$LOG_DIR"
    rm -f "$LOG_DIR"/*.log
fi

echo "Running all tests with IR mode enabled..."
echo "=========================================="
echo "Binary: $RACCOON"
echo "Options: Verbose=$VERBOSE Debug=$DEBUG Release=$RELEASE"
echo ""

for test_file in "$TEST_DIR"/*.rcc; do
    if [ -f "$test_file" ]; then
        TOTAL_TESTS=$((TOTAL_TESTS + 1))
        test_name=$(basename "$test_file")
        test_basename="${test_name%.rcc}"

        printf "%-40s " "Testing: $test_name"

        # Run the test with --use-ir flag and capture output
        TEMP_OUT="/tmp/test_output_$$.txt"
        TEMP_ERR="/tmp/test_error_$$.txt"

        if timeout 10 "$RACCOON" --use-ir "$test_file" > "$TEMP_OUT" 2> "$TEMP_ERR"; then
            echo "✓ PASSED"
            PASSED_TESTS=$((PASSED_TESTS + 1))
            rm -f "$TEMP_OUT" "$TEMP_ERR"
        else
            EXIT_CODE=$?

            if [ $EXIT_CODE -eq 124 ]; then
                echo "⏱ TIMEOUT"
                TIMEOUT_COUNT=$((TIMEOUT_COUNT + 1))
                FAILED_TESTS="$FAILED_TESTS\n  - $test_name (TIMEOUT)"
            else
                echo "✗ FAILED (exit code: $EXIT_CODE)"
                FAILED_TESTS="$FAILED_TESTS\n  - $test_name"
            fi

            FAILED_COUNT=$((FAILED_COUNT + 1))

            # Show error output
            if [ -s "$TEMP_ERR" ]; then
                echo "    ┌─ STDERR:"
                if [ $VERBOSE -eq 1 ]; then
                    cat "$TEMP_ERR" | sed 's/^/    │ /'
                else
                    head -10 "$TEMP_ERR" | sed 's/^/    │ /'
                    if [ $(wc -l < "$TEMP_ERR") -gt 10 ]; then
                        echo "    │ ... ($(expr $(wc -l < "$TEMP_ERR") - 10) more lines)"
                    fi
                fi
                echo "    └─"
            fi

            if [ -s "$TEMP_OUT" ]; then
                echo "    ┌─ STDOUT:"
                if [ $VERBOSE -eq 1 ]; then
                    cat "$TEMP_OUT" | sed 's/^/    │ /'
                else
                    head -10 "$TEMP_OUT" | sed 's/^/    │ /'
                    if [ $(wc -l < "$TEMP_OUT") -gt 10 ]; then
                        echo "    │ ... ($(expr $(wc -l < "$TEMP_OUT") - 10) more lines)"
                    fi
                fi
                echo "    └─"
            fi

            # Save logs if debug mode is enabled
            if [ $DEBUG -eq 1 ]; then
                cat "$TEMP_OUT" > "$LOG_DIR/${test_basename}_stdout.log"
                cat "$TEMP_ERR" > "$LOG_DIR/${test_basename}_stderr.log"
                echo "    ✓ Logs saved to: $LOG_DIR/${test_basename}_*.log"
            fi
        fi

        rm -f "$TEMP_OUT" "$TEMP_ERR"
    fi
done

echo ""
echo "=========================================="
echo "Test Results Summary:"
echo "  Total tests:    $TOTAL_TESTS"
echo "  Passed:         $PASSED_TESTS"
echo "  Failed:         $FAILED_COUNT"
if [ $TIMEOUT_COUNT -gt 0 ]; then
    echo "  Timeouts:       $TIMEOUT_COUNT"
fi
SUCCESS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
echo "  Success rate:   $SUCCESS_RATE%"
echo ""

if [ $FAILED_COUNT -gt 0 ]; then
    echo "Failed tests:"
    echo -e "$FAILED_TESTS"

    if [ $DEBUG -eq 1 ]; then
        echo ""
        echo "Debug logs saved in: $LOG_DIR/"
        echo "  View all errors: cat $LOG_DIR/*.log"
        echo "  View specific test: cat $LOG_DIR/<test_name>_stderr.log"
    fi

    exit 1
else
    echo "All tests passed! ✓"
    exit 0
fi
