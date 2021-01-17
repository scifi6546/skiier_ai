use nalgebra::Vector2;
#[derive(Clone)]
pub struct Topography {
    pub height: Vec<f32>,
    pub dimensions: Vector2<u32>,
}
impl Topography {
    pub fn iter(&self) -> std::slice::Iter<'_, f32> {
        self.height.iter()
    }

    pub fn flat(dimensions: Vector2<u32>) -> Self {
        Self {
            height: (0..dimensions.x * dimensions.y).map(|_| 0.0).collect(),
            dimensions,
        }
    }
    pub fn cone(dimensions: Vector2<u32>, center: Vector2<f32>, slope: f32) -> Self {
        Self {
            height: (0..dimensions.x * dimensions.y)
                .map(|i| {
                    let pos = Vector2::new((i / dimensions.y) as f32, (i % dimensions.y) as f32);
                    ((pos.x - center.x).powi(2) + (pos.y - center.y).powi(2)).sqrt() * slope
                })
                .collect(),
            dimensions,
        }
    }
    fn get(&self, index: Vector2<u32>) -> f32 {
        let i = index.x * self.dimensions.y + index.y;
        self.height[i as usize]
    }
    pub fn slope(&self, start: Vector2<u32>, end: Vector2<u32>) -> f32 {
        self.get(start) - self.get(end)
    }
}
