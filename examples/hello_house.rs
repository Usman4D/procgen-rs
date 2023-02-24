extern crate geometry;
mod freecamera;

use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use geometry::derivator::Derivator;
use geometry::rule::{Rule, RuleEvaluator, Rulea};
use geometry::scope::{Direction, Face, Scope};
use geometry::symbol::{Symbol, SymbolDat, SymbolData};
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
    let symbol_data = SymbolDat {
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
        lot.set_size(12f32, 0f32, 8f32);
        let scopes = lot.repeat(Direction::X, 4.0);

        let mut house_symbols = Vec::<Box<dyn Rule>>::with_capacity(scopes.len());
        for x in 0..scopes.len() {
            let symbol_data = SymbolDat {
                scope: scopes[x].clone(),
                is_terminal: true,
            };
            let house = HouseSymbol { data: symbol_data };

            house_symbols.insert(x, Box::new(house));
        }
        println!("AxiomRule applied");
        Some(house_symbols)
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
        self.get_data_mut().scope.extrude(8f32);
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
        self.get_data_mut().scope.extrude(5f32);
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
struct Axiom;
struct HouseSimple;
struct HouseComplex;
struct RoofSimple;
impl RuleEvaluator for AxiomSymbol {
    fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        Rulea::<Axiom>::evaluate(self)
    }

    fn get_symbol_data(&self) -> &SymbolDat {
        self.get_data()
    }
}
impl RuleEvaluator for HouseSymbol {
    fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        Rulea::<HouseSimple>::evaluate(self)
    }

    fn get_symbol_data(&self) -> &SymbolDat {
        self.get_data()
    }
}
impl RuleEvaluator for RoofSymbol {
    fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        Rulea::<RoofSimple>::evaluate(self)
    }

    fn get_symbol_data(&self) -> &SymbolDat {
        self.get_data()
    }
}
// impl RuleEvaluator for HouseSymbol{
//     fn evaluate_rules(&mut self) -> Option<Vec<Box<dyn Rule>>> {
//         Rulea::<HouseSimple>::evaluate(self)
//     }
// }
impl Rulea<Axiom> for AxiomSymbol{
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        let mut lot = Scope::default();
        lot.set_size(12f32, 0f32, 8f32);
        let scopes = lot.repeat(Direction::X, 4.0);

        let mut house_symbols = Vec::<Box<dyn RuleEvaluator>>::with_capacity(scopes.len());
        for x in 0..scopes.len() {
            let symbol_data = SymbolDat {
                scope: scopes[x].clone(),
                is_terminal: true,
            };
            let house = HouseSymbol { data: symbol_data };

            house_symbols.insert(x, Box::new(house));
        }
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
impl Rulea<HouseSimple> for HouseSymbol {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        self.get_data_mut().scope.extrude(8f32);
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

    fn probability() -> f32 {
        1.0
    }
}
impl Rulea<RoofSimple> for RoofSymbol {
    fn evaluate(&mut self) -> Option<Vec<Box<dyn RuleEvaluator>>> {
        self.get_data_mut().scope.extrude(5f32);
        println!("RoofRule applied");

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
