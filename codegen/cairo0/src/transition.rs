use ir::constraints::AlgebraicGraph;
use ir::constraints::ConstraintRoot;

use super::showvalue;
use showvalue::str;

/// Evaluate transition constraints for one segment
pub fn evaluate_transitions(
  graph: &AlgebraicGraph, 
  segment: usize, 
  integrity_constraints: &Vec<ConstraintRoot>,
) -> (
  String, 
  Vec<usize>,  // constraint degrees
  usize        // maximum degree
)
{
  let mut counter : i32 = 0;
  let mut s = "".to_string();
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
  let vc = &integrity_constraints;
  //s = s + "\n  // Integrity   constraints (" + &(vc.len().to_string()) + ")\n  // ----------------\n";
  for (i, w) in vc.iter().enumerate() {
    //s = s + "    // #" + &i.to_string() + ": root node " + &w.index.0.to_string() + " Domain: " + &w.domain.to_string() + "\n";
    s = s + "  // " + &str(&graph,&w.index) + "\n";
    let r = "v".to_string() + &counter.to_string(); counter = counter + 1;
    let eval = &showvalue::ascairo(&graph,&r, &w.index, &mut counter);
    s = s + &eval + "  assert t_evaluations[" + &i.to_string() + "] = " + &r + ";\n";
    let degree = &graph.degree(&w.index).base();
    s = s + "  // deg = " + &degree.to_string() + "\n\n";
    transition_degrees.push(*degree);
  }
  let mut transition_maxdeg : usize = 0;
  for w in transition_degrees.iter() {
    transition_maxdeg = transition_maxdeg.max(*w);
  }

  s = s + "\n  return ();\n";
  s = s + "}\n\n";

  return (s, transition_degrees,transition_maxdeg);
}


