use genetica::crossover::{single_point_crossover, two_point_crossover};
use genetica::individual::Chromosome;

use genetica::individual::{Generate, Mutate};

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
    let parent1: Chromosome<GeneType, 4> = Chromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: Chromosome<GeneType, 4> = Chromosome {
        genes: parent2_genes,
        fitness: None,
    };
    let (child1, child2) = single_point_crossover(parent1, parent2, 1.00);

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

    let parent1: Chromosome<GeneType, 4> = Chromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: Chromosome<GeneType, 4> = Chromosome {
        genes: parent2_genes,
        fitness: None,
    };
    let (child1, child2) = single_point_crossover(parent1, parent2, 0.00);

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

    let parent1: Chromosome<GeneType, 4> = Chromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: Chromosome<GeneType, 4> = Chromosome {
        genes: parent2_genes,
        fitness: None,
    };
    let (child1, child2) = two_point_crossover(parent1, parent2, 1.00);

    assert_ne!(child1.genes, parent1_genes);
    assert_ne!(child2.genes, parent2_genes);
}
