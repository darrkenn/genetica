use crate::individual::{Chromosome, Generate, Mutate};

/// Generates a new population of chromosomes
///
///
pub fn generate_population<GeneType, const GENESIZE: usize>(
    population_size: i32,
) -> Vec<Chromosome<GeneType, GENESIZE>>
where
    GeneType: Generate + Mutate,
{
    let mut population: Vec<Chromosome<GeneType, GENESIZE>> = Vec::new();
    for _ in 0..population_size {
        population.push(Chromosome::new());
    }
    population
}

// Sorts a population by ascending fitness
pub fn sort_population_ascending<GeneType, const GENESIZE: usize>(
    population: &mut Vec<Chromosome<GeneType, GENESIZE>>,
) where
    GeneType: Generate + Mutate,
{
    population.sort_by(|a, b| a.fitness.unwrap().cmp(&b.fitness.unwrap()));
}

// Sorts a population by descending fitness
pub fn sort_population_descending<GeneType, const GENESIZE: usize>(
    population: &mut Vec<Chromosome<GeneType, GENESIZE>>,
) where
    GeneType: Generate + Mutate,
{
    population.sort_by(|a, b| b.fitness.unwrap().cmp(&a.fitness.unwrap()));
}
