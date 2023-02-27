extern crate geometry;
mod freecamera;

use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use geometry::derivator::Derivator;
use geometry::rule::{Rule, RuleEvaluator};
use geometry::scope::{Direction, Face, Scope};
use geometry::symbol::{Symbol, SymbolData};
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(freecamera::FreeCameraPlugin)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let symbol_data = SymbolData {
        scope: Scope {
            ..Default::default()
        },
        is_terminal: false,
    };
    let axiom = AxiomSymbol { data: symbol_data };
    let derivator = Derivator::new(axiom);
    let geometry_data = derivator.derive();

    dbg!(&geometry_data);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // Positions of the vertices
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, geometry_data.verticies);

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(geometry_data.indicies)));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Wireframe);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[derive(Default, Clone)]
struct AxiomSymbol {
    data: SymbolData,
}
#[derive(Default, Clone)]
struct HouseSymbol {
    data: SymbolData,
}
#[derive(Default, Clone)]
struct RoofSymbol {
    data: SymbolData,
}
#[derive(Default, Clone)]
struct FacadeSymbol {
    data: SymbolData,
}

impl Symbol for AxiomSymbol {
    fn get_data(&self) -> &geometry::symbol::SymbolData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut geometry::symbol::SymbolData {
        &mut self.data
    }
}
impl Symbol for HouseSymbol {
    fn get_data(&self) -> &geometry::symbol::SymbolData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut geometry::symbol::SymbolData {
        &mut self.data
    }
}
impl Symbol for RoofSymbol {
    fn get_data(&self) -> &geometry::symbol::SymbolData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut geometry::symbol::SymbolData {
        &mut self.data
    }
}
impl Symbol for FacadeSymbol {
    fn get_data(&self) -> &geometry::symbol::SymbolData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut geometry::symbol::SymbolData {
        &mut self.data
    }
}

struct Axiom;
struct HouseSingleRoom;
struct HouseDualRoom;
struct HouseTriRoom;
struct RoofSimple;
struct Facade;
impl RuleEvaluator for AxiomSymbol {
    fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        Rule::<Axiom>::evaluate(self)
    }

    fn get_symbol_data(&self) -> &SymbolData {
        self.get_data()
    }
}
impl RuleEvaluator for HouseSymbol {
    fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        Rule::<HouseTriRoom>::evaluate(self)
    }

    fn get_symbol_data(&self) -> &SymbolData {
        self.get_data()
    }
}
impl RuleEvaluator for RoofSymbol {
    fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        Rule::<RoofSimple>::evaluate(self)
    }

    fn get_symbol_data(&self) -> &SymbolData {
        self.get_data()
    }
}
impl RuleEvaluator for FacadeSymbol {
    fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        Rule::<Facade>::evaluate(self)
    }

    fn get_symbol_data(&self) -> &SymbolData {
        self.get_data()
    }
}
// impl RuleEvaluator for HouseSymbol{
//     fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn Rule>>> {
//         Rulea::<HouseSimple>::evaluate(self)
//     }
// }
impl Rule<Axiom> for AxiomSymbol{
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        let mut lot = Scope::default();
        lot.set_size(12f32, 0f32, 8f32);
        // let scopes = lot.repeat(Direction::X, 4.0);

        let mut house_symbols = Vec::<Box<dyn RuleEvaluator>>::with_capacity(1);
        // for x in 0..scopes.len() {
        //     let symbol_data = SymbolData {
        //         scope: scopes[x].clone(),
        //         is_terminal: true,
        //     };
        //     let house = HouseSymbol { data: symbol_data };
        //
        //     house_symbols.insert(x, Box::new(house));
        // }
        let symbol_data = SymbolData {
            scope: lot.clone(),
            is_terminal: false,
        };
        let house = HouseSymbol { data: symbol_data };

        house_symbols.insert(0, Box::new(house));
        println!("AxiomRule applied");
        Some(house_symbols)
    }

    fn is_terminal(&self) -> bool {
        self.get_data().is_terminal
    }

    fn scope(&self) -> Scope {
        self.get_data().scope.clone()
    }

    fn probability() -> f32 {
        1.0
    }
}
impl Rule<HouseSingleRoom> for HouseSymbol {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        self.get_data_mut().scope.extrude(6f32);
        let symbol_data = SymbolData {
            scope: self.get_data().scope.get_face(Face::Top),
            is_terminal: false,
        };
        let roof = RoofSymbol { data: symbol_data };

        let facade_symbol_data = SymbolData {
            scope: self.get_data().scope.clone(),
            is_terminal: true,
        };
        let facade = FacadeSymbol{data:facade_symbol_data};

