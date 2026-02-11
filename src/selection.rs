use crate::individual::Individual;

/// Selects the most fit individiuals in a tournament
pub fn tournament_selection<I>(
    population: &Vec<I>,
    size: usize,
    winner_count: usize,
) -> Result<Vec<I>, Box<dyn std::error::Error>>
where
    I: Individual,
{
    if size >= population.len() {
        return Err("Tournament size larger than population".into());
    }
    if winner_count > size {
        return Err("Number of winners larger than tournament size".into());
    }

    let mut selected_indexes: Vec<usize> = Vec::with_capacity(size);
    let mut contestants: Vec<&I> = Vec::with_capacity(size);
    for _ in 0..size {
        loop {
            let index = rand::random_range(0..population.len());
            if !selected_indexes.contains(&index) {
                contestants.push(&population[index]);
                selected_indexes.push(index);
                break;
            }
        }
    }
    contestants.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
    let winners: Vec<I> = contestants
        .into_iter()
        .take(winner_count)
        .cloned()
        .collect();
    Ok(winners)
}
