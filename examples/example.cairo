
from starkware.cairo.common.registers import get_ap, get_fp_and_pc

// 2^64 - 2^32 - 1;
const PG = 18446744069414584321;

// multiply to felts modulo PG, these numbers must be smaller than PG
func mul_g{range_check_ptr}(a: felt, b: felt) -> felt {
    // add range checks for a, b
    let res = a * b;

    let r = [range_check_ptr];
    let q = [range_check_ptr + 1];
    let range_check_ptr = range_check_ptr + 2;

    %{
        ids.r = ids.res % ids.PG
        ids.q = ids.res // ids.PG
    %}
    assert q * PG + r = res;
    return r;
}

func add_g{range_check_ptr}(a: felt, b: felt) -> felt {
    let res = a + b;

    let r = [range_check_ptr];
    let q = [range_check_ptr + 1];
    let range_check_ptr = range_check_ptr + 2;

    %{
        ids.r = ids.res % ids.PG
        ids.q = ids.res // ids.PG
    %}
    assert q * PG + r = res;
    return r;
}

func inv_g{range_check_ptr}(a: felt) -> felt {
    let inv = [range_check_ptr];
    let range_check_ptr = range_check_ptr + 1;

    %{
        def mul_g(a, b):
            return (a * b) % ids.PG

        def square_g(a):
            return (a ** 2) % ids.PG
            
        def exp_acc(base, tail, exp_bits):
            result = base
            for i in range(exp_bits):
                result = square_g(result)
            return mul_g(result, tail)
        # compute base^(M - 2) using 72 multiplications
        # M - 2 = 0b1111111111111111111111111111111011111111111111111111111111111111
        a = ids.a
        # compute base^11
        t2 = mul_g(square_g(a), a)

        # compute base^111
        t3 = mul_g(square_g(t2), a)

        # compute base^111111 (6 ones)
        t6 = exp_acc(t3, t3, 3)

        # compute base^111111111111 (12 ones)
        t12 = exp_acc(t6, t6, 6)

        # compute base^111111111111111111111111 (24 ones)
        t24 = exp_acc(t12, t12, 12)

        # compute base^1111111111111111111111111111111 (31 ones)
        t30 = exp_acc(t24, t6, 6)
        t31 = mul_g(square_g(t30), a)

        # compute base^111111111111111111111111111111101111111111111111111111111111111
        t63 = exp_acc(t31, t31, 32)

        # compute base^1111111111111111111111111111111011111111111111111111111111111111
        ids.inv = mul_g(square_g(t63), a)
    %}
    assert mul_g(inv, a) = 1;
    return inv;
}

func div_g{range_check_ptr}(a: felt, b: felt) -> felt {
    let inv = inv_g(b);
    return mul_g(a, inv);
}

func sub_g{range_check_ptr}(a: felt, b: felt) -> felt {
    let r = [range_check_ptr];
    let a_greater_than_b = [range_check_ptr + 1];
    let range_check_ptr = range_check_ptr + 2;

    %{
        if ids.a < ids.b:
            ids.r = ids.a + ids.PG - ids.b
            ids.a_greater_than_b = 0
        else:
            ids.r = ids.a - ids.b
            ids.a_greater_than_b = 1
    %}

    if (a_greater_than_b == 1) {
        assert r = a - b;
    } else {
        assert r + b = a + PG;
    }
    return r;
}

func pow_g_loop{range_check_ptr}(base, exp, res) -> felt {
    if (exp == 0) {
        return res;
    }

    let base_square = mul_g(base, base);

    let bit = [range_check_ptr];
    let range_check_ptr = range_check_ptr + 1;

    %{ ids.bit = (ids.exp % ids.PG) & 1 %}
    if (bit == 1) {
        // odd case
        let tmp = exp - 1;
        let new_exp = tmp / 2;
        let r = mul_g(base, res);
        return pow_g_loop(base_square, new_exp, r);
    } else {
        // even case
        let new_exp = exp / 2;
        return pow_g_loop(base_square, new_exp, res);
    }
}

// Returns base ** exp % PG, for 0 <= exp < 2**63.
func pow_g{range_check_ptr}(base, exp) -> felt {
    if (exp == 0) {
        return 1;
    }

    if (base == 0) {
        return 0;
    }

    return pow_g_loop(base, exp, 1);
}


// Air name ExampleAir 1 segments
from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.memcpy import memcpy


func air_instance_new{range_check_ptr}(proof: StarkProof*, pub_inputs: PublicInputs*) -> AirInstance {
    alloc_locals;
    let (aux_segment_widths: felt*) = alloc();
    let (aux_segment_rands: felt*) = alloc();

    let (power) = pow(2, TWO_ADICITY - proof.context.log_trace_length);
    let (trace_domain_generator) = pow(TWO_ADIC_ROOT_OF_UNITY, power);
    
    let log_lde_domain_size = proof.context.options.log_blowup_factor + proof.context.log_trace_length;
    let (power) = pow(2, TWO_ADICITY - log_lde_domain_size);
    let (lde_domain_generator) = pow(TWO_ADIC_ROOT_OF_UNITY, power);

    // Configured for SomeAir
    let res = AirInstance(
        main_segment_width=1,
        aux_trace_width=2,
        aux_segment_widths=aux_segment_widths,
        aux_segment_rands=aux_segment_rands,
        num_aux_segments=3,
        context=proof.context,
        num_transition_constraints=4,
        num_assertions=5,
        ce_blowup_factor=4,
        eval_frame_size=2,
        trace_domain_generator=trace_domain_generator,
        lde_domain_generator=lde_domain_generator,
        pub_inputs=pub_inputs,
    );
    return res;
}
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

func degrees_0() -> felt* {
  let (d) = alloc();
  assert [d + 0] = 2;
  assert [d + 1] = 1;
  assert [d + 2] = 2;
  assert [d + 3] = 3;

  return (d);
}

func evaluate_boundary_0{range_check_ptr} (
  frame: EvaluationFrame,
  b_evaluations: felt*,
  public: felt*,
) {
  alloc_locals;
  let cur = frame.current;
  let nxt = frame.next;
// BOUNDARY CONSTRAINTS

  // (cur[1] - public[0])
  let v32 = cur[1];
  let v33 = public[0];
  let v31 = sub_g(v32, v33);
  assert b_evaluations[0] = v31;

  // (cur[2] - public[1])
  let v35 = cur[2];
  let v36 = public[1];
  let v34 = sub_g(v35, v36);
  assert b_evaluations[1] = v34;

  // (cur[3] - public[2])
  let v38 = cur[3];
  let v39 = public[2];
  let v37 = sub_g(v38, v39);
  assert b_evaluations[2] = v37;

  // (cur[1] - public[0])
  let v41 = cur[1];
  let v42 = public[0];
  let v40 = sub_g(v41, v42);
  assert b_evaluations[3] = v40;

  // (cur[2] - public[1])
  let v44 = cur[2];
  let v45 = public[1];
  let v43 = sub_g(v44, v45);
  assert b_evaluations[4] = v43;

  // (cur[3] - public[2])
  let v47 = cur[3];
  let v48 = public[2];
  let v46 = sub_g(v47, v48);
  assert b_evaluations[5] = v46;


  return ();
}

