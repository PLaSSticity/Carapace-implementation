#!/bin/bash                                                                                  
set -uo pipefail

SERVO_DIR="servo-unmodified"
readonly SERVO_DIR

SERVO_BIN="servo-unmodified/target/release/servo"
readonly SERVO_BIN

SERVO_IFC_DIR="servo-ifc"
readonly SERVO_IFC_DIR

SERVO_IFC_BIN="servo-ifc/target/release/servo"
readonly SERVO_IFC_BIN

RESULT_DIR="new_servo_results"
readonly RESULT_DIR

    echo "running tests"
    echo "$#"
    file="$1"
    echo "hello $file"
    count=0
    while [ $count -lt 1000 ]; do
        echo "ifc $count"
        pushd "servo-ifc" >/dev/null 2>/dev/null
	timeout -k 10 120 ./mach test-wpt --log-mach - --release --binary-arg "-o junk" $file
        #./mach test-wpt --release $line #>> ../${RESULT_DIR}/all_results_$count.txt
	pkill servo
	popd >/dev/null 2>/dev/null
	echo ""
	echo "non-ifc $count"
	pushd "servo-unmodified" >/dev/null 2>/dev/null
	timeout -k 10 120 ./mach test-wpt --log-mach - --release --binary-arg "-o junk" $file
	pkill servo
	popd >/dev/null 2>/dev/null
	echo ""
	count=$((count+1))
    done
