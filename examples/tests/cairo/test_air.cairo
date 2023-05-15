
// GENERATED Mon May 15 10:55:39 2023

%lang starknet

from starkware.cairo.common.cairo_builtins import BitwiseBuiltin
from starkware.cairo.common.hash import HashBuiltin
from starkware.cairo.common.alloc import alloc
from examples.tests.cairo.example import EvaluationFrame, evaluate_transition_0, evaluate_boundary_0

@external
func test_transition_constraints{range_check_ptr}() {
    alloc_locals;
    let (periodic: felt*) = alloc();
    assert periodic[0] = 5903898543833973905;

    let (current_frame: felt*) = alloc();
    let (next_frame: felt*) = alloc();
    %{
        current_frame_ptr = ids.current_frame
        for val in [1, 18412444050014700581, 8774229257757275069, 3918954966599443370]:
               memory[current_frame_ptr] = val
               current_frame_ptr = current_frame_ptr + 1

        next_frame_ptr = ids.next_frame
        for next_val in [1, 10303400192442738851, 16207298736276712756, 3979962108257833552]:
            memory[next_frame_ptr] = next_val
            next_frame_ptr = next_frame_ptr + 1
    %}

    let (t_evaluations: felt*) = alloc();
    local t_evaluations_ptr: felt* = t_evaluations;
    let main_frame = EvaluationFrame(4, current_frame, 4, next_frame);

    evaluate_transition_0(main_frame, t_evaluations, periodic);
    %{
        expected_t_evals = [0, 0, 0, 6351218673503739251]
        print("Transition Evaluations")
        for i in range(4):
            print(i, memory[ids.t_evaluations_ptr + i])
            assert memory[ids.t_evaluations_ptr + i] == expected_t_evals[i]
    %}

    let (b_current_frame: felt*) = alloc();
    let (b_next_frame: felt*) = alloc();

  // public inputs

    let (stack_inputs: felt*) = alloc();
    assert stack_inputs[0] = 1;
    assert stack_inputs[1] =  1;
    assert stack_inputs[2] =  1;
    assert stack_inputs[3] =  1;
    assert stack_inputs[4] =  1;
    assert stack_inputs[5] =  1;
    assert stack_inputs[6] =  1;
    assert stack_inputs[7] =  1;
    assert stack_inputs[8] =  1;
    assert stack_inputs[9] =  1;
    assert stack_inputs[10] =  1;
    assert stack_inputs[11] =  1;
    assert stack_inputs[12] =  1;
    assert stack_inputs[13] =  1;
    assert stack_inputs[14] =  1;
    assert stack_inputs[15] =  1;

    let (stack_outputs: felt*) = alloc();
    assert stack_outputs[0] =  7;
    assert stack_outputs[1] =  8;
    assert stack_outputs[2] =  56;
    assert stack_outputs[3] =  1;
    assert stack_outputs[4] =  1;
    assert stack_outputs[5] =  1;
    assert stack_outputs[6] =  1;
    assert stack_outputs[7] =  1;
    assert stack_outputs[8] =  1;
    assert stack_outputs[9] =  1;
    assert stack_outputs[10] =  1;
    assert stack_outputs[11] =  1;
    assert stack_outputs[12] =  1;
    assert stack_outputs[13] =  1;
    assert stack_outputs[14] =  1;
    assert stack_outputs[15] =  1;

  // Trace
    assert b_current_frame[0] = 0;
    assert b_current_frame[1] = 18412444050014700581;
    assert b_current_frame[2] = 8774229257757275069;
    assert b_current_frame[3] = 3918954966599443370;
    assert b_current_frame[4] = 0;
    assert b_current_frame[5] = 18412444050014700581;
    assert b_current_frame[6] = 8774229257757275069;
    assert b_current_frame[7] = 3918954966599443370;

    let b_frame = EvaluationFrame(8, b_current_frame, 8, next_frame);

    let (b_evaluations: felt*) = alloc();
    local b_evaluations_ptr: felt* = b_evaluations;
    evaluate_boundary_0(b_frame, b_evaluations, stack_inputs, stack_outputs);

    %{
        expected_b_evals = [0, 18412444050014700580, 8774229257757275068, 3918954966599443369, 0, 18412444050014700574, 8774229257757275061, 3918954966599443314, ]
        print("Boundary Evaluations")
        for i in range(8):
            print(i, memory[ids.b_evaluations_ptr + i])
            assert memory[ids.b_evaluations_ptr + i] == expected_b_evals[i]
    %}


    return ();
}
