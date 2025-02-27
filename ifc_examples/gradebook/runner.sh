#!/bin/bash
set -e

echo "Building the file" > $2

TRIALS=100

if [ "$3" == "counters" ]; then
    cp ../../ifc_library/macros/Cargo-counter.toml ../../ifc_library/macros/Cargo.toml
    cp ../../ifc_library/secret_structs/Cargo-counter.toml ../../ifc_library/secret_structs/Cargo.toml
    TRIALS=1
else
    cp ../../ifc_library/macros/Cargo-no-counter.toml ../../ifc_library/macros/Cargo.toml
    cp ../../ifc_library/secret_structs/Cargo-no-counter.toml ../../ifc_library/secret_structs/Cargo.toml
fi
cargo build --release
count=0
while [ $count -lt $TRIALS ]; do
    echo "Testing round ${count}" >> $2
    cargo run --release -- ifc_$1 >> $2
    cargo run --release -- non_ifc_$1 >> $2
    count=$((count+1))
done
if [ $TRIALS != "1" ]; then
  python3 results-parser.py $2 >> $2
fi
