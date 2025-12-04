#![allow(dead_code)]

use std::array;

/// A trait which allows a value to mutate itself in place
pub trait Mutate: Sized {
    fn mutate(&mut self);
}

/// A trait which allows a value to be generated
pub trait Generate: Sized {
    fn generate() -> Self;
}

/// A struct that holds a set of genes and a fitness value.
#[derive(Debug, Clone, Copy)]
pub struct Chromosome<GeneType: Mutate + Generate, const GENESIZE: usize> {
    pub genes: [GeneType; GENESIZE],
    pub fitness: Option<i32>,
}

impl<GeneType: Mutate + Generate, const GENESIZE: usize> Chromosome<GeneType, GENESIZE> {
    /// Creates a new chromosome with random genes
    pub fn new() -> Self {
        let genes = array::from_fn(|_| GeneType::generate());
        Chromosome {
            genes,
            fitness: None,
        }
    }

    /// Mutates all genes in a chromosome usin
    pub fn mutate_genes(&mut self) {
        for gene in &mut self.genes {
            gene.mutate();
        }
    }
}
