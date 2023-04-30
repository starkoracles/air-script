use ir::Value;
use ir::constraints::AlgebraicGraph;
use ir::NodeIndex;
use ir::constraints::Operation;

pub fn showvalue(x: &Value) -> String {
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
    Value::PeriodicColumn(index, _length) => "periodic_row[".to_string() + &index.to_string() + "]",
    Value::PublicInput(_, index) => "public[".to_string() + &index.to_string() + "]",
    Value::RandomValue(x) => "rand[".to_string() + &x.to_string() + "]",
  }
}

pub fn binop(graph:&AlgebraicGraph, r:&str, op:&str, a: &NodeIndex, b:&NodeIndex, counter: &mut i32) -> String {
  let va = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
  let vb = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
  let sa = ascairo(graph, &va, a, counter);
  let sb = ascairo(graph, &vb, b, counter);
  sa + &sb + &format!("  let {} = {}_g({}, {});\n", r, op, va, vb)
}


pub fn ascairo(graph: &AlgebraicGraph, r:&str, w: &NodeIndex, counter: &mut i32) -> String {
  let op = &graph.node(w).op;
  match op {
    Operation::Value(x) => "  let ".to_string() + r + " = " + &showvalue(x) + ";\n",
    Operation::Add(a, b) => binop(graph, r, "add", a, b, counter),
    Operation::Sub(a, b) => binop(graph, r, "sub", a, b, counter),
    Operation::Mul(a, b) => binop(graph, r, "mul", a, b, counter),
    Operation::Exp(a, j) => 
      {
        let va = "v".to_string() +&counter.to_string(); *counter = *counter + 1;
        let sa = ascairo(graph, &va, a, counter);
           sa + &format!("  let {} = pow_g({}, {});\n", r, va, j)
      },
  }
}

  pub fn str(graph:&AlgebraicGraph,  w: &NodeIndex ) -> String {
    let op = &graph.node(w).op;
    match op {
      Operation::Value(x) =>  showvalue(x),
      Operation::Add(a, b) => "(".to_string()  + &str(graph,a) + " + " + &str(graph,b) + ")",
      Operation::Sub(a, b) => "(".to_string()  + &str(graph,a) + " - " + &str(graph,b) + ")", 
      Operation::Mul(a, b) => "(".to_string()  + &str(graph,a) + " * " + &str(graph,b) + ")",
      Operation::Exp(a, j) => "(".to_string()  + &str(graph,a) + " ^ " + &j.to_string() + ")", 
    }
  }


