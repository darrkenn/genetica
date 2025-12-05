use crate::individual::Individual;

/// Generates a new population of chromosomes
pub fn generate_population<I>(population_size: i32) -> Vec<I>
where
    I: Individual,
{
    let mut population: Vec<I> = Vec::new();
    for _ in 0..population_size {
        population.push(Individual::new());
    }
    population
}

/// Sorts a population by ascending fitness
pub fn sort_population_ascending<I>(population: &mut Vec<I>)
where
    I: Individual,
{
    population.sort_by(|a, b| a.fitness().unwrap().cmp(&b.fitness().unwrap()));
}

/// Sorts a population by descending fitness
pub fn sort_population_descending<I>(population: &mut Vec<I>)
where
    I: Individual,
{
    population.sort_by(|a, b| b.fitness().unwrap().cmp(&a.fitness().unwrap()));
}
