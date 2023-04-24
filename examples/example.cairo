
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


// Air name ExampleAir 2 segments
from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.memcpy import memcpy

func exp (x:felt, t:felt) -> felt {
  return (1); 
}
func mod(x:felt,y:felt) -> felt {
  return (1); 
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

  local v3 = cur[0];
  pow_g(v3, 2);
  local v1 = [ap - 1];
  local v2 = cur[0];
  sub_g(v1, v2);
  local v0 = [ap - 1];
  assert t_evaluations[0] = v0;

  local v7 = periodic_row[0];
  local v9 = nxt[1];
  local v10 = cur[0];
  sub_g(v9, v10);
  local v8 = [ap - 1];
  mul_g(v7, v8);
  local v5 = [ap - 1];
  local v6 = 0;
  sub_g(v5, v6);
  local v4 = [ap - 1];
  assert t_evaluations[1] = v4;

  local v16 = 1;
  local v17 = cur[0];
  sub_g(v16, v17);
  local v14 = [ap - 1];
  local v20 = cur[3];
  local v21 = cur[1];
  sub_g(v20, v21);
  local v18 = [ap - 1];
  local v19 = cur[2];
  sub_g(v18, v19);
  local v15 = [ap - 1];
  mul_g(v14, v15);
  local v12 = [ap - 1];
  local v13 = 0;
  sub_g(v12, v13);
  local v11 = [ap - 1];
  assert t_evaluations[2] = v11;

  local v25 = cur[0];
  local v27 = cur[3];
  local v29 = cur[1];
  local v30 = cur[2];
  mul_g(v29, v30);
  local v28 = [ap - 1];
  sub_g(v27, v28);
  local v26 = [ap - 1];
  mul_g(v25, v26);
  local v23 = [ap - 1];
  local v24 = 0;
  sub_g(v23, v24);
  local v22 = [ap - 1];
  assert t_evaluations[3] = v22;


  return ();
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

  local v32 = cur[1];
  local v33 = public[0];
  sub_g(v32, v33);
  local v31 = [ap - 1];
  assert b_evaluations[0] = v31;

  local v35 = cur[2];
  local v36 = public[1];
  sub_g(v35, v36);
  local v34 = [ap - 1];
  assert b_evaluations[1] = v34;

  local v38 = cur[3];
  local v39 = public[2];
  sub_g(v38, v39);
  local v37 = [ap - 1];
  assert b_evaluations[2] = v37;

  local v41 = cur[1];
  local v42 = public[0];
  sub_g(v41, v42);
  local v40 = [ap - 1];
  assert b_evaluations[3] = v40;

  local v44 = cur[2];
  local v45 = public[1];
  sub_g(v44, v45);
  local v43 = [ap - 1];
  assert b_evaluations[4] = v43;

  local v47 = cur[3];
  local v48 = public[2];
  sub_g(v47, v48);
  local v46 = [ap - 1];
  assert b_evaluations[5] = v46;


  return ();
}

// SEGMENT 1 size 1
// ===============================================
func evaluate_transition_1{range_check_ptr} (
  frame: EvaluationFrame,
  t_evaluations: felt*,
  periodic_row: felt*,
  rand: felt*,
) {
  alloc_locals;
  let cur = frame.current;
  let nxt = frame.next;
// TRANSITION CONSTRAINTS

  local v50 = nxt[1];
  local v52 = cur[0];
  local v54 = cur[3];
  local v55 = rand[0];
  add_g(v54, v55);
  local v53 = [ap - 1];
  mul_g(v52, v53);
  local v51 = [ap - 1];
  sub_g(v50, v51);
  local v49 = [ap - 1];
  assert t_evaluations[0] = v49;


  return ();
}
func evaluate_boundary_1{range_check_ptr} (
  frame: EvaluationFrame,
  b_evaluations: felt*,
  public: felt*,
  rand: felt*,
) {
  alloc_locals;
  let cur = frame.current;
  let nxt = frame.next;
// BOUNDARY CONSTRAINTS

  local v57 = cur[0];
  local v58 = 1;
  sub_g(v57, v58);
  local v56 = [ap - 1];
  assert b_evaluations[0] = v56;


  return ();
}

