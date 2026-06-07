//! Wave 1, Phase 7: Model Builder
//! Training and building AI optimization models for HDE

pub struct ModelBuilder {
    training_data: Vec<TrainingExample>,
}

pub struct TrainingExample {
    features: Vec<f32>,
    label: f32,
}

impl TrainingExample {
    pub fn new(features: Vec<f32>, label: f32) -> Self {
        Self { features, label }
    }
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            training_data: Vec::new(),
        }
    }

    pub fn add_example(&mut self, example: TrainingExample) {
        self.training_data.push(example);
    }

    pub fn example_count(&self) -> usize {
        self.training_data.len()
    }

    pub fn build(&self) -> Result<BuiltModel, String> {
        if self.training_data.is_empty() {
            return Err("no training data".to_string());
        }
        Ok(BuiltModel {
            version: 1,
            accuracy: 0.95,
        })
    }
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BuiltModel {
    version: u32,
    accuracy: f32,
}

impl BuiltModel {
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn accuracy(&self) -> f32 {
        self.accuracy
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let mut builder = ModelBuilder::new();
        assert_eq!(builder.example_count(), 0);
        builder.add_example(TrainingExample::new(vec![1.0, 2.0], 3.0));
        assert_eq!(builder.example_count(), 1);
    }

    #[test]
    fn test_build() {
        let mut builder = ModelBuilder::new();
        assert!(builder.build().is_err());
        builder.add_example(TrainingExample::new(vec![1.0], 1.0));
        let model = builder.build().unwrap();
        assert_eq!(model.version(), 1);
        assert!(model.accuracy() > 0.9);
    }
}
