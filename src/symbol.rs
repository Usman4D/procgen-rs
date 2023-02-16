use crate::{geometry::GeometryData, scope::Scope};

pub trait Symbol {
    // type enabled;
    // type parent;
    // type child;
    fn get_data(&self) -> &SymbolDat;
    fn get_data_mut(&mut self) -> &mut SymbolDat;
}
#[derive(Default, Clone)]
pub struct SymbolDat {
    pub scope: Scope,
    pub is_terminal: bool,
}
pub struct SymbolData<T: Symbol> {
    pub scope: Scope,
    pub is_terminal: bool,
    symbol: T,
    pub geometry_data: Option<GeometryData>,
}
impl<T: Symbol> SymbolData<T> {
    pub fn new(scope: Scope, is_terminal: bool, symbol: T) -> Self {
        Self {
            scope,
            is_terminal,
            symbol,
            geometry_data: None,
        }
    }
}
