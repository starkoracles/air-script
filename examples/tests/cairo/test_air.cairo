%lang starknet

from starkware.cairo.common.cairo_builtins import BitwiseBuiltin
from starkware.cairo.common.hash import HashBuiltin
from starkware.cairo.common.alloc import alloc
from examples.tests.cairo.example import EvaluationFrame, evaluate_transition_0

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
        for i in range(4):
            print(i, memory[ids.t_evaluations_ptr + i])
            assert memory[ids.t_evaluations_ptr + i] == expected_t_evals[i]
    %}
    return ();
}
