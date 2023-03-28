// Hello, Cairo0!
// Air name ExampleAir 2 segments
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

  // Validity   constraints (4)
  // ----------------
    // #0: root node 20 Domain: every row
    //   Sub(Exp(TraceElement(0), 2), TraceElement(0))
    // #1: root node 26 Domain: every row
    //   Sub(Mul(PeriodicColumn, Sub(TraceElement(0+1), TraceElement(0))), Constant(0))
    // #2: root node 31 Domain: every row
    //   Sub(Mul(Sub(Constant(1), TraceElement(0)), Add(Sub(TraceElement(3), TraceElement(1)), TraceElement(2))), Constant(0))
    // #3: root node 35 Domain: every row
    //   Sub(Mul(TraceElement(0), Sub(TraceElement(3), Mul(TraceElement(1), TraceElement(2)))), Constant(0))

  // Transition constraints (4)
  // ----------------
    // #0: root node 20 Domain: every row
    //   Sub(Exp(TraceElement(0), 2), TraceElement(0))
    // #1: root node 26 Domain: every row
    //   Sub(Mul(PeriodicColumn, Sub(TraceElement(0+1), TraceElement(0))), Constant(0))
    // #2: root node 31 Domain: every row
    //   Sub(Mul(Sub(Constant(1), TraceElement(0)), Add(Sub(TraceElement(3), TraceElement(1)), TraceElement(2))), Constant(0))
    // #3: root node 35 Domain: every row
    //   Sub(Mul(TraceElement(0), Sub(TraceElement(3), Mul(TraceElement(1), TraceElement(2)))), Constant(0))
// SEGMENT 1 size 1
// ===============================================

  // Boundary   constraints (1) 
  // ----------------
    // #0: root node 17 Domain: the first row
    //    Sub(TraceElement(0), Constant(1))

  // Validity   constraints (1)
  // ----------------
    // #0: root node 40 Domain: every row
    //   Sub(TraceElement(0+1), Mul(TraceElement(0), Add(TraceElement(3), RandomValue)))

  // Transition constraints (1)
  // ----------------
    // #0: root node 40 Domain: every row
    //   Sub(TraceElement(0+1), Mul(TraceElement(0), Add(TraceElement(3), RandomValue)))

