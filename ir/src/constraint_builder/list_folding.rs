use super::{
    list_comprehension::unfold_lc, ConstantType, Expression, IndexedTraceAccess,
    ListFoldingValueType, SemanticError, SymbolTable, SymbolType, VariableType, CURRENT_ROW,
};

// LIST FOLDING
// ================================================================================================

/// Builds a list of expressions from a list folding value. The list folding value can be either a
/// vector, a list comprehension, or an identifier that refers to a vector.
///
/// # Errors
/// Returns an error if:
/// - the list folding value is an identifier that does not exist in the symbol table
/// - the list folding value is an identifier that does not refer to a vector
pub fn build_list_from_list_folding_value(
    lf_value_type: &ListFoldingValueType,
    symbol_table: &SymbolTable,
) -> Result<Vec<Expression>, SemanticError> {
    match lf_value_type {
        ListFoldingValueType::Identifier(ident) => {
            let symbol = symbol_table.get_symbol(ident.name())?;
            match symbol.symbol_type() {
                SymbolType::Constant(ConstantType::Vector(list)) => {
                    Ok(list.iter().map(|value| Expression::Const(*value)).collect())
                }
                SymbolType::Variable(var_type) => {
                    if let VariableType::Vector(list) = var_type {
                        Ok(list.clone())
                    } else {
                        Err(SemanticError::invalid_list_folding(
                            lf_value_type,
                            symbol.symbol_type(),
                        ))
                    }
                }
                SymbolType::TraceColumns(columns) => {
                    if columns.size() > 1 {
                        let trace_segment = columns.trace_segment();
                        Ok((0..columns.size())
                            .map(|i| {
                                Expression::IndexedTraceAccess(IndexedTraceAccess::new(
                                    trace_segment,
                                    columns.offset() + i,
                                    CURRENT_ROW,
                                ))
                            })
                            .collect())
                    } else {
                        Err(SemanticError::invalid_list_folding(
                            lf_value_type,
                            symbol.symbol_type(),
                        ))
                    }
                }
                _ => Err(SemanticError::invalid_list_folding(
                    lf_value_type,
                    symbol.symbol_type(),
                )),
            }
        }
        ListFoldingValueType::Vector(vector) => Ok(vector.clone()),
        ListFoldingValueType::ListComprehension(lc) => Ok(unfold_lc(lc, symbol_table)?),
    }
}