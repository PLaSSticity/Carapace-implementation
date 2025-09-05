# Mk48.io
Mk48.io's modified source code is located [here](https://github.com/PLaSSticity/mk48-ifc). The original source code is located [here](https://github.com/SoftbearStudios/mk48).
It is out-of-tree because it contains this repository as a submodule, and
because it is a large project.

## Experiment Environment
Experiments were conducted on a MacBook Pro laptop with a quad-core Intel Core i7-7920HQ at 3.1 GHz with 16 GB of RAM, running macOS 12.7.6.

## Reproducing
Included in each MK48.io repo's README is a set of instructions on how to run Mk48.io. Follow those instructions to get it running. (Note: in both modified and unmodified Mk48.io, on program start-up, three errors are printed out in the console. These appear to have no effect on the program execution, and are present in the original version we cloned from.

## Run-time Statistics
Statistics such as number of secure blocks and unwraps executed can be enabled by changing the second-to-last line of files `info-flow-library/ifc_library/secret_structs/Cargo.toml` and `info-flow-library/ifc_library/macros/Cargo.toml`. Without statistics enabled, these lines should each be `#default = \["benchmarking"\]`. With statistics enabled, these lines should each be `default = \["benchmarking"\]`.

After changing these lines, each time Carapace-modified mk48.io is executed, a set of counters will be printed out along with timing info. These counters can be viewed from the console after the program has been terminated.
