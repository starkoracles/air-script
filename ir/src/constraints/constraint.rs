use super::{Boundary, NodeIndex, SemanticError};
use std::fmt::Display;

/// A [ConstraintRoot] represents the entry node of a subgraph within the [AlgebraicGraph]
/// representing a constraint. It also contains the [ConstraintDomain] for the constraint, which is
/// the domain against which the constraint should be applied.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstraintRoot {
    pub index: NodeIndex,
    pub domain: ConstraintDomain,
}

impl ConstraintRoot {
    /// Creates a new [ConstraintRoot] with the specified entry index and row offset.
    pub fn new(index: NodeIndex, domain: ConstraintDomain) -> Self {
        Self { index, domain }
    }

    /// Returns the index of the entry node of the subgraph representing the constraint.
    pub fn node_index(&self) -> &NodeIndex {
        &self.index
    }

    /// Returns the [ConstraintDomain] for this constraint, which specifies the rows against which
    /// the constraint should be applied.
    pub fn domain(&self) -> ConstraintDomain {
        self.domain
    }
}

/// The domain to which the constraint is applied, which is either the first or last row (for
/// boundary constraints), every row (for validity constraints), or every frame (for transition
/// constraints). When the constraint is applied to a frame the inner value specifies the size of
/// the frame. For example, for a transition constraint that is applied against the current and next
/// rows, the frame size will be 2.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Ord, PartialOrd)]
pub enum ConstraintDomain {
    FirstRow,          // for boundary constraints against the first row
    LastRow,           // for boundary constraints against the last row
    EveryRow,          // for validity constraints
    EveryFrame(usize), // for transition constraints
}

impl ConstraintDomain {
    /// Returns true if this domain is a boundary domain (FirstRow or LastRow).
    pub fn is_boundary(&self) -> bool {
        matches!(
            *self,
            ConstraintDomain::FirstRow | ConstraintDomain::LastRow
        )
    }

    /// Returns true if this domain is an integrity constraint domain.
    pub fn is_integrity(&self) -> bool {
        matches!(
            *self,
            ConstraintDomain::EveryRow | ConstraintDomain::EveryFrame(_)
        )
    }

    /// Combines two compatible [ConstraintDomain]s into a single [ConstraintDomain] that represents
    /// the maximum of the two. For example, if one domain is [ConstraintDomain::EveryFrame(2)] and
    /// the other is [ConstraintDomain::EveryFrame(3)], then the result will be
    /// [ConstraintDomain::EveryFrame(3)].
    ///
    /// # Errors
    /// Domains for boundary constraints (FirstRow and LastRow) cannot be merged with other domains.
    pub fn merge(&self, other: &ConstraintDomain) -> Result<ConstraintDomain, SemanticError> {
        if self == other {
            return Ok(*other);
        }

        match (self, other) {
            (ConstraintDomain::EveryFrame(a), ConstraintDomain::EveryRow) => {
                Ok(ConstraintDomain::EveryFrame(*a))
            }
            (ConstraintDomain::EveryRow, ConstraintDomain::EveryFrame(b)) => {
                Ok(ConstraintDomain::EveryFrame(*b))
            }
            (ConstraintDomain::EveryFrame(a), ConstraintDomain::EveryFrame(b)) => {
                Ok(ConstraintDomain::EveryFrame(*a.max(b)))
            }
            // otherwise, the domains are not compatible.
            _ => Err(SemanticError::incompatible_constraint_domains(self, other)),
        }
    }
}

impl From<usize> for ConstraintDomain {
    /// Creates a [ConstraintDomain] from the specified row offset.
    fn from(row_offset: usize) -> Self {
        if row_offset == 0 {
            ConstraintDomain::EveryRow
        } else {
            ConstraintDomain::EveryFrame(row_offset + 1)
        }
    }
}

impl From<Boundary> for ConstraintDomain {
    fn from(boundary: Boundary) -> Self {
        match boundary {
            Boundary::First => ConstraintDomain::FirstRow,
            Boundary::Last => ConstraintDomain::LastRow,
        }
    }
}

impl Display for ConstraintDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintDomain::FirstRow => write!(f, "the first row"),
            ConstraintDomain::LastRow => write!(f, "the last row"),
            ConstraintDomain::EveryRow => write!(f, "every row"),
            ConstraintDomain::EveryFrame(size) => {
                write!(f, "every frame of {size} consecutive rows")
            }
        }
    }
}
