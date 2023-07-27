use ir::Value;
use ir::constraints::AlgebraicGraph;
use ir::NodeIndex;
use ir::constraints::Operation;
use ir::constraints::ConstraintDomain;

/// Cairo display of AlgebraicGraph::Value
pub fn showvalue(domain: &ConstraintDomain, x: &Value) -> String {
  match x {
    Value::BoundConstant(_) => "BoundConstant".to_string(),
    Value::InlineConstant(v) => v.to_string(),
    Value::TraceElement(ita) => {
      let trace_segment = &ita.trace_segment().to_string();
      let offset = &ita.row_offset();
      let colidx = &ita.col_idx().to_string();
      match domain {
        ConstraintDomain::FirstRow =>
          "first_".to_string() +trace_segment+"["+ &colidx + "]",
        ConstraintDomain::LastRow =>
          "last_".to_string() +trace_segment+"["+ &colidx + "]",
        _ =>
        if *offset == 0 {
          "cur_".to_string() +trace_segment+"["+ &colidx + "]"
        } else {
          "nxt_".to_string()+trace_segment+"[" + &colidx + "]"
        }
      }
    }
    Value::PeriodicColumn(index, _length) => "periodic_row[".to_string() + &index.to_string() + "]",
    Value::PublicInput(s, index) => s.to_string()+"[" + &index.to_string() + "]",
    Value::RandomValue(x) => "rand[".to_string() + &x.to_string() + "]",
  }
}

pub fn binop(graph:&AlgebraicGraph, r:&str, op:&str, a: &NodeIndex, b:&NodeIndex, domain: &ConstraintDomain, counter: &mut i32) -> String {
  let va = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
  let vb = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
  let sa = ascairo(graph, &va, a, domain, counter);
  let sb = ascairo(graph, &vb, b, domain, counter);
  sa + &sb + &format!("  let {} = {}_g({}, {});\n", r, op, va, vb)
}


/// Cairo evaluation of Air constraint from AlgenbraicGraph
pub fn ascairo(graph: &AlgebraicGraph, r:&str, w: &NodeIndex, domain: &ConstraintDomain, counter: &mut i32) -> String {
  let op = &graph.node(w).op;
  match op {
    Operation::Value(x) => "  let ".to_string() + r + " = " + &showvalue(domain, x) + ";\n",
    Operation::Add(a, b) => binop(graph, r, "add", a, b, domain, counter),
    Operation::Sub(a, b) => binop(graph, r, "sub", a, b, domain, counter),
    Operation::Mul(a, b) => binop(graph, r, "mul", a, b, domain, counter),
    Operation::Exp(a, j) => 
      {
        let va = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
        let sa = ascairo(graph, &va, a, domain, counter);
           sa + &format!("  let {} = pow_g({}, {});\n", r, va, j)
      },
  }
}

/// Human readable costraint display
pub fn str(graph:&AlgebraicGraph,  w: &NodeIndex, domain: &ConstraintDomain ) -> String {
  let op = &graph.node(w).op;
  match op {
    Operation::Value(x) =>  showvalue(domain, x),
    Operation::Add(a, b) => "(".to_string()  + &str(graph,a,domain) + " + " + &str(graph,b,domain) + ")",
    Operation::Sub(a, b) => "(".to_string()  + &str(graph,a,domain) + " - " + &str(graph,b,domain) + ")", 
    Operation::Mul(a, b) => "(".to_string()  + &str(graph,a,domain) + " * " + &str(graph,b,domain) + ")",
    Operation::Exp(a, j) => "(".to_string()  + &str(graph,a,domain) + " ^ " + &j.to_string() + ")", 
  }
}


