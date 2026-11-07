#!/bin/sh
# Check that the output of the template macros is deterministic: build a test
# binary that uses them twice, forcing a recompile in between, and check that
# the two binaries are bit-for-bit identical.
set -eu

export CARGO_INCREMENTAL=0
test_file=display-as/tests/with_template.rs

build() {
    cargo test -p display-as --test with_template --no-run --message-format=json 2>/dev/null |
        sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -n 1
}

first=$(build)
first_sum=$(sha256sum "$first" | cut -d' ' -f1)
sleep 1
touch "$test_file" # force the macros to be expanded again
second=$(build)
second_sum=$(sha256sum "$second" | cut -d' ' -f1)

echo "first build:  $first_sum"
echo "second build: $second_sum"
if [ "$first_sum" != "$second_sum" ]; then
    echo "The builds are not reproducible!" >&2
    exit 1
fi
