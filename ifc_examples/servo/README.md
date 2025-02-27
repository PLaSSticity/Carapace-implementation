# Servo
Servo's source code is located [here](https://github.com/PLaSSticity/servo-ifc).
It is out-of-tree because it contains this repository as a submodule, and
because it is a large project.

Branch `shared` contains the Carapace-modified code. Branch `master` contains the unmodified (except for timing infrastructure) version of Servo used to generate results.

## Experiment Environment
Experiments were conducted on a MacBook Pro laptop with a quad-core Intel Core i7-7920HQ at 3.1 GHz with 16 GB of RAM, running macOS 12.7.6.

## Reproducing
Included in this directory are the scripts to reproduce the performance results reported in the paper. These scripts are designed to be placed outside of the Servo directory, in a directory containing both modified and unmodified Servo.

To reproduce our results:
```
# clone the Carapace-modified version of Servo into directory servo-ifc
# clone the unmodified version of Servo into directory servo-unmodified
# create a new directory called new_servo_results
# run the following command
$ ./indiv.sh
```

This experiment takes a long time to complete: multiple days on the machine used to generate results.

Following completion, the results will be stored in directory new_servo_results. A set of times will be including in the files named `results_$number$.txt`, with each file having the test name at the top.

To compute averages and confidence intervals for each file, change the last line of script servo_parser.py to choose the correct file, and then run the script servo_parser.py.

## Run-time Statistics
Statistics such as number of secure blocks and unwraps executed can be enabled by changing the second-to-last line of files `info-flow-library/ifc_library/secret_structs/Cargo.toml` and `info-flow-library/ifc_library/macros/Cargo.toml`. Without statistics enabled, these lines should each be `#default = \["benchmarking"\]`. With statistics enabled, these lines should each be `default = \["benchmarking"\]`.

After changing these lines, each time Carapace-modified Servo is executed, a set of counters will be printed out along with timing info. To view these counters, run the command `cat complete_results_$number$.txt | grep grep joins -C 7` from inside the new_servo_results directory.
