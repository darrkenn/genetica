/// A trait which allows a value to mutate itself in place
pub trait Mutate: Sized {
    fn mutate(&mut self);
}

/// A trait which allows a value to be generated
pub trait Generate: Sized {
    fn generate() -> Self;
}

/// A trait which represents a individual value.
pub trait Individual: Sized + Clone {
    type GeneType: Mutate + Generate + Clone + Copy;
    const GENES_SIZE: usize;

    fn new() -> Self;

    fn mutate_genes(&mut self);
    fn genes(&self) -> &[Self::GeneType];
    fn genes_mut(&mut self) -> &mut [Self::GeneType];

    fn fitness(&self) -> Option<f32>;
    fn fitness_mut(&mut self) -> &mut Option<f32>;
    fn calculate_fitness(&mut self);
}
