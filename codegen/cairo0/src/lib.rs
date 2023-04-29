use ir::AirIR;
use ir::ConstantBinding;
use ir::PeriodicColumn;
use ir::PublicInput;

use ir::constraints::AlgebraicGraph;
use ir::constraints::ConstraintRoot;
use ir::constraints::Operation;
use ir::constraints::ConstraintDomain;
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
          "nxt[".to_string() + &colidx + "]"
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
    sa + &sb + &format!("  let {} = {}_g({}, {});\n", r, op, va, vb)
  }

  pub fn ascairo(&self, r:&str, w: &NodeIndex, counter: &mut i32) -> String {
    let op = &self.graph.node(w).op;
    match op {
      Operation::Value(x) => "  let ".to_string() + r + " = " + &self.showvalue(x) + ";\n",
      Operation::Add(a, b) => self.binop(r, "add", a, b, counter),
      Operation::Sub(a, b) => self.binop(r, "sub", a, b, counter),
      Operation::Mul(a, b) => self.binop(r, "mul", a, b, counter),
      Operation::Exp(a, j) => 
        {
          let va = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
          let sa = self.ascairo(&va, a, counter);
             sa + &format!("  let {} = pow_g({}, {});\n", r, va, j)
        },
    }
  }

  pub fn str(&self,  w: &NodeIndex ) -> String {
    let op = &self.graph.node(w).op;
    match op {
      Operation::Value(x) =>  self.showvalue(x),
      Operation::Add(a, b) => "(".to_string()  + &self.str(a) + " + " + &self.str(b) + ")",
      Operation::Sub(a, b) => "(".to_string()  + &self.str(a) + " - " + &self.str(b) + ")", 
      Operation::Mul(a, b) => "(".to_string()  + &self.str(a) + " * " + &self.str(b) + ")",
      Operation::Exp(a, j) => "(".to_string()  + &self.str(a) + " ^ " + &j.to_string() + ")", 
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
     s = s + "from math_goldilocks import add_g, sub_g, mul_g, pow_g, div_g\n";
     s = s + "\n";


     // count total number of transition and boundary constrainst
     //{
     //  let mut nt : usize = 0;   
     //  let mut nb :  usize = 0;   
     //  let ns = self.segment_widths.len();
     //  for (i, w) in self.segment_widths.iter().enumerate() {
     //     nt = nt + &self.integrity_constraints[i].len();
     //     nt = nt + &self.boundary_constraints[i].len();
     //  }
     //  
     //   s = s + &fmtairinst(
     //     &self.air_name, 
     //     self.segment_widths[0].into(), // main_segment_width
     //     99999999, // aux_trace_width
     //     ns - 1, // num_aux_segments
     //     nt, // num_transition_constraints
     //     nb  // num_boundary_counstraints
     //   );
     // };

     s = s +
       "struct EvaluationFrame {\n" +
       "  current_len: felt,\n" +
       "  current: felt*,\n" +
       "  next_len: felt,\n" +
       "  next: felt*,\n" +
       "}\n"
     ;

     // Each segment
     for (segment, w) in self.segment_widths.iter().enumerate() {
       s = s + "\n// SEGMENT " + &segment.to_string() + " size " + &w.to_string() + "\n" + 
         "// ===============================================\n"
       ;
       s = s + 
         "func evaluate_transition_" + &segment.to_string() + "{range_check_ptr} (\n" + 
         "  frame: EvaluationFrame,\n" + 
         "  t_evaluations: felt*,\n" + 
         "  periodic_row: felt*,\n" +                       // periodic value vector FIXME: DESIGN FAULT!
         { if segment > 0 { "  rand: felt*,\n" } else { "" }} + 
         ") {\n" + 
         "  alloc_locals;\n" + 
         "  let cur = frame.current;\n" + 
         "  let nxt = frame.next;\n"
       ;
       let mut transition_degrees: Vec<usize> = Vec::new();
     
       // transition constraints
       s = s + "// TRANSITION CONSTRAINTS\n\n";
       let vc = &self.integrity_constraints[segment];
       //s = s + "\n  // Integrity   constraints (" + &(vc.len().to_string()) + ")\n  // ----------------\n";
       for (i, w) in vc.iter().enumerate() {
         //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
        s = s + "  // " + &self.str(&w.index) + "\n";
        let r = "v".to_string() + &counter.to_string(); counter = counter + 1;
        let eval = &self.ascairo(&r, &w.index, &mut counter);
        s = s + &eval + "  assert t_evaluations[" + &i.to_string() + "] = " + &r + ";\n";
        let degree = &self.graph.degree(&w.index).base();
        s = s + "  // deg = " + &degree.to_string() + "\n\n";
        transition_degrees.push(*degree);
       }
       let mut transition_maxdeg : usize = 0;
       for (i, w) in transition_degrees.iter().enumerate() {
         transition_maxdeg = transition_maxdeg.max(*w);
       }

       s = s + "\n  return ();\n";
       s = s + "}\n\n";

       s = s + 
         "func evaluate_boundary_" + &segment.to_string() + "{range_check_ptr} (\n" + 
         "  frame: EvaluationFrame,\n" + 
         "  b_evaluations: felt*,\n" + 
         "  public: felt*,\n" + 
         { if segment > 0 { "  rand: felt*,\n" } else { "" }} + 
         ") {\n" + 
         "  alloc_locals;\n" + 
         "  let cur = frame.current;\n"
       ;
            

       // boundary constraints
       s = s + "// BOUNDARY CONSTRAINTS\n\n";
       let mut boundary_degrees: Vec<usize> = Vec::new();
       let mut boundary_domain: Vec<ConstraintDomain> = Vec::new();

       let bc = &self.boundary_constraints[segment];
       //s = s + "\n  // Integrity   constraints (" + &(vc.len().to_string()) + ")\n  // ----------------\n";
       for (i, w) in bc.iter().enumerate() {
         //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
        s = s + "  // " + &self.str(&w.index) + "\n";
        let r = "v".to_string() + &counter.to_string(); counter = counter + 1;
        let eval = &self.ascairo(&r, &w.index, &mut counter);
        s = s + &eval + "  assert b_evaluations[" + &i.to_string() + "] = " + &r + ";\n";
        let degree = &self.graph.degree(&w.index).base();
        s = s + "  // deg = " + &degree.to_string() + ", Domain: " + &w.domain.to_string() + "\n\n";
        boundary_degrees.push(*degree);
        boundary_domain.push(w.domain) 

       } // constraints

       let mut boundary_maxdeg : usize = 0;
       for (i, w) in boundary_degrees.iter().enumerate() {
         boundary_maxdeg = boundary_maxdeg.max(*w);
       }


       s = s + "\n  return ();\n";
       s = s + "}\n";

       s = s + "// MERGE EVALUATIONS\n";
       s = s + "func merge_transitions_" + &segment.to_string() + "{range_check_ptr}(\n";
       s = s + "  trace_length: felt,\n";
       s = s + "  target_degree: felt,\n";
       s = s + "  coeffs_transition_a: felt*,\n";
       s = s + "  coeffs_transition_b: felt*, \n";
       s = s + "  t_evaluations: felt*, \n";
       s = s + "  x: felt, \n";
       s = s + "  z: felt, \n";
       s = s + ") -> felt {\n";
       s = s + "  alloc_locals;\n";
       s = s + "  local sum_0 = 0;\n";
       let mut counter = 0;
       for deg in 0 .. (transition_maxdeg+1) {
         let mut ntrans = 0;
         for (_tr, trdeg) in transition_degrees.iter().enumerate() {
           if deg == *trdeg { ntrans = ntrans + 1; }
         }

         if ntrans > 0 {
           s = s + "\n  // Merge degree "+ &deg.to_string() +"\n";
           s = s + "  let evaluation_degree = "+&deg.to_string() +" * (trace_length - 1);\n";
           s = s + "  let degree_adjustment = target_degree - evaluation_degree;\n";
           s = s + "  let xp = pow_g(x, degree_adjustment);\n";
           for (tr, trdeg) in transition_degrees.iter().enumerate() {
             if deg == *trdeg {
               let trno = &tr.to_string();
               s = s + "\n  // Include transition " + &trno + "\n";
               s = s + "  let v1 = mul_g(coeffs_transition_b["+&trno+"],  xp);\n";
               s = s + "  let v2 = add_g(coeffs_transition_a["+ &trno +"], v1);\n";
               s = s + "  let v3 = mul_g(v2, t_evaluations["+&trno+"]);\n";
               s = s + "  local sum_"+&(counter+1).to_string() +" = add_g(sum_"+&counter.to_string()+",v3);\n";
               counter = counter + 1;
             }
           }
         }
       }

       s = s + "\n  return div_g(sum_"+&counter.to_string()+",z);\n";
       s = s + "}\n";

// WARNING: THIS CODE ONLY HANDLES BOUNDARIES ON MAIN SEGMENT
// AUX SEGMENT REQUIRES SLIGHTLY DIFFERENT CALCULATION
// USES TRACE LENGTH INSTEAD OF PUBLIC INPUT STEPS
       s = s + "func merge_boundary_" + &segment.to_string() + "{range_check_ptr}(\n";
       s = s + "  trace_length: felt,\n";
       s = s + "  target_degree: felt,\n";
       s = s + "  blowup_factor: felt,\n";
       s = s + "  coeffs_boundary_a: felt*,\n";
       s = s + "  coeffs_boundary_b: felt*, \n";
       s = s + "  b_evaluations: felt*, \n";
       s = s + "  trace_domain_generator: felt, \n";
       s = s + "  npub_steps: felt, \n";
       s = s + "  z: felt, \n";
       s = s + ") -> felt {\n";
       s = s + "  alloc_locals;\n";

       s = s + "  let composition_degree = trace_length * blowup_factor - 1;\n";
       s = s + "  let trace_poly_degree = trace_length  - 1;\n";
       s = s + "  let divisor_degree = 1;\n";
       s = s + "  let target_degree =  composition_degree + divisor_degree;\n";
       s = s + "  let first_z = z - 1;\n\n";
       s = s + "  let g = trace_domain_generator;\n\n";
       s = s + "  let gn = pow_g(g,npub_steps - 1);\n\n";
       s = s + "  let last_z = z - gn;\n";
       s = s + "\n";
       s = s + "  local first_sum_0 = 0;\n";
       s = s + "  local last_sum_0 = 0;\n";
       let mut first_counter = 0;
       let mut last_counter = 0;
       for deg in 0 ..(boundary_maxdeg+1) {
         let mut ntrans = 0;
         for (__tr, trdeg) in boundary_degrees.iter().enumerate() {
           if deg == *trdeg { ntrans = ntrans + 1; }
         }

         if ntrans > 0 {
           s = s + "\n  // Merge degree "+ &deg.to_string() +"\n";
           s = s + "  let evaluation_degree = "+&deg.to_string() +" * (trace_length - 1);\n";
           s = s + "  let degree_adjustment = target_degree - evaluation_degree;\n";
           s = s + "  let xp = pow_g(z, degree_adjustment);\n";
           for (tr, trdeg) in boundary_degrees.iter().enumerate() {
             if deg == *trdeg {
               let trno = &tr.to_string();
               s = s + "\n  // Include boundary " + &trno + "\n";
               s = s + "  let v1 = mul_g(coeffs_boundary_b["+&trno+"],  xp);\n";
               s = s + "  let v2 = add_g(coeffs_boundary_a["+ &trno +"], v1);\n";
               s = s + "  let v3 = mul_g(v2, b_evaluations["+&trno+"]);\n";
               match boundary_domain[tr] {
                 ConstraintDomain::FirstRow => {
                    s = s + "  local first_sum_"+&(first_counter+1).to_string() +" = add_g(first_sum_"+&first_counter.to_string()+",v3);\n";
                    first_counter = first_counter + 1;
                 },
                 ConstraintDomain::LastRow => {
                    s = s + "  local last_sum_"+&(last_counter+1).to_string() +" = add_g(last_sum_"+&last_counter.to_string()+",v3);\n";
                    last_counter = last_counter + 1;
                 },
                 _ => { panic!("Bad Boundary Constraint Domain"); }
               }
             }
           }
         }
       }

       s = s + "\n";
       s = s + "  let first = div_g(first_sum_"+&first_counter.to_string() + ",first_z);\n";
       s = s + "  let last = div_g(last_sum_"+&last_counter.to_string() + ",last_z);\n";
       s = s + "  return add_g(first,last);\n";
       s = s + "}\n";


     } // segments

     s = s + "\n// PUT CONSTRAINT EVALUATION FUNCTION HERE\n";

     return s + "\n";
  } // generate
} // CodeGenerator
