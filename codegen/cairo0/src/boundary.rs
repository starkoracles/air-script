use ir::constraints::AlgebraicGraph;
use ir::constraints::ConstraintRoot;
use ir::constraints::ConstraintDomain;
use ir::PublicInput;


use super::showvalue;
use showvalue::str;

// Evaluate boundary constraints for one segment
pub fn evaluate_boundaries(
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
/*
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
*/
    if segment == 0 { // MAIN
      s = s + 
      "func evaluate_boundary_" + &segment.to_string() + "{range_check_ptr} (\n" + 
      "  frame_0: EvaluationFrame,\n" + 
      "  b_evaluations: felt*,\n" + 
    &{ 
      let mut s = "".to_string();
      for (name,_size) in public_inputs.iter() {
        s = s + "  " + name + ": felt*,\n";
      }
      s
    }
    +
      ") {\n" + 
      "  alloc_locals;\n" + 
      "  let first_0 = frame_0.current;\n"+ 
      "  let last_0 = frame_0.next;\n"
      ;
    } else { // AUX
      s = s + 
      "func evaluate_boundary_" + &segment.to_string() + "{range_check_ptr} (\n" + 
        "  frame_0: EvaluationFrame,\n" + 
        "  frame_1: EvaluationFrame,\n" + 
        "  b_evaluations: felt*,\n" + 
    &{ 
      let mut s = "".to_string();
      for (name,_size) in public_inputs.iter() {
        s = s + "  " + name + ": felt*,\n";
      }
      s
    }
    +
        "  rand: felt*,\n" + 
        ") {\n" + 
        "  alloc_locals;\n" + 
        "  let first_0 = frame_0.current;\n" + 
        "  let last_0 = frame_0.next;\n" + 
        "  let first_1 = frame_1.current;\n" + 
        "  let last_1 = frame_1.next;\n"  
    };
        
  // boundary constraints
  s = s + "// BOUNDARY CONSTRAINTS\n\n";
  let mut boundary_degrees: Vec<usize> = Vec::new();
  let mut boundary_domain: Vec<ConstraintDomain> = Vec::new();

  let mut print_constraint = |i:usize, w: &ConstraintRoot| -> String {
    //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
    let domain = &w.domain;
    let mut s = "  // ".to_string() + &str(&graph,&w.index,domain) + "\n";
    let r = "v".to_string() + &counter.to_string(); counter = counter + 1;
    let eval = &showvalue::ascairo(&graph,&r, &w.index, domain, &mut counter);
    s = s.clone() + &eval + "  assert b_evaluations[" + &i.to_string() + "] = " + &r + ";\n";
    let degree = &graph.degree(&w.index).base();
    s = s.clone() + "  // deg = " + &degree.to_string() + ", Domain: " + &w.domain.to_string() + "\n\n";
    boundary_degrees.push(*degree);
    boundary_domain.push(w.domain); 
    s
  };

  for (index, bcon) in boundary_constraints.iter().enumerate() {
    s = s + &print_constraint(index,&bcon);
  }

  let mut boundary_maxdeg : usize = 0;
  for w in boundary_degrees.iter() {
    boundary_maxdeg = boundary_maxdeg.max(*w);
  }

  s = s + "\n  return ();\n";
  s = s + "}\n";

  return (s, boundary_degrees,boundary_maxdeg, boundary_domain);
}
