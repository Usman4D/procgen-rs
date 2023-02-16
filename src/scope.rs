#[derive(Clone, Default)]
pub struct Scope {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

impl Scope {
    pub fn new(x: f64, y: f64, z: f64, sx: f64, sy: f64, sz: f64) -> Self {
        Self {
            x,
            y,
            z,
            sx,
            sy,
            sz,
        }
    }
    pub fn set_size(&mut self, sx: f64, sy: f64, sz: f64) -> &Self {
        self.sx = sx;
        self.sy = sy;
        self.sz = sz;

        return self;
    }
    pub fn extrude(&mut self, sy: f64) -> &Self {
        self.sy = sy;

        self
    }
    pub fn get_face(&self, face: Face) -> Self {
        match face {
            Face::Top => Scope::new(self.x, self.y + self.sy, self.z, self.sx, 0f64, self.sz),
            Face::Bottom => todo!(),
            Face::Right => todo!(),
            Face::Left => todo!(),
            Face::Front => todo!(),
            Face::Back => todo!(),
        }
    }
    pub fn repeat(&self, dir: Direction, size: f64) -> Vec<Self> {
        match dir {
            Direction::X => {
                let width = (self.sx / size).floor();
                let count = width as usize;

                let mut scopes = Vec::<Self>::with_capacity(10);
                for x in 0..count {
                    let new_scope = Self::new(
                        self.x + width * x as f64,
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
                        self.y + height * i as f64,
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
                        self.z + breadth * i as f64,
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
