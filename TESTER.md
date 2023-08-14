# Test procedure


Inputs live in the directory
```
test/input
```
Two are required:
```
test/input/example.air
test/input/example_winterfell_main.rs
```

The `example.air` file contains the airscript AIR specification.

The `example_winterfell_main.rs` is the mainline for the Winterfell
package.

This file must create a main trace matrix and public inputs
and call the Wintefell prover and verifier.

The test script must be called by
```
flx tester.flx test/input/example
``` 
Note the `.air` suffix must be omitted.
The layout will be changed soon to allow the same air constraints to
be applied to multiple test cases, by allowing more than one
winterfell mainline.

The test script runs `airscript` to generate both the Winterfell prover and verifier
and the Cairo verifier. The original inputs and output are put in `test/workspace/example`,
where `example` is the basename of the input air file.
``` 
test/workspace/example/example.air	
test/workspace/example/example.rs
test/workspace/example/example.cairo
```

These files are given a time stamp at the top.

The tester then runs a hacked version of Winterfell using
the inputs:
```
test/input/example_winterfell_main.rs
test/workspace/example/example.rs
```
which has been patched to log two things:

1. the actual inputs the verifier receives from the prover
2. some of the data calculated during verification.

The test script generates a cairo mainline to run the generated cairo verifier.
For the main trace there are four functions:

1. evaluate transiction constraints
2. combine transition constraints
3. evaluate boundary constraints 
4. combine boundary constraints

The test script generated cairo mainline supplies the data the Winterfell prover
sent to its verifier, and adds Python assertions to check the 4 functions above
produce the same results as the Winterfell verifier.

The test script then runs the cairo cairo mainline with the airscript generated
cairo functions under protostar version 0.10.0.
`

