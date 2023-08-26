// TEST: Sat Aug 26 09:16:53 2023 UTC
// Air name AirType 1 segments
from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.memcpy import memcpy
from math_goldilocks import add_g, sub_g, mul_g, pow_g, div_g

struct EvaluationFrame {
  current_len: felt,
  current: felt*,
  next_len: felt,
  next: felt*,
}

// SEGMENT 0 size 4
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

  // ((cur_0[0] ^ 2) - cur_0[0])
  let v3 = cur_0[0];
  let v1 = pow_g(v3, 2);
  let v2 = cur_0[0];
  let v0 = sub_g(v1, v2);
  assert t_evaluations[0] = v0;
  // deg = 2

  // ((periodic_row[0] * (nxt_0[0] - cur_0[0])) - 0)
  let v7 = periodic_row[0];
  let v9 = nxt_0[0];
  let v10 = cur_0[0];
  let v8 = sub_g(v9, v10);
  let v5 = mul_g(v7, v8);
  let v6 = 0;
  let v4 = sub_g(v5, v6);
  assert t_evaluations[1] = v4;
  // deg = 1

  // (((1 - cur_0[0]) * ((cur_0[3] - cur_0[1]) - cur_0[2])) - 0)
  let v16 = 1;
  let v17 = cur_0[0];
  let v14 = sub_g(v16, v17);
  let v20 = cur_0[3];
  let v21 = cur_0[1];
  let v18 = sub_g(v20, v21);
  let v19 = cur_0[2];
  let v15 = sub_g(v18, v19);
  let v12 = mul_g(v14, v15);
  let v13 = 0;
  let v11 = sub_g(v12, v13);
  assert t_evaluations[2] = v11;
  // deg = 2

  // ((cur_0[0] * (cur_0[3] - (cur_0[1] * cur_0[2]))) - 0)
  let v25 = cur_0[0];
  let v27 = cur_0[3];
  let v29 = cur_0[1];
  let v30 = cur_0[2];
  let v28 = mul_g(v29, v30);
  let v26 = sub_g(v27, v28);
  let v23 = mul_g(v25, v26);
  let v24 = 0;
  let v22 = sub_g(v23, v24);
  assert t_evaluations[3] = v22;
  // deg = 3


  return ();
}

