#[derive(Clone, Default)]
pub struct Scope {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub sx: f32,
    pub sy: f32,
    pub sz: f32,
}

impl Scope {
    pub fn ZERO() -> Self{
        Self{..Default::default()}
    }
    pub fn new(x: f32, y: f32, z: f32, sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            x,
            y,
            z,
            sx,
            sy,
            sz,
        }
    }
    pub fn set_size(&mut self, sx: f32, sy: f32, sz: f32) -> &Self {
        self.sx = sx;
        self.sy = sy;
        self.sz = sz;

        return self;
    }
    pub fn extrude(&mut self, sy: f32) -> &Self {
        self.sy += sy;

        self
    }
    pub fn extruded_scope(&self, sy: f32) -> Self {
        let mut new_scope = self.clone();

        new_scope.sy = sy;
        new_scope.y += self.sy;

        new_scope
    }
    pub fn get_face(&self, face: Face) -> Self {
        match face {
            Face::Top => Scope::new(self.x, self.y + self.sy, self.z, self.sx, 0f32, self.sz),
            Face::Bottom => todo!(),
            Face::Right => todo!(),
            Face::Left => todo!(),
            Face::Front => todo!(),
            Face::Back => todo!(),
        }
    }
    pub fn repeat(&self, dir: Direction, size: f32) -> Vec<Self> {
        match dir {
            Direction::X => {
                let width = (self.sx / size).floor();
                let count = width as usize;

                let mut scopes = Vec::<Self>::with_capacity(10);
                for x in 0..count {
                    let new_scope = Self::new(
                        self.x + width * x as f32,
                        self.y,
                        self.z,
                        width,
                        self.sy,
                        self.sz,
                    );
                    scopes.insert(scopes.len(), new_scope);
                }

                scopes
            }
            Direction::Y => {
                let height = (self.sx / size).floor();
                let count = height as usize;

                let mut scopes = Vec::<Self>::with_capacity(10);
                for i in 0..count {
                    let new_scope = Self::new(
                        self.x,
                        self.y + height * i as f32,
                        self.z,
                        self.sx,
                        height,
                        self.sz,
                    );
                    scopes.insert(scopes.len(), new_scope);
                }

                scopes
            }
            Direction::Z => {
                let breadth = (self.sx / size).floor();
                let count = breadth as usize;

                let mut scopes = Vec::<Self>::with_capacity(10);
                for i in 0..count {
                    let new_scope = Self::new(
                        self.x,
                        self.y,
                        self.z + breadth * i as f32,
                        self.sx,
                        self.sy,
                        breadth,
                    );
                    scopes.insert(scopes.len(), new_scope);
                }

                scopes
            }
        }
    }
    pub fn split(&self, dir: Direction, spec: &str, vals: &[f32]) -> Result<Vec<Self>, Self>{
        if spec.chars().count() != vals.len(){
            return Err(Scope::ZERO());
        }
        let mut total_absolute: f32 = 0.0;
        let mut total_relative: f32 = 0.0;

        let mut scopes = Vec::<Self>::with_capacity(vals.len());
        let count = vals.len();

        for i in 0..count{
            match spec.chars().nth(i).unwrap() {
                'a' => {total_absolute += vals[i]},
                'r' => {total_relative += vals[i]},
                _ => {},
            };
        }

        let mut last_size = 0.0;

        match dir{
            Direction::X => {
                let const_relative_size = (self.sx - total_absolute) / total_relative;
                for i in 0..count{
                    let size = match spec.chars().nth(i).unwrap() {
                        'a' => vals[i],
                        'r' => vals[i] * const_relative_size,
                        _ => 0.0,
                    };
                    let new_scope = Self::new(
                        self.x + last_size,
                        self.y,
                        self.z,
                        size,
                        self.sy,
                        self.sz,
                        );
                    last_size += size;

                    scopes.push(new_scope);
                }
            },
            Direction::Y => {
                let const_relative_size = (self.sy - total_absolute) / total_relative;
                for i in 0..count{
                    let size = match spec.chars().nth(i).unwrap() {
                        'a' => vals[i],
                        'r' => vals[i] * const_relative_size,
                        _ => 0.0,
                    };
                    let new_scope = Self::new(
                        self.x,
                        self.y + last_size,
                        self.z,
                        self.x,
                        size,
                        self.sz,
                        );
                    last_size += size;

                    scopes.push(new_scope);
                }
            },
            Direction::Z => {
                let const_relative_size = (self.sz - total_absolute) / total_relative;
                for i in 0..count{
                    let size = match spec.chars().nth(i).unwrap() {
                        'a' => vals[i],
                        'r' => vals[i] * const_relative_size,
                        _ => 0.0,
                    };
                    let new_scope = Self::new(
                        self.x,
                        self.y,
                        self.z + last_size,
                        self.sx,
                        self.sy,
                        size,
                        );
                    last_size += size;

                    scopes.push(new_scope);
                }
            },
        }

        Ok(scopes)
    }
    pub fn inset(&self, normal: Direction, amount: f32) -> Result<Vec<Self>, Self>{
        Err(Scope::ZERO())
    }
}
pub enum Face {
    Top,
    Bottom,
    Right,
    Left,
    Front,
    Back,
}
pub enum Direction {
    X,
    Y,
    Z,
}
