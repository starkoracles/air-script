use ir::AirIR;

// GENERATE RUST CODE FOR WINTERFELL AIR
// ================================================================================================

/// CodeGenerator is used to generate a Rust implementation of the Cairo0 STARK prover library's
/// Air trait. The generated Air expresses the constraints specified by the AirIR used to build the
/// CodeGenerator.
pub struct CodeGenerator {}

impl CodeGenerator {
    // --- CONSTRUCTOR ----------------------------------------------------------------------------

    /// Builds a new Rust scope that represents a Cairo0 Air trait implementation for the
    /// provided AirIR.
    pub fn new(_ir: &AirIR) -> Self {
        Self {}
    }

    /// Returns a string of Cairo code implementing Cairo0
    pub fn generate(&self) -> String {
        return "// Hello, Cairo0!".to_string();
    }
}