func evaluate_boundary_0{range_check_ptr} (
  frame_0: EvaluationFrame,
  b_evaluations: felt*,
  inputs: felt*,
  outputs: felt*,
) {
  alloc_locals;
  let first_0 = frame_0.current;
  let last_0 = frame_0.next;
// BOUNDARY CONSTRAINTS

  // (first_0[0] - inputs[0])
  let v1 = first_0[0];
  let v2 = inputs[0];
  let v0 = sub_g(v1, v2);
  assert b_evaluations[0] = v0;
  // deg = 1, Domain: the first row

  // (first_0[1] - inputs[1])
  let v4 = first_0[1];
  let v5 = inputs[1];
  let v3 = sub_g(v4, v5);
  assert b_evaluations[1] = v3;
  // deg = 1, Domain: the first row

  // (first_0[2] - inputs[2])
  let v7 = first_0[2];
  let v8 = inputs[2];
  let v6 = sub_g(v7, v8);
  assert b_evaluations[2] = v6;
  // deg = 1, Domain: the first row

  // (first_0[3] - inputs[3])
  let v10 = first_0[3];
  let v11 = inputs[3];
  let v9 = sub_g(v10, v11);
  assert b_evaluations[3] = v9;
  // deg = 1, Domain: the first row

  // (last_0[0] - outputs[0])
  let v13 = last_0[0];
  let v14 = outputs[0];
  let v12 = sub_g(v13, v14);
  assert b_evaluations[4] = v12;
  // deg = 1, Domain: the last row

  // (last_0[1] - outputs[1])
  let v16 = last_0[1];
  let v17 = outputs[1];
  let v15 = sub_g(v16, v17);
  assert b_evaluations[5] = v15;
  // deg = 1, Domain: the last row

  // (last_0[2] - outputs[2])
  let v19 = last_0[2];
  let v20 = outputs[2];
  let v18 = sub_g(v19, v20);
  assert b_evaluations[6] = v18;
  // deg = 1, Domain: the last row

  // (last_0[3] - outputs[3])
  let v22 = last_0[3];
  let v23 = outputs[3];
  let v21 = sub_g(v22, v23);
  assert b_evaluations[7] = v21;
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

  // Include transition 1
  let v1 = mul_g(coeffs_transition_b[1],  xp);
  let v2 = add_g(coeffs_transition_a[1], v1);
  let v3 = mul_g(v2, t_evaluations[1]);
  local sum_1 = add_g(sum_0,v3);

  // Merge degree 2
  let evaluation_degree = 2 * (trace_length - 1);
  let degree_adjustment = target_degree - evaluation_degree;
  let xp = pow_g(x, degree_adjustment);

  // Include transition 0
  let v1 = mul_g(coeffs_transition_b[0],  xp);
  let v2 = add_g(coeffs_transition_a[0], v1);
  let v3 = mul_g(v2, t_evaluations[0]);
  local sum_2 = add_g(sum_1,v3);

  // Include transition 2
  let v1 = mul_g(coeffs_transition_b[2],  xp);
  let v2 = add_g(coeffs_transition_a[2], v1);
  let v3 = mul_g(v2, t_evaluations[2]);
  local sum_3 = add_g(sum_2,v3);

  // Merge degree 3
  let evaluation_degree = 3 * (trace_length - 1);
  let degree_adjustment = target_degree - evaluation_degree;
  let xp = pow_g(x, degree_adjustment);

  // Include transition 3
  let v1 = mul_g(coeffs_transition_b[3],  xp);
  let v2 = add_g(coeffs_transition_a[3], v1);
  let v3 = mul_g(v2, t_evaluations[3]);
  local sum_4 = add_g(sum_3,v3);

  return div_g(sum_4,z);
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
 %{
    print('CAIRO Trace length ', ids.trace_poly_degree)
    print('CAIRO blowup factor ', ids.blowup_factor)
    print('CAIRO Composition degree ', ids.composition_degree)
    print('CAIRO Divisor degree ', ids.divisor_degree)
    print('CAIRO Target degree ', ids.target_degree)
 %}
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
  %{
    print('CAIRO evaluation degree ', ids.evaluation_degree)
    print('CAIRO degree adjustment ', ids.degree_adjustment)
  %}
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
  local first_sum_3 = add_g(first_sum_2,v3);

  // Include boundary 3
  let v1 = mul_g(coeffs_boundary_b[3],  xp);
  let v2 = add_g(coeffs_boundary_a[3], v1);
  let v3 = mul_g(v2, b_evaluations[3]);
  local first_sum_4 = add_g(first_sum_3,v3);

  // Include boundary 4
  let v1 = mul_g(coeffs_boundary_b[4],  xp);
  let v2 = add_g(coeffs_boundary_a[4], v1);
  let v3 = mul_g(v2, b_evaluations[4]);
  local last_sum_1 = add_g(last_sum_0,v3);

  // Include boundary 5
  let v1 = mul_g(coeffs_boundary_b[5],  xp);
  let v2 = add_g(coeffs_boundary_a[5], v1);
  let v3 = mul_g(v2, b_evaluations[5]);
  local last_sum_2 = add_g(last_sum_1,v3);

  // Include boundary 6
  let v1 = mul_g(coeffs_boundary_b[6],  xp);
  let v2 = add_g(coeffs_boundary_a[6], v1);
  let v3 = mul_g(v2, b_evaluations[6]);
  local last_sum_3 = add_g(last_sum_2,v3);

  // Include boundary 7
  let v1 = mul_g(coeffs_boundary_b[7],  xp);
  let v2 = add_g(coeffs_boundary_a[7], v1);
  let v3 = mul_g(v2, b_evaluations[7]);
  local last_sum_4 = add_g(last_sum_3,v3);
let first_sum = first_sum_4;
let last_sum = last_sum_4;
  %{
    print('CAIRO final numerator first', ids.first_sum)
    print('CAIRO final numerator  last', ids.last_sum)
  %}

  let first = div_g(first_sum,first_z);
  let last = div_g(last_sum,last_z);
  %{
    print('CAIRO quotient first', ids.first)
    print('CAIRO quotient last', ids.last)
  %}
  let combined = add_g(first,last);
  %{
    print('CAIRO combined ', ids.combined)
  %}
  return combined;
}

// PUT CONSTRAINT EVALUATION FUNCTION HERE

