use ir::constraints::AlgebraicGraph;
use ir::constraints::ConstraintRoot;
use ir::constraints::ConstraintDomain;

use super::showvalue;
use showvalue::str;

// Evaluate boundary constraints for one segment
pub fn evaluate_boundaries(
  graph: &AlgebraicGraph, 
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

  let bc = &boundary_constraints;
  //s = s + "\n  // Integrity   constraints (" + &(vc.len().to_string()) + ")\n  // ----------------\n";
  for (i, w) in bc.iter().enumerate() {
    //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
    s = s + "  // " + &str(&graph,&w.index) + "\n";
    let r = "v".to_string() + &counter.to_string(); counter = counter + 1;
    let eval = &showvalue::ascairo(&graph,&r, &w.index, &mut counter);
    s = s + &eval + "  assert b_evaluations[" + &i.to_string() + "] = " + &r + ";\n";
    let degree = &graph.degree(&w.index).base();
    s = s + "  // deg = " + &degree.to_string() + ", Domain: " + &w.domain.to_string() + "\n\n";
    boundary_degrees.push(*degree);
    boundary_domain.push(w.domain) 

  } // constraints

  let mut boundary_maxdeg : usize = 0;
  for w in boundary_degrees.iter() {
    boundary_maxdeg = boundary_maxdeg.max(*w);
  }

  s = s + "\n  return ();\n";
  s = s + "}\n";

  return (s, boundary_degrees,boundary_maxdeg, boundary_domain);
}