        println!("HouseRule applied");
        Some(vec![Box::new(roof),Box::new(facade)])
    }

    fn is_terminal(&self) -> bool {
        self.get_data().is_terminal
    }
    fn scope(&self) -> Scope {
        self.get_data().scope.clone()
    }

    fn probability() -> f32 {
        1.0
    }
}
impl Rule<HouseDualRoom> for HouseSymbol{
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        let scope_split_result = self.get_data().scope.clone().extrude(6.0).split(Direction::X, "rr", &[2.0,1.0]);
        let splits = match scope_split_result{
            Ok(val) => val,
            Err(val) => {
                warn!("split operation failed");
                vec![val]
            },
        };
        let mut new_scopes = Vec::<Box<dyn RuleEvaluator>>::with_capacity(splits.len());
        let mut index = 0;
        for mut scope in splits{
            let mut top_face = scope.get_face(Face::Top);
            if index == 0{
                top_face.extrude(1.0);
            } else{
                top_face.extrude(0.0);
            }
            let symbol_data = SymbolData {
                scope: top_face.get_face(Face::Top),
                is_terminal: false,
            };
            let roof = RoofSymbol { data: symbol_data };

            let facade_symbol_data = SymbolData {
                scope: scope.clone(),
                is_terminal: true,
            };
            let facade = FacadeSymbol{data:facade_symbol_data};

            let facade_symbol_data_2 = SymbolData {
                scope: top_face.clone(),
                is_terminal: true,
            };
            let facade_2 = FacadeSymbol{data:facade_symbol_data_2};

            new_scopes.push(Box::new(roof));
            new_scopes.push(Box::new(facade));
            new_scopes.push(Box::new(facade_2));

            index+=1;
        }

        println!("HouseRule applied");
        Some(new_scopes)
    }

    fn is_terminal(&self) -> bool {
        todo!()
    }

    fn scope(&self) -> Scope {
        todo!()
    }

    fn probability() -> f32 {
        todo!()
    }
}
impl Rule<HouseTriRoom> for HouseSymbol{
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        let scope_split_result = self.get_data().scope.clone().extrude(6.0).split(Direction::X, "rrr", &[1.1,1.0,1.1]);
        let splits = match scope_split_result{
            Ok(val) => val,
            Err(val) => {
                warn!("split operation failed");
                vec![val]
            },
        };
        let mut new_scopes = Vec::<Box<dyn RuleEvaluator>>::with_capacity(splits.len());
        let mut index = 0;
        for mut scope in splits{
            let mut top_face = scope.get_face(Face::Top);
            if index == 1{
                top_face.extrude(3.0);
            } else{
                top_face.extrude(0.0);
            }
            let symbol_data = SymbolData {
                scope: top_face.get_face(Face::Top),
                is_terminal: false,
            };
            let roof = RoofSymbol { data: symbol_data };

            let facade_symbol_data = SymbolData {
                scope: scope.clone(),
                is_terminal: true,
            };
            let facade = FacadeSymbol{data:facade_symbol_data};

            let facade_symbol_data_2 = SymbolData {
                scope: top_face.clone(),
                is_terminal: true,
            };
            let facade_2 = FacadeSymbol{data:facade_symbol_data_2};

            new_scopes.push(Box::new(roof));
            new_scopes.push(Box::new(facade));
            new_scopes.push(Box::new(facade_2));

            index+=1;
        }

        println!("HouseRule applied");
        Some(new_scopes)
    }

    fn is_terminal(&self) -> bool {
        todo!()
    }

    fn scope(&self) -> Scope {
        todo!()
    }

    fn probability() -> f32 {
        todo!()
    }
}
impl Rule<RoofSimple> for RoofSymbol {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        self.get_data_mut().scope.extrude(1.0f32);

        let scope_split_result = self.get_data().scope.split(Direction::X, "ara", &[0.5,1.0,0.5]);
        let splits = match scope_split_result{
            Ok(val) => val,
            Err(val) => {
                warn!("split operation failed");
                vec![val]
            },
        };

        let mut facades = Vec::<Box<dyn RuleEvaluator>>::with_capacity(3);
        for s in 0..3{
            if s == 1{
                let scope_split_result = splits[s].split(Direction::Z, "ara", &[0.5,1.0,0.5]);
                let mut splits = match scope_split_result{
                    Ok(val) => val,
                    Err(val) => {
                        warn!("split operation failed");
                        vec![val]
                    },
                };

                for y in 0..3{
                    if y ==1{
                        splits[y].extrude(-0.5);
                    }
                    let facade_symbol_data = SymbolData {
                        scope: splits[y].clone(),
                        is_terminal: true,
                    };
                    let mut facade = FacadeSymbol{data:facade_symbol_data};


                    facades.push(Box::new(facade));

                }
            }
            else{
                let facade_symbol_data = SymbolData {
                    scope: splits[s].clone(),
                    is_terminal: true,
                };
                let mut facade = FacadeSymbol{data:facade_symbol_data};

                facades.push(Box::new(facade));

            }


        }

        println!("RoofRule applied");
        Some(facades)
    }

    fn is_terminal(&self) -> bool {
        self.get_data().is_terminal
    }
    fn scope(&self) -> Scope {
        self.get_data().scope.clone()
    }

    fn probability() -> f32 {
        1.0
    }
}
impl Rule<Facade> for FacadeSymbol {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        println!("FacadeRule applied");

        None
    }

    fn is_terminal(&self) -> bool {
        self.get_data().is_terminal
    }
    fn scope(&self) -> Scope {
        self.get_data().scope.clone()
    }

    fn probability() -> f32 {
        1.0
    }
}
