use geometry::{Geometry, GeometryData};
use rule::Rule;
use scope::{Face, Scope};
use symbol::{Symbol, SymbolData};

mod derivator;
mod geometry;
mod rule;
mod scope;
mod symbol;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let axiom = SymbolData::<AxiomSymbol>::new(
            Scope {
                ..Default::default()
            },
            false,
            AxiomSymbol {},
        );
    }
}
struct AxiomSymbol {}
struct HouseSymbol {}
struct RoofSymbol {}

struct HouseRule {}
struct AxiomRule {}

impl Rule for SymbolData<AxiomSymbol> {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn Rule>>> {
        let mut lot = Scope::default();
        lot.set_size(10f64, 0f64, 10f64);
        let house_a = SymbolData::<HouseSymbol>::new(lot, true, HouseSymbol {});
        println!("AxiomRule applied");
        Some(vec![Box::new(house_a)])
    }

    fn is_terminal(&self) -> bool {
        self.is_terminal
    }
    fn scope(&self) -> Scope {
        self.scope.clone()
    }
}
impl Rule for SymbolData<HouseSymbol> {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn Rule>>> {
        self.scope.extrude(10f64);
        let roof_a =
            SymbolData::<RoofSymbol>::new(self.scope.get_face(Face::Top), true, RoofSymbol {});
        println!("HouseRule applied");
        Some(vec![Box::new(roof_a)])
    }

    fn is_terminal(&self) -> bool {
        self.is_terminal
    }
    fn scope(&self) -> Scope {
        self.scope.clone()
    }
}
impl Rule for SymbolData<RoofSymbol> {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn Rule>>> {
        self.scope.extrude(5f64);
        println!("RoofRule applied");

        None
    }

    fn is_terminal(&self) -> bool {
        self.is_terminal
    }
    fn scope(&self) -> Scope {
        self.scope.clone()
    }
}
impl Symbol for AxiomSymbol {}
impl Symbol for HouseSymbol {}
impl Symbol for RoofSymbol {}

// impl AxiomSymbol {
//     fn new(scope: Scope, is_terminal: bool) -> Self {
//         Self {
//             scope: scope,
//             is_terminal: is_terminal,
//         }
//     }
//     fn evaluate(&mut self) -> Vec<Box<dyn Symbol>> {
//         self.scope.sx = 10.0;
//         self.scope.sy = 0.0;
//         self.scope.sz = 10.0;
//
//         let mut house = HouseSymbol::new(self.scope.clone(), false);
//         house.evaluate()
//     }
// }
// impl HouseSymbol {
//     fn new(scope: Scope, is_terminal: bool) -> Self {
//         Self {
//             scope: scope,
//             is_terminal: is_terminal,
//         }
//     }
//     pub fn evaluate(&mut self) -> Vec<Box<dyn Symbol>> {
//         self.scope.sy = 10.0;
//         let roof = RoofSymbol::new(self.scope.clone(), false);
//         vec![Box::new(roof)]
//     }
// }
// impl RoofSymbol {
//     fn new(scope: Scope, is_terminal: bool) -> Self {
//         Self {
//             scope: scope,
//             is_terminal: is_terminal,
//         }
//     }
// }
