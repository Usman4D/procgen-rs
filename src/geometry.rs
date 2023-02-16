use crate::scope::Scope;

#[derive(Debug)]
pub struct GeometryData {
    pub verticies: Vec<[f64; 3]>,
    pub indicies: Vec<usize>,
}
pub struct MaterialData {
    pub color: [u8; 3],
}
pub struct Geometry {
    mesh_format: MeshFormat,
}
enum MeshFormat {
    TriangleList,
    QuadList,
}
impl Geometry {
    pub fn construct_from_scope(scope: Scope) -> GeometryData {
        let mut verticies_vec = Vec::<[f64; 3]>::with_capacity(8);
        verticies_vec.insert(0, [scope.x, scope.y, scope.z]);
        verticies_vec.insert(1, [scope.x, scope.y, scope.z + scope.sz]);
        verticies_vec.insert(2, [scope.x + scope.sx, scope.y, scope.z]);
        verticies_vec.insert(3, [scope.x + scope.sx, scope.y, scope.z + scope.sz]);

        verticies_vec.insert(4, [scope.x, scope.y + scope.sy, scope.z]);
        verticies_vec.insert(5, [scope.x, scope.y + scope.sy, scope.z + scope.sz]);
        verticies_vec.insert(6, [scope.x + scope.sx, scope.y + scope.sy, scope.z]);
        verticies_vec.insert(
            7,
            [scope.x + scope.sx, scope.y + scope.sy, scope.z + scope.sz],
        );

        let mut indicies_vec = Vec::<usize>::with_capacity(24);

        indicies_vec.append(&mut vec![0, 1, 2, 3]);
        indicies_vec.append(&mut vec![7, 4, 0, 3]);
        indicies_vec.append(&mut vec![4, 5, 1, 0]);
        indicies_vec.append(&mut vec![6, 7, 3, 2]);
        indicies_vec.append(&mut vec![5, 6, 2, 1]);
        indicies_vec.append(&mut vec![7, 6, 5, 4]);

        GeometryData {
            verticies: verticies_vec,
            indicies: indicies_vec,
        }
    }
    // fn load_obj_with_scope() -> GeometryData {}
}
