// Air name ExampleAir 1 segments
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
  frame: EvaluationFrame,
  t_evaluations: felt*,
  periodic_row: felt*,
) {
  alloc_locals;
  let cur = frame.current;
  let nxt = frame.next;
// TRANSITION CONSTRAINTS

  // ((cur[0] ^ 2) - cur[0])
  let v3 = cur[0];
  let v1 = pow_g(v3, 2);
  let v2 = cur[0];
  let v0 = sub_g(v1, v2);
  assert t_evaluations[0] = v0;
  // deg = 2

  // ((periodic_row[0] * (nxt[0] - cur[0])) - 0)
  let v7 = periodic_row[0];
  let v9 = nxt[0];
  let v10 = cur[0];
  let v8 = sub_g(v9, v10);
  let v5 = mul_g(v7, v8);
  let v6 = 0;
  let v4 = sub_g(v5, v6);
  assert t_evaluations[1] = v4;
  // deg = 1

  // (((1 - cur[0]) * ((cur[3] - cur[1]) - cur[2])) - 0)
  let v16 = 1;
  let v17 = cur[0];
  let v14 = sub_g(v16, v17);
  let v20 = cur[3];
  let v21 = cur[1];
  let v18 = sub_g(v20, v21);
  let v19 = cur[2];
  let v15 = sub_g(v18, v19);
  let v12 = mul_g(v14, v15);
  let v13 = 0;
  let v11 = sub_g(v12, v13);
  assert t_evaluations[2] = v11;
  // deg = 2

  // ((cur[0] * (cur[3] - (cur[1] * cur[2]))) - 0)
  let v25 = cur[0];
  let v27 = cur[3];
  let v29 = cur[1];
  let v30 = cur[2];
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
  frame: EvaluationFrame,
  b_evaluations: felt*,
  stack_inputs: felt*,
  stack_outputs: felt*,
) {
  alloc_locals;
  let cur = frame.current;
// BOUNDARY CONSTRAINTS

// First Row

  assert b_evaluations[0] = 0;

  // (cur[1] - stack_inputs[1])
  let v1 = cur[1];
  let v2 = stack_inputs[1];
  let v0 = sub_g(v1, v2);
  assert b_evaluations[1] = v0;
  // deg = 1, Domain: the first row

  // (cur[2] - stack_inputs[0])
  let v4 = cur[2];
  let v5 = stack_inputs[0];
  let v3 = sub_g(v4, v5);
  assert b_evaluations[2] = v3;
  // deg = 1, Domain: the first row

  // (cur[3] - stack_inputs[2])
  let v7 = cur[3];
  let v8 = stack_inputs[2];
  let v6 = sub_g(v7, v8);
  assert b_evaluations[3] = v6;
  // deg = 1, Domain: the first row

// Last Row

  assert b_evaluations[4] = 0;

  // (cur[1] - stack_outputs[0])
  let v10 = cur[1];
  let v11 = stack_outputs[0];
  let v9 = sub_g(v10, v11);
  assert b_evaluations[5] = v9;
  // deg = 1, Domain: the last row

  // (cur[2] - stack_outputs[1])
  let v13 = cur[2];
  let v14 = stack_outputs[1];
  let v12 = sub_g(v13, v14);
  assert b_evaluations[6] = v12;
  // deg = 1, Domain: the last row

  // (cur[3] - stack_outputs[2])
  let v16 = cur[3];
  let v17 = stack_outputs[2];
  let v15 = sub_g(v16, v17);
  assert b_evaluations[7] = v15;
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
  z: felt, 
) -> felt {
  alloc_locals;
  local sum_0 = 0;

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
  target_degree: felt,
  blowup_factor: felt,
  coeffs_boundary_a: felt*,
  coeffs_boundary_b: felt*, 
  b_evaluations: felt*, 
  trace_domain_generator: felt, 
  npub_steps: felt, 
  z: felt, 
) -> felt {
  alloc_locals;
  let composition_degree = trace_length * blowup_factor - 1;
  let trace_poly_degree = trace_length  - 1;
  let divisor_degree = 1;
  let target_degree =  composition_degree + divisor_degree;
  let first_z = z - 1;

  let g = trace_domain_generator;

  let gn = pow_g(g,npub_steps - 1);

  let last_z = z - gn;

  local first_sum_0 = 0;
  local last_sum_0 = 0;

  // Merge degree 1
  let evaluation_degree = 1 * (trace_length - 1);
  let degree_adjustment = target_degree - evaluation_degree;
  let xp = pow_g(z, degree_adjustment);

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
  local last_sum_1 = add_g(last_sum_0,v3);

  // Include boundary 4
  let v1 = mul_g(coeffs_boundary_b[4],  xp);
  let v2 = add_g(coeffs_boundary_a[4], v1);
  let v3 = mul_g(v2, b_evaluations[4]);
  local last_sum_2 = add_g(last_sum_1,v3);

  // Include boundary 5
  let v1 = mul_g(coeffs_boundary_b[5],  xp);
  let v2 = add_g(coeffs_boundary_a[5], v1);
  let v3 = mul_g(v2, b_evaluations[5]);
  local last_sum_3 = add_g(last_sum_2,v3);

  let first = div_g(first_sum_3,first_z);
  let last = div_g(last_sum_3,last_z);
  return add_g(first,last);
}

// PUT CONSTRAINT EVALUATION FUNCTION HERE

