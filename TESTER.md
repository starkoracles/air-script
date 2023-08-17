# Test procedure

The latest machinery is as follows:

1. Implement a VM. Currently it must be a Felix script, this will change shortly to a shell script.
2. The vm file must be named `test/input/<vmname>_vm.flx`
4. Implement an airscript specification.
5. The airscript file must be named `test/input/<vmname>.air`
6. Implement a Winterfell mainline,
7. The mainline must be named `test/input/<vmname>_winterfell_main.rs`
8. Implement one or more public input specifications.
9. The inputs files must be named `test/input/<vmname>.public_input_<input_case>`
10. Run ```flx tester.flx test/input/<vmname>```

# VM specs
1. The VM must accept exactly 4 command line arguments.
2. These are
    1. `--input_file=<input_file_name>`
    2. `--trace_file=<trace_file_name>`
    3. `--output_file=<output_file_name>`
    4. `--trace_length=<trace_length>`

4. It must read the input file and use that to initialise the VM.
5. It must write the trace to the trace file.
6. It must write the result to the output file.
7. It should run exactly the specified number of steps.

The format for the input and output files is a single line of space separated integers
The format for the trace is a sequence of lines of space separated integers

# Winterfell mainline specs
Sorry this thing is a mess, please just copy and edit an existing case.
I hope to eliminate the need to hand write this file.

# Prerequisites
1. You have to have Felix up and running. 
2. It requires Ocaml 4.12, Python 3.x, and recent g++ or clang supporting -std=c++17.
3. You need recent version of Rust
4. You need Protostar version: 0.10.0 it will NOT work with newer version at present.
5. You need cairo compiler, I use these:
6. Cairo-lang version: 0.11.0.1
7. Cairo 1 compiler version: 1.0.0a6

