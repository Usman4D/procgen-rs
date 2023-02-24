use crate::scope::Scope;

#[derive(Debug)]
pub struct GeometryData {
    pub verticies: Vec<[f32; 3]>,
    pub indicies: Vec<u32>,
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
        let mut verticies_vec = Vec::<[f32; 3]>::with_capacity(8);
        verticies_vec.insert(0, [scope.x, scope.y, scope.z]);
        verticies_vec.insert(1, [scope.x, scope.y, scope.z + scope.sz]);
        verticies_vec.insert(2, [scope.x + scope.sx, scope.y, scope.z + scope.sz]);
        verticies_vec.insert(3, [scope.x + scope.sx, scope.y, scope.z]);

        verticies_vec.insert(4, [scope.x, scope.y + scope.sy, scope.z]);
        verticies_vec.insert(5, [scope.x, scope.y + scope.sy, scope.z + scope.sz]);
        verticies_vec.insert(
            6,
            [scope.x + scope.sx, scope.y + scope.sy, scope.z + scope.sz],
        );
        verticies_vec.insert(7, [scope.x + scope.sx, scope.y + scope.sy, scope.z]);

        let mut indicies_vec = Vec::<u32>::with_capacity(24);

        indicies_vec.append(&mut vec![2, 1, 0]);
        indicies_vec.append(&mut vec![0, 3, 2]);

        indicies_vec.append(&mut vec![0, 4, 7]);
        indicies_vec.append(&mut vec![7, 3, 0]);

        indicies_vec.append(&mut vec![1, 5, 4]);
        indicies_vec.append(&mut vec![4, 0, 1]);

        indicies_vec.append(&mut vec![3, 7, 6]);
        indicies_vec.append(&mut vec![6, 2, 3]);

        indicies_vec.append(&mut vec![2, 6, 5]);
        indicies_vec.append(&mut vec![5, 1, 2]);

        indicies_vec.append(&mut vec![5, 6, 7]);
        indicies_vec.append(&mut vec![7, 4, 5]);

        GeometryData {
            verticies: verticies_vec,
            indicies: indicies_vec,
        }
    }
    // fn load_obj_with_scope() -> GeometryData {}
}
