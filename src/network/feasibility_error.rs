//! Error type that represents an infeasible problem - e.g. too many or too few workers to assign to
//! tasks, or (much more expensive to identify) a situation wherein not all workers can be assigned
//! to the set of tasks because of their affinity scores.

use std::fmt;

#[derive(Debug, Clone)]
pub struct FeasibilityError {
    pub message: String
}

impl fmt::Display for FeasibilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}
