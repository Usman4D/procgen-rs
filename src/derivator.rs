use crate::{
    geometry::{Geometry, GeometryData},
    rule::RuleEvaluator,
};

pub struct Derivator<R: RuleEvaluator + Clone> {
    axiom: R,
}

impl<R: RuleEvaluator + Clone> Derivator<R> {
    pub fn new(axiom: R) -> Self {
        Self { axiom }
    }
    pub fn derive(&self) -> GeometryData {
        let mut rules_vec = Vec::<Box<dyn RuleEvaluator>>::with_capacity(50);

        let mut geometry_vec = Vec::<GeometryData>::with_capacity(10);
        rules_vec.append(&mut vec![
            Box::new(self.axiom.clone()) as Box<dyn RuleEvaluator>
        ]);
        'outer: loop {
            let mut rules_vec_back = Vec::<Box<dyn RuleEvaluator>>::with_capacity(50);
            for rule in rules_vec.iter_mut() {
                if let Some(mut new_rules) = rule.evaluate_rules() {
                    rules_vec_back.append(&mut new_rules);
                }

                if rule.get_symbol_data().is_terminal {
                    let rule_scope = rule.get_symbol_data().scope.clone();
                    let geometry = Geometry::construct_from_scope(rule_scope);

                    geometry_vec.insert(geometry_vec.len(), geometry);
                    // continue;
                }
            }
            if rules_vec_back.len() == 0 {
                break 'outer;
            }

            rules_vec = rules_vec_back;
        }
        let mut verticies = Vec::<[f32; 3]>::with_capacity(50);
        let mut indicies = Vec::<u32>::with_capacity(50);

        for geometry in geometry_vec.iter_mut() {
            indicies.append(
                &mut geometry
                    .indicies
                    .iter()
                    .map(|x| x + verticies.len() as u32)
                    .collect(),
            );
            verticies.append(&mut geometry.verticies);
        }

        GeometryData {
            verticies,
            indicies,
        }
    }
}
