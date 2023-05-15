use ir::constraints::AlgebraicGraph;
use ir::constraints::ConstraintRoot;
use ir::constraints::ConstraintDomain;
use ir::Value;
use ir::constraints::Operation;
use ir::PublicInput;


use super::showvalue;
use showvalue::str;

// Evaluate boundary constraints for one segment
pub fn evaluate_boundaries(
  trace_width: usize,
  graph: &AlgebraicGraph, 
  public_inputs: &Vec<PublicInput>,
  segment: usize, 
  boundary_constraints: &Vec<ConstraintRoot>,
) -> (
  String, 
  Vec<usize>,  // constraint degrees
  usize,       // maximum degree
  Vec<ConstraintDomain>)
{
  let mut counter : i32 = 0;
  let mut s = "".to_string();

  s = s + 
    "func evaluate_boundary_" + &segment.to_string() + "{range_check_ptr} (\n" + 
    "  frame: EvaluationFrame,\n" + 
    "  b_evaluations: felt*,\n" + 
    &{ 
      let mut s = "".to_string();
      for (name,_size) in public_inputs.iter() {
        s = s + "  " + name + ": felt*,\n";
      }
      s
    }
    +
    { if segment > 0 { "  rand: felt*,\n" } else { "" }} + 
    ") {\n" + 
    "  alloc_locals;\n" + 
    "  let cur = frame.current;\n"
  ;
        
  // boundary constraints
  s = s + "// BOUNDARY CONSTRAINTS\n\n";
  let mut boundary_degrees: Vec<usize> = Vec::new();
  let mut boundary_domain: Vec<ConstraintDomain> = Vec::new();

  let bc = &boundary_constraints;

  let mut print_constraint = |i:usize, w: &ConstraintRoot| -> String {
    //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
    let mut s = "  // ".to_string() + &str(&graph,&w.index) + "\n";
    let r = "v".to_string() + &counter.to_string(); counter = counter + 1;
    let eval = &showvalue::ascairo(&graph,&r, &w.index, &mut counter);
    s = s.clone() + &eval + "  assert b_evaluations[" + &i.to_string() + "] = " + &r + ";\n";
    let degree = &graph.degree(&w.index).base();
    s = s.clone() + "  // deg = " + &degree.to_string() + ", Domain: " + &w.domain.to_string() + "\n\n";
    boundary_degrees.push(*degree);
    boundary_domain.push(w.domain); 
    s
  };

  let find_column = |column: usize, domain: ConstraintDomain, w: &ConstraintRoot| -> bool {
    if w.domain != domain { false } else {
      match graph.node(w.node_index()).op() 
      {
        Operation::Sub(nidx,_) => 
        {
          match graph.node(nidx).op() 
          {
            Operation::Value(te) => 
              match te {
                Value::TraceElement(ita) => ita.col_idx() == column,
                _ => false // other than a trace element
              },
            _ => false // other than a value
          }
        }, // Sub
        _ => false // some other thing than Sub
      } // end of top level match
    } // domain
  };
 
  s = s + "// First Row\n\n";
  for column in 0..trace_width {
    let mut done = false;
    for w in bc.iter() {
      if !done && find_column (column, ConstraintDomain::FirstRow, w) {
        s = s + &print_constraint(column,w);
        done = true;
      }
    }
    if !done {
      s = s + "  assert b_evaluations["+&column.to_string()+"] = 0;\n\n";
    }
  }; 

  s = s + "// Last Row\n\n";
  for column in 0..trace_width {
    let mut done = false;
    for w in bc.iter() {
      if find_column (column, ConstraintDomain::LastRow, w) {
        s = s + &print_constraint(column+trace_width,w);
        done = true;
      }
    }
    if !done {
      s = s + "  assert b_evaluations["+&(column + trace_width).to_string()+"] = 0;\n\n";
    }
  }; 

  let mut boundary_maxdeg : usize = 0;
  for w in boundary_degrees.iter() {
    boundary_maxdeg = boundary_maxdeg.max(*w);
  }

  s = s + "\n  return ();\n";
  s = s + "}\n";

  return (s, boundary_degrees,boundary_maxdeg, boundary_domain);
}
