use crate::individual::{Chromosome, Generate, Mutate};

/// Preforms a single point crossover on two chromosomes based on the crossover probability.
pub fn single_point_crossover<GeneType, const GENESIZE: usize>(
    mut parent1: Chromosome<GeneType, GENESIZE>,
    mut parent2: Chromosome<GeneType, GENESIZE>,
    crossover_probability: f32,
) -> (
    Chromosome<GeneType, GENESIZE>,
    Chromosome<GeneType, GENESIZE>,
)
where
    GeneType: Generate + Mutate + Copy,
{
    if rand::random::<f32>() <= crossover_probability {
        let split: usize = rand::random_range(0..GENESIZE) as usize;
        let (_p1_start, p1_end) = parent1.genes.split_at(split);
        let (_p2_start, p2_end) = parent2.genes.split_at(split);
        let p1_end = p1_end.to_vec();
        let p2_end = p2_end.to_vec();

        parent1.genes[split..].copy_from_slice(&p2_end);
        parent2.genes[split..].copy_from_slice(&p1_end);

        (parent1, parent2)
    } else {
        (parent1, parent2)
    }
}

/// Preforms a two point crossover on two chromosomes based on the crossover probability.
pub fn two_point_crossover<GeneType, const GENESIZE: usize>(
    mut parent1: Chromosome<GeneType, GENESIZE>,
    mut parent2: Chromosome<GeneType, GENESIZE>,
    crossover_probability: f32,
) -> (
    Chromosome<GeneType, GENESIZE>,
    Chromosome<GeneType, GENESIZE>,
)
where
    GeneType: Generate + Mutate + Copy,
{
    if rand::random::<f32>() <= crossover_probability {
        let cut1 = rand::random_range(0..GENESIZE) as usize;
        let mut cut2 = rand::random_range(0..GENESIZE) as usize;

        while cut1 == cut2 {
            cut2 = rand::random_range(1..GENESIZE) as usize;
        }

        let (start, end) = if cut1 < cut2 {
            (cut1, cut2)
        } else {
            (cut2, cut1)
        };

        for i in start..end {
            let gene = parent1.genes[i];
            parent1.genes[i] = parent2.genes[i];
            parent2.genes[i] = gene;
        }
        (parent1, parent2)
    } else {
        (parent1, parent2)
    }
}
