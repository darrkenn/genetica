use std::array;

use genetica::crossover::{single_point_crossover, two_point_crossover};

use genetica::individual::{Generate, Individual, Mutate};

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
struct Chromosome {
    genes: [GeneType; 4],
    fitness: Option<f32>,
}

impl Individual for Chromosome {
    type GeneType = GeneType;
    const GENES_SIZE: usize = 4;
    fn new() -> Self {
        let genes: [GeneType; 4] = array::from_fn(|_| GeneType::generate());
        Chromosome {
            genes,
            fitness: None,
        }
    }
    fn mutate_genes(&mut self) {
        for gene in &mut self.genes {
            gene.mutate();
        }
    }
    fn genes(&self) -> &[Self::GeneType] {
        &self.genes
    }
    fn genes_mut(&mut self) -> &mut [Self::GeneType] {
        &mut self.genes
    }
    fn fitness(&self) -> Option<f32> {
        self.fitness
    }
    fn fitness_mut(&mut self) -> &mut Option<f32> {
        &mut self.fitness
    }
    fn calculate_fitness(&mut self) {
        self.fitness = Some(0.0);
    }
}

#[test]
fn test_single_point_crossover_success() {
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
    let parent1: Chromosome = Chromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: Chromosome = Chromosome {
        genes: parent2_genes,
        fitness: None,
    };

    let (child1, child2) = single_point_crossover(&parent1, &parent2, 1.00);

    assert_ne!(child1.genes, parent1_genes);
    assert_ne!(child2.genes, parent2_genes);
}

#[test]
fn test_single_point_crossover_no_probability() {
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

    let parent1: Chromosome = Chromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: Chromosome = Chromosome {
        genes: parent2_genes,
        fitness: None,
    };
    let (child1, child2) = single_point_crossover(&parent1, &parent2, 0.00);

    assert_eq!(child1.genes, parent1_genes);
    assert_eq!(child2.genes, parent2_genes);
}

#[test]
fn test_two_point_crossover_success() {
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

    let parent1: Chromosome = Chromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: Chromosome = Chromosome {
        genes: parent2_genes,
        fitness: None,
    };
    let (child1, child2) = two_point_crossover(parent1, parent2, 1.00);

    assert_ne!(child1.genes, parent1_genes);
    assert_ne!(child2.genes, parent2_genes);
}
