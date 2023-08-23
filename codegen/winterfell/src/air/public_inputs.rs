use super::{AirIR, Scope};

/// Updates the provided scope with a public inputs.
pub(super) fn add_public_inputs_struct(scope: &mut Scope, ir: &AirIR) {
    let name = "PublicInputs";
    // define the PublicInputs struct.
    let pub_inputs_struct = scope.new_struct(name).vis("pub");

    for (pub_input, _pub_input_size) in ir.public_inputs() {
        pub_inputs_struct.field(pub_input, format!("Vec::<Felt>"));
    }

    // add the public inputs implementation block
    let base_impl = scope.new_impl(name);

    let pub_inputs_values: Vec<String> = ir
        .public_inputs()
        .iter()
        .map(|input| input.0.clone())
        .collect();

    // add a constructor for public inputs
    let new_fn = base_impl
        .new_fn("new")
        .vis("pub")
        .ret("Self")
        .line(format!("Self {{ {} }}", pub_inputs_values.join(", ")));
    for (pub_input, _pub_input_size) in ir.public_inputs() {
        new_fn.arg(pub_input, format!("Vec::<Felt>"));
    }

    add_serializable_impl(scope, &pub_inputs_values);
    add_to_elements_impl(scope, &pub_inputs_values)
}

/// Adds Serialization implementation for PublicInputs to the scope
fn add_serializable_impl(scope: &mut Scope, pub_input_values: &Vec<String>) {
    let serializable_impl = scope.new_impl("PublicInputs").impl_trait("Serializable");
    let write_into_fn = serializable_impl
        .new_fn("write_into")
        .generic("W: ByteWriter")
        .arg_ref_self()
        .arg("target", "&mut W");
    for pub_input_value in pub_input_values {
        write_into_fn.line(format!("target.write(self.{pub_input_value}.as_slice());"));
    }
}

fn add_to_elements_impl(scope: &mut Scope, pub_input_values: &Vec<String>) {
    let to_elements_impl = scope
        .new_impl("PublicInputs")
        .impl_trait("ToElements<Felt>");

    let to_elements_fn = to_elements_impl
        .new_fn("to_elements")
        .ret("Vec<Felt>")
        .arg_ref_self();
    let to_elements_fn = to_elements_fn.line("let mut result = Vec::new();");
    for pub_input_value in pub_input_values {
        to_elements_fn.line(format!(
            "result.extend_from_slice(&self.{pub_input_value});"
        ));
    }
    to_elements_fn.line("result");
}
