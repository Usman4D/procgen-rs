use crate::scope::Scope;

pub trait Symbol {
    // type enabled;
    // type parent;
    // type child;
    fn get_data(&self) -> &SymbolData;
    fn get_data_mut(&mut self) -> &mut SymbolData;
}
#[derive(Default, Clone)]
pub struct SymbolData {
    pub scope: Scope,
    pub is_terminal: bool,
}
