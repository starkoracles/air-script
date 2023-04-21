use ir::AirIR;
use ir::ConstantBinding;
use ir::PeriodicColumn;
use ir::PublicInput;

use ir::constraints::AlgebraicGraph;
use ir::constraints::ConstraintRoot;
use ir::constraints::Operation;
use ir::NodeIndex;
use ir::Value;

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
      Value::PeriodicColumn(index, length) => "periodic[".to_string() + &index.to_string() + " + mod(row, " + &length.to_string() + ")" + "]",
      Value::PublicInput(_, index) => "punlic[".to_string() + "]",
      Value::RandomValue(x) => "rand[".to_string() + &x.to_string() + "]",
    }
  }

  pub fn ascairo(&self, w: &NodeIndex) -> String {
    let op = &self.graph.node(w).op;
    match op {
      Operation::Value(x) => self.showvalue(x),
      Operation::Add(a, b) => "(".to_string() + &self.ascairo(a) + " + " + &self.ascairo(b) + ")",
      Operation::Sub(a, b) => "(".to_string() + &self.ascairo(a) + " - " + &self.ascairo(b) + ")",
      Operation::Mul(a, b) => "(".to_string() + &self.ascairo(a) + " * " + &self.ascairo(b) + ")",
      Operation::Exp(a, j) => "exp(".to_string() + &self.ascairo(a) + ", " + &j.to_string() + ")",
    }
  }

  /// Returns a string of Cairo code implementing Cairo0
  pub fn generate(&self) -> String {
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
       "  row: felt,\n" + 
       "}\n"
     ;

     // Each segment
     for (i, w) in self.segment_widths.iter().enumerate() {
       s = s + "\n// SEGMENT " + &i.to_string() + " size " + &w.to_string() + "\n" + 
         "// ===============================================\n"
       ;
       s = s + 
         "func evaluate_transition_" + &i.to_string() + " (\n" + 
         "  frame: EvaluationFrame,\n" + 
         "  t_evaluations: felt*,\n" + 
         "  periodic: felt*,\n" +                       // periodic value vector FIXME: DESIGN FAULT!
         { if i > 0 { "  rand: felt*,\n" } else { "" }} + 
         ") {\n" + 
         "  alloc_locals;\n" + 
         "  let cur = frame.current;\n" + 
         "  let nxt = frame.next;\n" + 
         "  let row = frame.row;\n"
       ;
            
       // just validity constraints
       let vc = &self.integrity_constraints[i];
       //s = s + "\n  // Integrity   constraints (" + &(vc.len().to_string()) + ")\n  // ----------------\n";
       for (i, w) in vc.iter().enumerate() {
         //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
         s = s + "  assert t_evaluations[" + &i.to_string() + "] = " + &self.ascairo(&w.index) + ";\n";
       } // constraints

       s = s + "\n  return ();\n";
       s = s + "}\n";

     } // segments

     return s + "\n";
  } // generate
} // CodeGenerator
