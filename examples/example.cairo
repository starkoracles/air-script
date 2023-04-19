// Hello, Cairo0!
// Air name ExampleAir 2 segments
from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.memcpy import memcpy
struct EvaluationFrame {
  current_len: felt,
  current: felt*,
  next_len: felt,
  next: felt*,
}
// Segment 0  size 4
// Segment 1  size 1
// // SEGMENT 0 size 4
// ===============================================

  // Boundary   constraints (6) 
  // ----------------
    // #0: root node 2 Domain: the first row
    //    Sub(TraceElement(1), PublicInput)
    // #1: root node 5 Domain: the first row
    //    Sub(TraceElement(2), PublicInput)
    // #2: root node 8 Domain: the first row
    //    Sub(TraceElement(3), PublicInput)
    // #3: root node 10 Domain: the last row
    //    Sub(TraceElement(1), PublicInput)
    // #4: root node 12 Domain: the last row
    //    Sub(TraceElement(2), PublicInput)
    // #5: root node 14 Domain: the last row
    //    Sub(TraceElement(3), PublicInput)

  // Integrity   constraints (4)
  // ----------------
    // #0: root node 20 Domain: every row
    //   Sub(Exp(TraceElement(0), 2), TraceElement(0))
    assert t_evalations[0] = (exp(cur[0], 2) - cur[0]);
    // #1: root node 26 Domain: every frame of 2 consecutive rows
    //   Sub(Mul(PeriodicColumn, Sub(TraceElement(0+1), TraceElement(0))), 0)
    assert t_evalations[1] = ((PeriodicColumn * (nxt[1] - cur[0])) - 0);
    // #2: root node 31 Domain: every row
    //   Sub(Mul(Sub(1, TraceElement(0)), Sub(Sub(TraceElement(3), TraceElement(1)), TraceElement(2))), 0)
    assert t_evalations[2] = (((1 - cur[0]) * ((cur[3] - cur[1]) - cur[2])) - 0);
    // #3: root node 35 Domain: every row
    //   Sub(Mul(TraceElement(0), Sub(TraceElement(3), Mul(TraceElement(1), TraceElement(2)))), 0)
    assert t_evalations[3] = ((cur[0] * (cur[3] - (cur[1] * cur[2]))) - 0);
// SEGMENT 1 size 1
// ===============================================

  // Boundary   constraints (1) 
  // ----------------
    // #0: root node 17 Domain: the first row
    //    Sub(TraceElement(0), 1)

  // Integrity   constraints (1)
  // ----------------
    // #0: root node 40 Domain: every frame of 2 consecutive rows
    //   Sub(TraceElement(0+1), Mul(TraceElement(0), Add(TraceElement(3), RandomValue)))
    assert t_evalations[0] = (nxt[1] - (cur[0] * (cur[3] + RandomValue)));

