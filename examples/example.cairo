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
  row: felt,
}

// SEGMENT 0 size 4
// ===============================================
func evaluate_transition_0 (
  frame: EvaluationFrame,
  t_evaluations: felt*,
  periodic: felt*,
) {
  alloc_locals;
  let cur = frame.current;
  let nxt = frame.next;
  let row = frame.row;
  assert t_evaluations[0] = (exp(cur[0], 2) - cur[0]);
  assert t_evaluations[1] = ((periodic[0 + mod(row, 8)] * (nxt[1] - cur[0])) - 0);
  assert t_evaluations[2] = (((1 - cur[0]) * ((cur[3] - cur[1]) - cur[2])) - 0);
  assert t_evaluations[3] = ((cur[0] * (cur[3] - (cur[1] * cur[2]))) - 0);

  return ();
}

// SEGMENT 1 size 1
// ===============================================
func evaluate_transition_1 (
  frame: EvaluationFrame,
  t_evaluations: felt*,
  periodic: felt*,
  rand: felt*,
) {
  alloc_locals;
  let cur = frame.current;
  let nxt = frame.next;
  let row = frame.row;
  assert t_evaluations[0] = (nxt[1] - (cur[0] * (cur[3] + rand[0])));

  return ();
}

