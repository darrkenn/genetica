use std::cmp::min;

use crate::individual::{DynamicLengthIndividual, FixedLengthIndividual};

/// Preforms a single point crossover on two chromosomes of fixed length based on the crossover probability.
pub fn fixed_length_single_point_crossover<I>(
    parent1: &I,
    parent2: &I,
    crossover_probability: f32,
) -> (I, I)
where
    I: FixedLengthIndividual,
    I::GeneType: Copy,
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

/// Preforms a two point crossover on two chromosomes of fixed length based on the crossover probability.
pub fn fixed_length_two_point_crossover<I>(
    mut parent1: I,
    mut parent2: I,
    crossover_probability: f32,
) -> (I, I)
where
    I: FixedLengthIndividual,
    I::GeneType: Copy,
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

/// Preforms a single point crossover on two chromosomes of dynamic length based on the crossover probability.
pub fn dynamic_length_single_point_crossover<I>(
    parent1: &I,
    parent2: &I,
    crossover_probability: f32,
) -> (I, I)
where
    I: DynamicLengthIndividual,
{
    if rand::random_range(0.00..1.00) <= crossover_probability {
        let len1 = parent1.genes().len();
        let len2 = parent2.genes().len();
        let min_length = min(len1, len2);
        if min_length == 0 {
            return (parent1.clone(), parent2.clone());
        };
        let crossover_point = rand::random_range(0..min_length);

        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();

        child1.genes_mut().truncate(crossover_point);
        child1
            .genes_mut()
            .extend_from_slice(&parent2.genes()[crossover_point..]);

        child2.genes_mut().truncate(crossover_point);
        child2
            .genes_mut()
            .extend_from_slice(&parent1.genes()[crossover_point..]);

        (child1, child2)
    } else {
        (parent1.clone(), parent2.clone())
    }
}

/// Preforms a TWO point crossover on two chromosomes of dynamic length based on the crossover probability.
pub fn dynamic_length_two_point_crossover<I>(
    parent1: &I,
    parent2: &I,
    crossover_probability: f32,
) -> (I, I)
where
    I: DynamicLengthIndividual,
{
    if rand::random_range(0.00..1.00) <= crossover_probability {
        let len1 = parent1.genes().len();
        let len2 = parent2.genes().len();
        let min_length = min(len1, len2);

        if min_length < 2 {
            return (parent1.clone(), parent2.clone());
        }

        let point1 = rand::random_range(0..min_length - 1);
        let point2 = rand::random_range(point1 + 1..min_length);

        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();

        child1.genes_mut()[point1..point2].swap_with_slice(&mut child2.genes_mut()[point1..point2]);

        return (child1, child2);
    }

    (parent1.clone(), parent2.clone())
}
