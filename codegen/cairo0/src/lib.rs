use ir::AirIR;
use ir::Constants;
use ir::PublicInputs;
use ir::PeriodicColumns;
use ir::Constraints;


// GENERATE verifier for proof as Cairo v0.4 
// ================================================================================================

pub struct CodeGenerator {
  air_name: String,
  segment_widths: Vec<u16>,
  constants: Constants,
  public_inputs: PublicInputs,
  periodic_columns: PeriodicColumns,
  constraints: Constraints,
}

impl CodeGenerator {
    // --- CONSTRUCTOR ----------------------------------------------------------------------------

    /// Builds a new Rust scope that represents a Cairo0 Air trait implementation for the
    /// provided AirIR.
    pub fn new(_ir: &AirIR) -> CodeGenerator {
        Self { 
          air_name: _ir.air_name.clone(),
          segment_widths: _ir.segment_widths.clone(),
          constants: _ir.constants.clone(),  //Vec<air_ir::Constant> 
          public_inputs: _ir.public_inputs.clone(), //Vec<(String, usize)> 
          periodic_columns: _ir.periodic_columns.clone(), //Vec<Vec<u64>>`
          constraints: _ir.constraints.clone(), //Constraints
        }
    }

    /// Returns a string of Cairo code implementing Cairo0
    pub fn generate(&self) -> String {
      let s1 = "// Hello, Cairo0!\n".to_string() + 
        "// Air name " + &self.air_name + " " + &(self.segment_widths.len().to_string())+" segments\n"
      ;
      let mut s2 = "// ".to_string();
      for (i,w) in self.segment_widths.iter().enumerate() {
        s2 = s2  + "Segment " + &i.to_string() + "  size " + &w.to_string() + "\n// ";
      }
      return s1 + &s2 + "\n";
    }
}
