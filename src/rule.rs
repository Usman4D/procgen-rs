use crate::scope::Scope;

pub trait Rule {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn Rule>>>;
    fn is_terminal(&self) -> bool;
    fn scope(&self) -> Scope;
}
