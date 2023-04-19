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
            Value::PeriodicColumn(_, _) => "PeriodicColumn".to_string(),
            Value::PublicInput(_, _) => "PublicInput".to_string(),
            Value::RandomValue(_) => "RandomValue".to_string(),
        }
    }

    pub fn ascairo(&self, w: &NodeIndex) -> String {
        let op = &self.graph.node(w).op;
        match op {
            Operation::Value(x) => self.showvalue(x),
            Operation::Add(a, b) => {
                "(".to_string() + &self.ascairo(a) + " + " + &self.ascairo(b) + ")"
            }
            Operation::Sub(a, b) => {
                "(".to_string() + &self.ascairo(a) + " - " + &self.ascairo(b) + ")"
            }
            Operation::Mul(a, b) => {
                "(".to_string() + &self.ascairo(a) + " * " + &self.ascairo(b) + ")"
            }
            Operation::Exp(a, j) => {
                "exp(".to_string() + &self.ascairo(a) + ", " + &j.to_string() + ")"
            }
        }
    }

    pub fn showconvalue(&self, x: &Value) -> String {
        match x {
            Value::BoundConstant(_) => todo!(),
            Value::InlineConstant(v) => v.to_string(),
            Value::TraceElement(ita) => {
                "TraceElement(".to_string()
                    + &ita.col_idx().to_string()
                    + &{
                        let offset = &ita.row_offset();
                        if *offset == 0 {
                            "".to_string()
                        } else {
                            "+".to_string() + &offset.to_string()
                        }
                    }
                    + ")"
            }
            Value::PeriodicColumn(_, _) => "PeriodicColumn".to_string(),
            Value::PublicInput(_, _) => "PublicInput".to_string(),
            Value::RandomValue(_) => "RandomValue".to_string(),
        }
    }

    pub fn showcon(&self, w: &NodeIndex) -> String {
        let op = &self.graph.node(w).op;
        match op {
            Operation::Value(x) => self.showconvalue(x),
            Operation::Add(a, b) => {
                "Add(".to_string() + &self.showcon(a) + ", " + &self.showcon(b) + ")"
            }
            Operation::Sub(a, b) => {
                "Sub(".to_string() + &self.showcon(a) + ", " + &self.showcon(b) + ")"
            }
            Operation::Mul(a, b) => {
                "Mul(".to_string() + &self.showcon(a) + ", " + &self.showcon(b) + ")"
            }
            Operation::Exp(a, j) => {
                "Exp(".to_string() + &self.showcon(a) + ", " + &j.to_string() + ")"
            }
        }
    }

    /// Returns a string of Cairo code implementing Cairo0
    pub fn generate(&self) -> String {
        let mut s1 = "// Hello, Cairo0!\n".to_string()
            + "// Air name "
            + &self.air_name
            + " "
            + &(self.segment_widths.len().to_string())
            + " segments\n";
        s1 = s1 + "from starkware.cairo.common.alloc import alloc\n";
        s1 = s1 + "from starkware.cairo.common.memcpy import memcpy\n";
        s1 = s1
            + "struct EvaluationFrame {\n"
            + "  current_len: felt,\n"
            + "  current: felt*,\n"
            + "  next_len: felt,\n"
            + "  next: felt*,\n"
            + "}\n";
        let mut s2 = "// ".to_string();
        for (i, w) in self.segment_widths.iter().enumerate() {
            s2 = s2 + "Segment " + &i.to_string() + "  size " + &w.to_string() + "\n// ";
        }
        let mut s3 = "".to_string();
        for (i, w) in self.segment_widths.iter().enumerate() {
            s3 = s3
                + "// SEGMENT "
                + &i.to_string()
                + " size "
                + &w.to_string()
                + "\n"
                + "// ===============================================\n";
            let bc = &self.boundary_constraints[i];
            s3 = s3
                + "\n  // Boundary   constraints ("
                + &(bc.len().to_string())
                + ") \n  // ----------------\n";
            for (i, w) in bc.iter().enumerate() {
                s3 = s3
                    + "    // #"
                    + &i.to_string()
                    + ": root node "
                    + &w.index.0.to_string()
                    + " Domain: "
                    + &w.domain.to_string()
                    + "\n";
                let opdisp = &self.showcon(&w.index);
                s3 = s3 + "    //    " + opdisp + "\n";
            }

            let vc = &self.integrity_constraints[i];
            s3 = s3
                + "\n  // Integrity   constraints ("
                + &(vc.len().to_string())
                + ")\n  // ----------------\n";
            for (i, w) in vc.iter().enumerate() {
                s3 = s3
                    + "    // #"
                    + &i.to_string()
                    + ": root node "
                    + &w.index.0.to_string()
                    + " Domain: "
                    + &w.domain.to_string()
                    + "\n";
                let opdisp = &self.showcon(&w.index);
                s3 = s3 + "    //   " + opdisp + "\n";
                s3 = s3
                    + "    assert t_evalations["
                    + &i.to_string()
                    + "] = "
                    + &self.ascairo(&w.index)
                    + ";\n";
            }
        }

        return s1 + &s2 + &s3 + "\n";
    }
}
