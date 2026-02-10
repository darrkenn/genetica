/// A trait which allows a value to mutate itself in place
pub trait Mutate: Sized {
    fn mutate(&mut self);
}

/// A trait which allows a value to be generated
pub trait Generate: Sized {
    fn generate() -> Self;
}

pub trait Individual: Sized + Clone {
    type GeneType: Mutate + Generate + Clone;

    fn new() -> Self;

    fn mutate_genes(&mut self);
    fn fitness(&self) -> f32;
    fn fitness_mut(&mut self) -> &mut f32;
    fn calculate_fitness(&mut self);
}

/// A trait which represents a individual of fixed length.
pub trait FixedLengthIndividual: Individual + Sized + Clone
where
    Self::GeneType: Copy,
{
    const GENES_SIZE: usize;
    fn genes(&self) -> &[Self::GeneType];
    fn genes_mut(&mut self) -> &mut [Self::GeneType];
}

/// A trait which represents a individual of dynamic length.
pub trait DynamicLengthIndividual: Individual + Sized + Clone {
    fn genes(&self) -> &Vec<Self::GeneType>;
    fn genes_mut(&mut self) -> &mut Vec<Self::GeneType>;
}
