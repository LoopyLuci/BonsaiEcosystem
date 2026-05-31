pub mod evaluator;
pub mod model;
pub mod train;
pub mod training_loop;

pub use evaluator::NeuralEvaluator;
pub use model::GoNet;
pub use training_loop::GoTrainingLoop;
