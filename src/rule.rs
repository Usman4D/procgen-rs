use crate::{
    scope::Scope,
    symbol::{SymbolDat, SymbolData},
};

pub trait Rule {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn Rule>>>;
    fn is_terminal(&self) -> bool;
    fn scope(&self) -> Scope;
}
pub trait Rulea<T> {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>>;
    fn is_terminal(&self) -> bool;
    fn scope(&self) -> Scope;
    fn probability() -> f32;
}
pub trait RuleEvaluator {
    fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>>;
    fn get_symbol_data(&self) -> &SymbolDat;
}
