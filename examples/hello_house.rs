extern crate geometry;

use geometry::derivator::Derivator;
use geometry::rule::Rule;
use geometry::scope::{Face, Scope};
use geometry::symbol::{Symbol, SymbolDat, SymbolData};
fn main() {
    let symbol_data = SymbolDat {
        scope: Scope {
            ..Default::default()
        },
        is_terminal: false,
    };
    let axiom = AxiomSymbol { data: symbol_data };
    let derivator = Derivator::new(axiom);
}

#[derive(Default, Clone)]
struct AxiomSymbol {
    data: SymbolDat,
}
#[derive(Default, Clone)]
struct HouseSymbol {
    data: SymbolDat,
}
#[derive(Default, Clone)]
struct RoofSymbol {
    data: SymbolDat,
}

impl Symbol for AxiomSymbol {
    fn get_data(&self) -> &geometry::symbol::SymbolDat {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut geometry::symbol::SymbolDat {
        &mut self.data
    }
}
impl Symbol for HouseSymbol {
    fn get_data(&self) -> &geometry::symbol::SymbolDat {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut geometry::symbol::SymbolDat {
        &mut self.data
    }
}
impl Symbol for RoofSymbol {
    fn get_data(&self) -> &geometry::symbol::SymbolDat {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut geometry::symbol::SymbolDat {
        &mut self.data
    }
}

impl Rule for AxiomSymbol {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn Rule>>> {
        let mut lot = Scope::default();
        lot.set_size(10f64, 0f64, 10f64);

        let symbol_data = SymbolDat {
            scope: lot,
            is_terminal: true,
        };
        let house = HouseSymbol { data: symbol_data };
        println!("AxiomRule applied");
        Some(vec![Box::new(house)])
    }

    fn is_terminal(&self) -> bool {
        self.get_data().is_terminal
    }
    fn scope(&self) -> Scope {
        self.get_data().scope.clone()
    }
}
impl Rule for HouseSymbol {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn Rule>>> {
        self.get_data_mut().scope.extrude(10f64);
        let symbol_data = SymbolDat {
            scope: self.get_data().scope.get_face(Face::Top),
            is_terminal: true,
        };
        let roof = RoofSymbol { data: symbol_data };

        println!("HouseRule applied");
        Some(vec![Box::new(roof)])
    }

    fn is_terminal(&self) -> bool {
        self.get_data().is_terminal
    }
    fn scope(&self) -> Scope {
        self.get_data().scope.clone()
    }
}
impl Rule for RoofSymbol {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn Rule>>> {
        self.get_data_mut().scope.extrude(5f64);
        println!("RoofRule applied");

        None
    }

    fn is_terminal(&self) -> bool {
        self.get_data().is_terminal
    }
    fn scope(&self) -> Scope {
        self.get_data().scope.clone()
    }
}
