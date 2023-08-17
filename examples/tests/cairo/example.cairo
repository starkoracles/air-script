// TESTCASE: test/input/fib AT: Thu Aug 17 16:16:15 2023 UTC
// Air name FibAir 1 segments
from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.memcpy import memcpy
from math_goldilocks import add_g, sub_g, mul_g, pow_g, div_g

struct EvaluationFrame {
  current_len: felt,
  current: felt*,
  next_len: felt,
  next: felt*,
}

// SEGMENT 0 size 2
// ===============================================
func evaluate_transition_0{range_check_ptr} (
  frame_0: EvaluationFrame,
  t_evaluations: felt*,
  periodic_row: felt*,
) {
  alloc_locals;
  let cur_0 = frame_0.current;
  let nxt_0 = frame_0.next;
// TRANSITION CONSTRAINTS

  // ((nxt_0[0] - cur_0[1]) - 0)
  let v3 = nxt_0[0];
  let v4 = cur_0[1];
  let v1 = sub_g(v3, v4);
  let v2 = 0;
  let v0 = sub_g(v1, v2);
  assert t_evaluations[0] = v0;
  // deg = 1

  // (((nxt_0[1] - cur_0[0]) - cur_0[1]) - 0)
  let v10 = nxt_0[1];
  let v11 = cur_0[0];
  let v8 = sub_g(v10, v11);
  let v9 = cur_0[1];
  let v6 = sub_g(v8, v9);
  let v7 = 0;
  let v5 = sub_g(v6, v7);
  assert t_evaluations[1] = v5;
  // deg = 1


  return ();
}

func evaluate_boundary_0{range_check_ptr} (
  frame_0: EvaluationFrame,
  b_evaluations: felt*,
  fib_inputs: felt*,
  fib_outputs: felt*,
) {
  alloc_locals;
  let first_0 = frame_0.current;
  let last_0 = frame_0.next;
// BOUNDARY CONSTRAINTS

  // (first_0[0] - fib_inputs[0])
  let v1 = first_0[0];
  let v2 = fib_inputs[0];
  let v0 = sub_g(v1, v2);
  assert b_evaluations[0] = v0;
  // deg = 1, Domain: the first row

  // (first_0[1] - fib_inputs[1])
  let v4 = first_0[1];
  let v5 = fib_inputs[1];
  let v3 = sub_g(v4, v5);
  assert b_evaluations[1] = v3;
  // deg = 1, Domain: the first row

  // (last_0[0] - fib_outputs[0])
  let v7 = last_0[0];
  let v8 = fib_outputs[0];
  let v6 = sub_g(v7, v8);
  assert b_evaluations[2] = v6;
  // deg = 1, Domain: the last row

  // (last_0[1] - fib_outputs[1])
  let v10 = last_0[1];
  let v11 = fib_outputs[1];
  let v9 = sub_g(v10, v11);
  assert b_evaluations[3] = v9;
  // deg = 1, Domain: the last row


  return ();
}
// MERGE EVALUATIONS
func merge_transitions_0{range_check_ptr}(
  trace_length: felt,
  target_degree: felt,
  coeffs_transition_a: felt*,
  coeffs_transition_b: felt*, 
  t_evaluations: felt*, 
  x: felt, 
  trace_domain_generator: felt, 
) -> felt {
  alloc_locals;
  local sum_0 = 0;
  // Evaluate transition divisor
  // Airscript only handles boundary constraints on one row
  // So the number of 'exemptions' forming the divisor for transitions is always 1

  let g = trace_domain_generator;
  let v1  = pow_g(x, trace_length);
  let numerator = v1 - 1;
  let v2 = pow_g(g, trace_length - 1);
  let denominator = sub_g(x, v2);
  let z = div_g(numerator, denominator);
  %{
    print('CAIRO transition divisor z = ',ids.z)
  %}

  // Merge degree 1
  let evaluation_degree = 1 * (trace_length - 1);
  let degree_adjustment = target_degree - evaluation_degree;
  let xp = pow_g(x, degree_adjustment);

  // Include transition 0
  let v1 = mul_g(coeffs_transition_b[0],  xp);
  let v2 = add_g(coeffs_transition_a[0], v1);
  let v3 = mul_g(v2, t_evaluations[0]);
  local sum_1 = add_g(sum_0,v3);

  // Include transition 1
  let v1 = mul_g(coeffs_transition_b[1],  xp);
  let v2 = add_g(coeffs_transition_a[1], v1);
  let v3 = mul_g(v2, t_evaluations[1]);
  local sum_2 = add_g(sum_1,v3);

  return div_g(sum_2,z);
}
func merge_boundary_0{range_check_ptr}(
  trace_length: felt,
  blowup_factor: felt,
  coeffs_boundary_a: felt*,
  coeffs_boundary_b: felt*, 
  b_evaluations: felt*, 
  trace_domain_generator: felt, 
  npub_steps: felt, 
  x: felt, 
) -> felt {
  alloc_locals;
  %{
    print('CAIRO OOD evaluation x = ',ids.x)
  %}
  // Evaluate boundary divisor
  let g = trace_domain_generator;
  let composition_degree = trace_length * blowup_factor - 1;
  let trace_poly_degree = trace_length  - 1;
  let divisor_degree = 1;
  let target_degree =  composition_degree + divisor_degree;
  // Evaluate divisor
  let first_z = sub_g(x, 1);
  let v1 = sub_g(trace_length, 1);
  let v2 = pow_g(g, v1);
  let last_z = sub_g(x, v2);
  %{
    print('CAIRO DIVISOR first ',ids.first_z)
    print('CAIRO DIVISOR last  ',ids.last_z)
  %}

  local first_sum_0 = 0;
  local last_sum_0 = 0;

  // Merge degree 1
  let evaluation_degree = 1 * (trace_length - 1);
  let degree_adjustment = target_degree - evaluation_degree;
  let xp = pow_g(x, degree_adjustment);

  // Include boundary 0
  let v1 = mul_g(coeffs_boundary_b[0],  xp);
  let v2 = add_g(coeffs_boundary_a[0], v1);
  let v3 = mul_g(v2, b_evaluations[0]);
  local first_sum_1 = add_g(first_sum_0,v3);

  // Include boundary 1
  let v1 = mul_g(coeffs_boundary_b[1],  xp);
  let v2 = add_g(coeffs_boundary_a[1], v1);
  let v3 = mul_g(v2, b_evaluations[1]);
  local first_sum_2 = add_g(first_sum_1,v3);

  // Include boundary 2
  let v1 = mul_g(coeffs_boundary_b[2],  xp);
  let v2 = add_g(coeffs_boundary_a[2], v1);
  let v3 = mul_g(v2, b_evaluations[2]);
  local last_sum_1 = add_g(last_sum_0,v3);

  // Include boundary 3
  let v1 = mul_g(coeffs_boundary_b[3],  xp);
  let v2 = add_g(coeffs_boundary_a[3], v1);
  let v3 = mul_g(v2, b_evaluations[3]);
  local last_sum_2 = add_g(last_sum_1,v3);

  let first = div_g(first_sum_2,first_z);
  let last = div_g(last_sum_2,last_z);
  return add_g(first,last);
}

// PUT CONSTRAINT EVALUATION FUNCTION HERE

