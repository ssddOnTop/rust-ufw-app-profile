#!/bin/bash

# Configuration for file types to be tested via prettier
FILE_TYPES="{graphql,yml,json,md,ts,js}"

run_cargo_fmt() {
    MODE=$1
    if [ "$MODE" == "check" ]; then
        cargo +nightly fmt -- --check
    else
        cargo +nightly fmt
    fi
    return $?
}

run_cargo_clippy() {
    MODE=$1
    CMD="cargo +nightly clippy --all-targets --all-features"
    if [ "$MODE" == "fix" ]; then
        # if mode is fix first run clippy with --fix flag as usual
        $CMD --fix --allow-staged --allow-dirty
    fi
    # call anyway check command without --fix despite actual $MODE
    # this is to show error as warnings similar to check mode
    # since clippy won't fail on warnings when --fix and -D warnings are used together
    # see https://github.com/rust-lang/rust-clippy/issues/11241
    CMD="$CMD -- -D warnings"
    $CMD
    return $?
}

run_prettier() {
    MODE=$1
    if [ "$MODE" == "check" ]; then
        prettier -c .prettierrc --check "**/*.$FILE_TYPES"
    else
        prettier -c .prettierrc --write "**/*.$FILE_TYPES"
    fi
    return $?
}

# Extract the mode from the argument
if [[ $1 == "--mode="* ]]; then
    MODE=${1#--mode=}
else
    echo "Please specify a mode with --mode=check or --mode=fix"
    exit 1
fi

# Run commands based on mode
case $MODE in
    check|fix)
        run_cargo_fmt $MODE
        FMT_EXIT_CODE=$?
        run_cargo_clippy $MODE
        CLIPPY_EXIT_CODE=$?
        run_prettier $MODE
        PRETTIER_EXIT_CODE=$?
        ;;
    *)
        echo "Invalid mode. Please use --mode=check or --mode=fix"
        exit 1
        ;;
esac

# If any command failed, exit with a non-zero status code
if [ $FMT_EXIT_CODE -ne 0 ] || [ $CLIPPY_EXIT_CODE -ne 0 ] || [ $PRETTIER_EXIT_CODE -ne 0 ]; then
    exit 1
fi
