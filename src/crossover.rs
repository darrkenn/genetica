use crate::individual::Individual;

/// Preforms a single point crossover on two chromosomes based on the crossover probability.
pub fn single_point_crossover<I>(parent1: &I, parent2: &I, crossover_probability: f32) -> (I, I)
where
    I: Individual,
{
    let mut child1 = parent1.clone();
    let mut child2 = parent2.clone();

    if rand::random::<f32>() <= crossover_probability {
        let genes_size = I::GENES_SIZE;
        let split: usize = rand::random_range(1..genes_size) as usize;
        let temp_slice: Vec<I::GeneType> = child1.genes()[split..].to_vec();
        child1.genes_mut()[split..].copy_from_slice(&child2.genes()[split..]);
        child2.genes_mut()[split..].copy_from_slice(&temp_slice);
    };
    (child1, child2)
}

/// Preforms a two point crossover on two chromosomes based on the crossover probability.
pub fn two_point_crossover<I>(mut parent1: I, mut parent2: I, crossover_probability: f32) -> (I, I)
where
    I: Individual,
{
    if rand::random::<f32>() <= crossover_probability {
        let genes_size = I::GENES_SIZE;
        let cut1 = rand::random_range(1..genes_size) as usize;
        let mut cut2 = rand::random_range(1..genes_size) as usize;

        while cut1 == cut2 {
            cut2 = rand::random_range(1..genes_size) as usize;
        }

        let (start, end) = if cut1 < cut2 {
            (cut1, cut2)
        } else {
            (cut2, cut1)
        };

        for i in start..end {
            let gene = parent1.genes()[i];
            parent1.genes_mut()[i] = parent2.genes()[i];
            parent2.genes_mut()[i] = gene;
        }
        (parent1, parent2)
    } else {
        (parent1, parent2)
    }
}
