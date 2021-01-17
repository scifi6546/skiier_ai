use nalgebra::Vector2;
#[derive(Clone)]
pub struct Topography {
    pub height: Vec<f32>,
    pub edges: Vec<f32>,
    pub dimensions: Vector2<u32>,
}
impl Topography {
    pub fn iter(&self) -> std::slice::Iter<'_, f32> {
        self.height.iter()
    }

    pub fn flat(dimensions: Vector2<u32>) -> Self {
        let mut s = Self {
            height: (0..dimensions.x * dimensions.y).map(|_| 0.0).collect(),
            edges: vec![],
            dimensions,
        };
        s.build_slope();
        s
    }
    fn get(&self, index: Vector2<u32>) -> f32 {
        let i = index.x * self.dimensions.y + index.y;
        self.height[i as usize]
    }
    pub fn slope(&self, start: Vector2<u32>, end: Vector2<u32>) -> f32 {
        todo!()
    }
    /// Rebuilds Slope from self height
    fn build_slope(&mut self) {
        self.edges = vec![0.0; (self.dimensions.x as usize - 1) * (self.dimensions.y as usize - 1)];
        for i in 0..self.dimensions.x - 1 {
            for j in 0..self.dimensions.y - 1 {}
        }
    }
}
