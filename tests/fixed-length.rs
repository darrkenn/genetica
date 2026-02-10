use std::array;

use genetica::{
    crossover::{fixed_length_single_point_crossover, fixed_length_two_point_crossover},
    individual::{FixedLengthIndividual, Generate, Individual, Mutate},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct GeneType(pub bool);

impl Generate for GeneType {
    fn generate() -> Self {
        GeneType(rand::random_bool(0.25))
    }
}

impl Mutate for GeneType {
    fn mutate(&mut self) {
        if rand::random_range(0.00..1.00) <= 0.05 {
            self.0 = !self.0
        };
    }
}

#[derive(Debug, Clone, Copy)]
struct FixedLengthChromosome {
    genes: [GeneType; 4],
    fitness: f32,
}

impl Individual for FixedLengthChromosome {
    type GeneType = GeneType;

    fn new() -> Self {
        let genes: [GeneType; 4] = array::from_fn(|_| GeneType::generate());
        FixedLengthChromosome {
            genes,
            fitness: 0.00,
        }
    }
    fn mutate_genes(&mut self) {
        for gene in self.genes_mut() {
            gene.mutate();
        }
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
    fn fitness_mut(&mut self) -> &mut f32 {
        &mut self.fitness
    }
    fn calculate_fitness(&mut self) {
        self.fitness = 0.00;
    }
}

impl FixedLengthIndividual for FixedLengthChromosome {
    const GENES_SIZE: usize = 4;
    fn genes(&self) -> &[Self::GeneType] {
        &self.genes
    }
    fn genes_mut(&mut self) -> &mut [Self::GeneType] {
        &mut self.genes
    }
}

#[test]
fn test_fixed_single_point_crossover_success() {
    let parent1_genes: [GeneType; 4] = [
        GeneType(true),
        GeneType(true),
        GeneType(true),
        GeneType(true),
    ];
    let parent2_genes: [GeneType; 4] = [
        GeneType(false),
        GeneType(false),
        GeneType(false),
        GeneType(false),
    ];
    let parent1: FixedLengthChromosome = FixedLengthChromosome {
        genes: parent1_genes,
        fitness: 0.00,
    };
    let parent2: FixedLengthChromosome = FixedLengthChromosome {
        genes: parent2_genes,
        fitness: 0.00,
    };

    let (child1, child2) = fixed_length_single_point_crossover(&parent1, &parent2, 1.00);

    assert_ne!(child1.genes, parent1_genes);
    assert_ne!(child2.genes, parent2_genes);
}

#[test]
fn test_fixed_single_point_crossover_no_probability() {
    let parent1_genes: [GeneType; 4] = [
        GeneType(true),
        GeneType(true),
        GeneType(true),
        GeneType(true),
    ];
    let parent2_genes: [GeneType; 4] = [
        GeneType(false),
        GeneType(false),
        GeneType(false),
        GeneType(false),
    ];

    let parent1: FixedLengthChromosome = FixedLengthChromosome {
        genes: parent1_genes,
        fitness: 0.00,
    };
    let parent2: FixedLengthChromosome = FixedLengthChromosome {
        genes: parent2_genes,
        fitness: 0.00,
    };
    let (child1, child2) = fixed_length_single_point_crossover(&parent1, &parent2, 0.00);

    assert_eq!(child1.genes, parent1_genes);
    assert_eq!(child2.genes, parent2_genes);
}

#[test]
fn test_fixed_two_point_crossover_success() {
    let parent1_genes: [GeneType; 4] = [
        GeneType(true),
        GeneType(true),
        GeneType(true),
        GeneType(true),
    ];
    let parent2_genes: [GeneType; 4] = [
        GeneType(false),
        GeneType(false),
        GeneType(false),
        GeneType(false),
    ];

    let parent1: FixedLengthChromosome = FixedLengthChromosome {
        genes: parent1_genes,
        fitness: 0.00,
    };
    let parent2: FixedLengthChromosome = FixedLengthChromosome {
        genes: parent2_genes,
        fitness: 0.00,
    };
    let (child1, child2) = fixed_length_two_point_crossover(parent1, parent2, 1.00);

    assert_ne!(child1.genes, parent1_genes);
    assert_ne!(child2.genes, parent2_genes);
}
