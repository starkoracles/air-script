use ir::AirIR;
use ir::ConstantBinding;
use ir::PeriodicColumn;
use ir::PublicInput;

use ir::constraints::AlgebraicGraph;
use ir::constraints::ConstraintRoot;
use ir::constraints::Operation;
use ir::NodeIndex;
use ir::Value;

const glib : &str = r#"
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


"#;


// GENERATE verifier for proof as Cairo v0.4
// ================================================================================================

pub struct CodeGenerator {
    air_name: String,
    segment_widths: Vec<u16>,
    constants: Vec<ConstantBinding>,
    public_inputs: Vec<PublicInput>,
    periodic_columns: Vec<PeriodicColumn>,
    boundary_constraints: Vec<Vec<ConstraintRoot>>,
    integrity_constraints: Vec<Vec<ConstraintRoot>>,
    graph: AlgebraicGraph,
}

impl CodeGenerator {
  // --- CONSTRUCTOR ----------------------------------------------------------------------------

  /// Builds a new Rust scope that represents a Cairo0 Air trait implementation for the
  /// provided AirIR.
  pub fn new(_ir: &AirIR) -> CodeGenerator {
    Self {
      air_name: _ir.air_name.clone(),
      segment_widths: _ir.declarations.trace_segment_widths().to_vec(),
      constants: _ir.declarations.constants().to_vec(),
      public_inputs: _ir.declarations.public_inputs().to_vec(), //Vec<(String, usize)>
      periodic_columns: _ir.declarations.periodic_columns().to_vec(), //Vec<Vec<u64>>`
      boundary_constraints: _ir.constraints.boundary_constraints.clone(), //Constraints
      integrity_constraints: _ir.constraints.integrity_constraints.clone(), //Constraints
      graph: _ir.constraints.graph.clone(),
    }
  }

  pub fn showvalue(&self, x: &Value) -> String {
    match x {
      Value::BoundConstant(_) => "BoundConstant".to_string(),
      Value::InlineConstant(v) => v.to_string(),
      Value::TraceElement(ita) => {
        let offset = &ita.row_offset();
        let colidx = &ita.col_idx().to_string();
        if *offset == 0 {
          "cur[".to_string() + &colidx + "]"
        } else {
          "nxt[".to_string() + &offset.to_string() + "]"
        }
      }
      Value::PeriodicColumn(index, length) => "periodic_row[".to_string() + &index.to_string() + "]",
      Value::PublicInput(_, index) => "public[".to_string() + &index.to_string() + "]",
      Value::RandomValue(x) => "rand[".to_string() + &x.to_string() + "]",
    }
  }

  pub fn binop(&self, r:&str, op:&str, a: &NodeIndex, b:&NodeIndex, counter: &mut i32) -> String {
    let va = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
    let vb = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
    let sa = self.ascairo(&va, a, counter);
    let sb = self.ascairo(&vb, b, counter);
    sa + &sb + 
      "  " + op + "_g(" + &va + ", " + &vb + ");\n" + 
      "  local " + r + " = [ap - 1];\n"
  }

  pub fn ascairo(&self, r:&str, w: &NodeIndex, counter: &mut i32) -> String {
    let op = &self.graph.node(w).op;
    match op {
      Operation::Value(x) => "  local ".to_string() + r + " = " + &self.showvalue(x) + ";\n",
      Operation::Add(a, b) => self.binop(r, "add", a, b, counter),
      Operation::Sub(a, b) => self.binop(r, "sub", a, b, counter),
      Operation::Mul(a, b) => self.binop(r, "mul", a, b, counter),
      Operation::Exp(a, j) => 
        {
          let va = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
          let sa = self.ascairo(&va, a, counter);
          let r = 
             sa + 
             "  pow_g(" + &va + ", " + &j.to_string() + ")"  + ";\n" +
             "  local " + r + " = [ap - 1];\n"
          ; 
          r 
        },
    }
  }

  /// Returns a string of Cairo code implementing Cairo0
  pub fn generate(&self) -> String {
    let mut counter : i32 = 0;
    // header
    let mut s = 
      "// Air name ".to_string() + &self.air_name + " " + &(self.segment_widths.len().to_string()) + " segments\n"
     ;
     s = s + "from starkware.cairo.common.alloc import alloc\n";
     s = s + "from starkware.cairo.common.memcpy import memcpy\n";
     s = s + "\n";
     s = s + "func exp (x:felt, t:felt) -> felt {\n  return (1); \n}\n";
     s = s + "func mod(x:felt,y:felt) -> felt {\n  return (1); \n}\n";
   
     s = s +
       "struct EvaluationFrame {\n" +
       "  current_len: felt,\n" +
       "  current: felt*,\n" +
       "  next_len: felt,\n" +
       "  next: felt*,\n" +
       "}\n"
     ;

     // Each segment
     for (i, w) in self.segment_widths.iter().enumerate() {
       s = s + "\n// SEGMENT " + &i.to_string() + " size " + &w.to_string() + "\n" + 
         "// ===============================================\n"
       ;
       s = s + 
         "func evaluate_transition_" + &i.to_string() + "{range_check_ptr} (\n" + 
         "  frame: EvaluationFrame,\n" + 
         "  t_evaluations: felt*,\n" + 
         "  periodic_row: felt*,\n" +                       // periodic value vector FIXME: DESIGN FAULT!
         { if i > 0 { "  rand: felt*,\n" } else { "" }} + 
         ") {\n" + 
         "  alloc_locals;\n" + 
         "  let cur = frame.current;\n" + 
         "  let nxt = frame.next;\n"
       ;
            
       // transition constraints
       s = s + "// TRANSITION CONSTRAINTS\n\n";
       let vc = &self.integrity_constraints[i];
       //s = s + "\n  // Integrity   constraints (" + &(vc.len().to_string()) + ")\n  // ----------------\n";
       for (i, w) in vc.iter().enumerate() {
         //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
        let r = "v".to_string() + &counter.to_string(); counter = counter + 1;
        let eval = &self.ascairo(&r, &w.index, &mut counter);
        s = s + &eval + "  assert t_evaluations[" + &i.to_string() + "] = " + &r + ";\n\n";
       }

       s = s + "\n  return ();\n";
       s = s + "}\n";

       s = s + 
         "func evaluate_boundary_" + &i.to_string() + "{range_check_ptr} (\n" + 
         "  frame: EvaluationFrame,\n" + 
         "  b_evaluations: felt*,\n" + 
         "  public: felt*,\n" + 
         { if i > 0 { "  rand: felt*,\n" } else { "" }} + 
         ") {\n" + 
         "  alloc_locals;\n" + 
         "  let cur = frame.current;\n" + 
         "  let nxt = frame.next;\n" 
       ;
            

       // boundary constraints
       s = s + "// BOUNDARY CONSTRAINTS\n\n";
       let bc = &self.boundary_constraints[i];
       //s = s + "\n  // Integrity   constraints (" + &(vc.len().to_string()) + ")\n  // ----------------\n";
       for (i, w) in bc.iter().enumerate() {
         //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
        let r = "v".to_string() + &counter.to_string(); counter = counter + 1;
        let eval = &self.ascairo(&r, &w.index, &mut counter);
        s = s + &eval + "  assert b_evaluations[" + &i.to_string() + "] = " + &r + ";\n\n";


       } // constraints

       s = s + "\n  return ();\n";
       s = s + "}\n";

     } // segments

     return glib.to_string() + &s + "\n";
  } // generate
} // CodeGenerator
