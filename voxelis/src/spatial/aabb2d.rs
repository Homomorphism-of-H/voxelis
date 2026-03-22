use glam::Vec2;

#[derive(Debug)]
pub struct Aabb2d {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb2d {
    #[must_use]
    pub const fn with_min_max(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub fn with_position_and_size(position: Vec2, size: Vec2) -> Self {
        Self {
            min: position,
            max: position + size,
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    #[must_use]
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    #[must_use]
    pub const fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    #[must_use]
    pub const fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}
