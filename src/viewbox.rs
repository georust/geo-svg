#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ViewBox {
    pub min_x: Option<f32>,
    pub min_y: Option<f32>,
    pub max_x: Option<f32>,
    pub max_y: Option<f32>,
}

impl ViewBox {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x: Some(min_x),
            min_y: Some(min_y),
            max_x: Some(max_x),
            max_y: Some(max_y),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            min_x: Self::min_option(self.min_x, other.min_x),
            min_y: Self::min_option(self.min_y, other.min_y),
            max_x: Self::max_option(self.max_x, other.max_x),
            max_y: Self::max_option(self.max_y, other.max_y),
        }
    }

    pub fn min_x(&self) -> f32 {
        self.min_x.unwrap_or_default()
    }

    pub fn min_y(&self) -> f32 {
        self.min_y.unwrap_or_default()
    }

    pub fn max_x(&self) -> f32 {
        self.max_x.unwrap_or_default()
    }

    pub fn max_y(&self) -> f32 {
        self.max_y.unwrap_or_default()
    }

    pub fn width(&self) -> f32 {
        (self.min_x() - self.max_x()).abs()
    }

    pub fn height(&self) -> f32 {
        (self.min_y() - self.max_y()).abs()
    }

    fn min_option(a: Option<f32>, b: Option<f32>) -> Option<f32> {
        match (a, b) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    fn max_option(a: Option<f32>, b: Option<f32>) -> Option<f32> {
        match (a, b) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.min_x = self.min_x.map(|x| x - margin);
        self.min_y = self.min_y.map(|y| y - margin);
        self.max_x = self.max_x.map(|x| x + margin);
        self.max_y = self.max_y.map(|y| y + margin);
        self
    }
}
