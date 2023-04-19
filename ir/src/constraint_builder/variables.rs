use super::{AccessType, Expression, SemanticError, SymbolAccess, VariableValueExpr};

/// Returns an expression representing a single element, based on the name and value of a variable
/// binding and the type of access into that variable which is being attempted.
///
/// # Errors
/// Returns an error if the access would result in a vector or matrix of expressions instead of a
/// single expression.
pub(crate) fn get_variable_expr(
    bound_value: &VariableValueExpr,
    symbol_access: SymbolAccess,
) -> Result<Expression, SemanticError> {
    let (ident, access_type, offset) = symbol_access.into_parts();

    // access the expression in the bound value that is specified by the symbol_access.
    let (inner_expr, inner_access_type) = match bound_value {
        // return the expression. the access type does not change.
        VariableValueExpr::Scalar(expr) => (expr, access_type),
        VariableValueExpr::Vector(expr_vector) => {
            // get the specified expression from the expression vector.
            let inner_expr = match &access_type {
                AccessType::Vector(idx) | AccessType::Matrix(idx, _) => {
                    if *idx < expr_vector.len() {
                        &expr_vector[*idx]
                    } else {
                        return Err(SemanticError::invalid_variable_access_type(
                            ident.name(),
                            &access_type,
                        ));
                    }
                }
                _ => {
                    return Err(SemanticError::invalid_variable_access_type(
                        ident.name(),
                        &access_type,
                    ));
                }
            };
            // reduce the dimension of the access by 1, since we indexed the bound value once.
            let inner_access_type = reduce_access_dim(ident.name(), access_type)?;
            (inner_expr, inner_access_type)
        }
        VariableValueExpr::Matrix(expr_matrix) => {
            // get the specified expression from the expression matrix.
            let inner_expr = match &access_type {
                AccessType::Matrix(row_idx, col_idx) => {
                    if *row_idx < expr_matrix.len() && *col_idx < expr_matrix[0].len() {
                        &expr_matrix[*row_idx][*col_idx]
                    } else {
                        return Err(SemanticError::invalid_variable_access_type(
                            ident.name(),
                            &access_type,
                        ));
                    }
                }
                _ => {
                    return Err(SemanticError::invalid_variable_access_type(
                        ident.name(),
                        &access_type,
                    ));
                }
            };
            // reduce the dimension of the access by 2, since we indexed the bound value twice.
            let inner_access_type = reduce_access_dim(ident.name(), access_type)?;
            let inner_access_type = reduce_access_dim(ident.name(), inner_access_type)?;
            (inner_expr, inner_access_type)
        }
        _ => {
            return Err(SemanticError::invalid_variable_access_type(
                ident.name(),
                &access_type,
            ));
        }
    };

    // access the inner expression with the specified access type to get the expression
    access_inner_expr(ident.name(), inner_access_type, offset, inner_expr)
}

// HELPERS
// ================================================================================================

/// Returns a new [AccessType] with the dimension reduced by one. For example, a Matrix access
/// becomes a Vector access.
fn reduce_access_dim(var_name: &str, access_type: AccessType) -> Result<AccessType, SemanticError> {
    match access_type {
        AccessType::Default | AccessType::Slice(_) => Err(
            SemanticError::invalid_variable_access_type(var_name, &access_type),
        ),
        AccessType::Vector(_) => Ok(AccessType::Default),
        AccessType::Matrix(_, idx) => Ok(AccessType::Vector(idx)),
    }
}

/// Accesses into a `SymbolAccess` expression and returns a new `SymbolAccess` of a higher
/// dimension.
///
/// For example:
/// Suppose the `expr` is a [SymbolAccess] specifying that a binding `A` is being accessed as a
/// vector at index 0 (i.e. the access_type is [AccessType::Vector(0)], representing `A[0]`).
/// Suppose also the specified `access_type` is [AccessType::Vector(i)]. Then the resulting
/// expression would be A[i][0], represented by a new [SymbolAccess] with identifier `A` and
/// [AccessType::Matrix(0, i)].
///
/// # Errors
/// Returns an error if the expression is one that can't be accessed
fn access_inner_expr(
    parent_name: &str,
    access_type: AccessType,
    parent_offset: usize,
    expr: &Expression,
) -> Result<Expression, SemanticError> {
    match access_type {
        // access the entire expression
        AccessType::Default => Ok(expr.clone()),
        // TODO: handle slice access type
        AccessType::Slice(_) => todo!(),
        // access into the expression at the specified index
        AccessType::Vector(new_idx) => match expr {
            Expression::SymbolAccess(inner_binding) => match inner_binding.access_type() {
                AccessType::Default => {
                    let new_symbol_access = SymbolAccess::new(
                        inner_binding.ident().clone(),
                        AccessType::Vector(new_idx),
                        parent_offset + inner_binding.offset(),
                    );
                    Ok(Expression::SymbolAccess(new_symbol_access))
                }
                AccessType::Vector(old_idx) => {
                    let new_symbol_access = SymbolAccess::new(
                        inner_binding.ident().clone(),
                        AccessType::Matrix(*old_idx, new_idx),
                        parent_offset + inner_binding.offset(),
                    );
                    Ok(Expression::SymbolAccess(new_symbol_access))
                }
                _ => Err(SemanticError::invalid_variable_access_type(
                    inner_binding.name(),
                    &access_type,
                )),
            },
            _ => {
                // other variable value expressions cannot be accessed directly.
                Err(SemanticError::invalid_variable_access_type(
                    parent_name,
                    &access_type,
                ))
            }
        },
        // access into the expression at the specified row and column indices
        AccessType::Matrix(row_idx, col_idx) => match expr {
            Expression::SymbolAccess(inner_binding) => match inner_binding.access_type() {
                AccessType::Default => {
                    let new_symbol_access = SymbolAccess::new(
                        inner_binding.ident().clone(),
                        AccessType::Matrix(row_idx, col_idx),
                        parent_offset + inner_binding.offset(),
                    );
                    Ok(Expression::SymbolAccess(new_symbol_access))
                }
                _ => Err(SemanticError::invalid_variable_access_type(
                    inner_binding.name(),
                    &access_type,
                )),
            },
            _ => {
                // other variable value expressions cannot be accessed directly.
                Err(SemanticError::invalid_variable_access_type(
                    parent_name,
                    &access_type,
                ))
            }
        },
    }
}
