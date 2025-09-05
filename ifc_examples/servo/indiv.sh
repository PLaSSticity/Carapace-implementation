file="list.txt"
file_counter=1

echo "Compiling modified servo"
pushd "servo-ifc" >/dev/null 2>/dev/null
#./mach build --release
popd >/dev/null 2>/dev/null

echo "Compiling unmodified servo"
pushd "servo-unmodified" >/dev/null 2>/dev/null
#./mach build --release
popd >/dev/null 2>/dev/null

while read -r line; do
    ./individual_runner.sh $line >> new_servo_results/complete_results_$file_counter.txt
    echo "$line" > new_servo_results/results_$file_counter.txt
    cat new_servo_results/complete_results_$file_counter.txt | grep "Elapsed\|ifc " >> new_servo_results/results_$file_counter.txt
    file_counter=$((file_counter+1))
done <$file
