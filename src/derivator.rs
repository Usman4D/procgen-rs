use crate::{
    geometry::{Geometry, GeometryData},
    rule::Rule,
};

struct Derivator<R: Rule + Clone> {
    axiom: R,
}

impl<R: Rule + Clone> Derivator<R> {
    fn new(axiom: R) -> Self {
        Self { axiom }
    }
    fn derive(&self) -> GeometryData {
        let mut rules_vec = Vec::<Box<dyn Rule>>::with_capacity(50);

        let mut geometry_vec = Vec::<GeometryData>::with_capacity(10);
        rules_vec.append(&mut vec![Box::new(self.axiom.clone()) as Box<dyn Rule>]);
        'outer: loop {
            let mut rules_vec_back = Vec::<Box<dyn Rule>>::with_capacity(50);
            for rule in rules_vec.iter_mut() {
                if let Some(mut new_rules) = rule.evaluate() {
                    rules_vec_back.append(&mut new_rules);
                }

                if rule.is_terminal() {
                    let rule_scope = rule.scope();
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
        let mut verticies = Vec::<[f64; 3]>::with_capacity(50);
        let mut indicies = Vec::<usize>::with_capacity(50);

        for geometry in geometry_vec.iter_mut() {
            verticies.append(&mut geometry.verticies);
            indicies.append(&mut geometry.indicies);
        }

        GeometryData {
            verticies,
            indicies,
        }
    }
}
