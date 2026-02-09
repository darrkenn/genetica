use genetica::{
    crossover::{dynamic_length_single_point_crossover, dynamic_length_two_point_crossover},
    individual::{DynamicLengthIndividual, Generate, Individual, Mutate},
    population::generate_population,
    selection::tournament_selection,
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

#[derive(Debug, Clone)]
struct DynamicLengthChromosome {
    genes: Vec<GeneType>,
    fitness: Option<f32>,
}

impl Individual for DynamicLengthChromosome {
    type GeneType = GeneType;

    fn new() -> Self {
        let genes: Vec<GeneType> = (0..4).map(|_| GeneType::generate()).collect();
        DynamicLengthChromosome {
            genes,
            fitness: None,
        }
    }
    fn mutate_genes(&mut self) {
        for gene in self.genes_mut() {
            gene.mutate();
        }
    }

    fn fitness(&self) -> Option<f32> {
        self.fitness
    }
    fn fitness_mut(&mut self) -> &mut Option<f32> {
        &mut self.fitness
    }
    fn calculate_fitness(&mut self) {
        self.fitness = Some(0.00);
    }
}

impl DynamicLengthIndividual for DynamicLengthChromosome {
    fn genes(&self) -> &Vec<GeneType> {
        &self.genes
    }
    fn genes_mut(&mut self) -> &mut Vec<GeneType> {
        &mut self.genes
    }
}

#[test]
fn test_dynamic_single_point_crossover_success() {
    let parent1_genes: Vec<GeneType> = (0..4).map(|_| GeneType(true)).collect();
    let parent2_genes: Vec<GeneType> = (0..4).map(|_| GeneType(false)).collect();

    let parent1: DynamicLengthChromosome = DynamicLengthChromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: DynamicLengthChromosome = DynamicLengthChromosome {
        genes: parent2_genes,
        fitness: None,
    };

    let (child1, child2) = dynamic_length_single_point_crossover(&parent1, &parent2, 1.00);

    assert_ne!(child1.genes, parent1.genes);
    assert_ne!(child2.genes, parent2.genes);
}

#[test]
fn test_dynamic_single_point_crossover_no_probability() {
    let parent1_genes: Vec<GeneType> = (0..4).map(|_| GeneType(true)).collect();
    let parent2_genes: Vec<GeneType> = (0..4).map(|_| GeneType(false)).collect();

    let parent1: DynamicLengthChromosome = DynamicLengthChromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: DynamicLengthChromosome = DynamicLengthChromosome {
        genes: parent2_genes,
        fitness: None,
    };

    let (child1, child2) = dynamic_length_single_point_crossover(&parent1, &parent2, 0.00);

    assert_eq!(child1.genes, parent1.genes);
    assert_eq!(child2.genes, parent2.genes);
}

#[test]
fn test_dynamic_two_point_crossover_success() {
    let parent1_genes: Vec<GeneType> = (0..4).map(|_| GeneType(true)).collect();
    let parent2_genes: Vec<GeneType> = (0..4).map(|_| GeneType(false)).collect();

    let parent1: DynamicLengthChromosome = DynamicLengthChromosome {
        genes: parent1_genes,
        fitness: None,
    };
    let parent2: DynamicLengthChromosome = DynamicLengthChromosome {
        genes: parent2_genes,
        fitness: None,
    };
    let (child1, child2) = dynamic_length_two_point_crossover(&parent1, &parent2, 1.00);

    assert_ne!(child1.genes, parent1.genes);
    assert_ne!(child2.genes, parent2.genes);
}

#[test]
fn test_tournament_selection_success() {
    let population: Vec<DynamicLengthChromosome> = generate_population(8);
    let winners = match tournament_selection(&population, 4, 3) {
        Ok(winners) => winners,
        Err(e) => {
            panic!("{e}")
        }
    };
    assert_eq!(3, winners.len());
}

#[test]
fn test_tournament_selection_invalid_size() {
    let population: Vec<DynamicLengthChromosome> = generate_population(8);
    let _ = match tournament_selection(&population, 1000, 3) {
        Ok(_) => {
            panic!("Function returned ok when it should have errored out")
        }
        Err(e) => {
            assert_eq!("Tournament size larger than population", e.to_string());
        }
    };
}

#[test]
fn test_tournament_selection_invalid_winner_count() {
    let population: Vec<DynamicLengthChromosome> = generate_population(8);
    let _ = match tournament_selection(&population, 4, 11111) {
        Ok(_) => {
            panic!("Function return ok when it shoud have errored out");
        }
        Err(e) => {
            assert_eq!(
                "Number of winners larger than tournament size",
                e.to_string()
            );
        }
    };
}
