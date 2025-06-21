mod member; // adds the member.rs file to the project
mod nn_architecture; // Import the nn_architecture module
mod snakegame;
mod point;
mod population;

use population::{Population}; //  Import the Member struct

const GENS: usize = 50;
const ITER_PER_MEMBER: usize = 10;

const POP_SIZE: usize = 20; // Population size
const BEST_N_TO_KEEP: usize = 10; // Number of best members to keep for the next generation
const CROSSOVER_N: usize = 5; // Number of crossovers to perform
const RANDOM_N_TO_ADD: usize = 5; // Number of random members to add

fn main() {
    let mut pop: Population = Population::new(POP_SIZE, Some(ITER_PER_MEMBER));
    for generation in 0..GENS {
        println!("Generation {generation}");
        pop.update_fitness();

        // Save the best member's architecture to a file

        // Create new empty population for the next generation
        let mut new_pop: Population = Population::new(POP_SIZE, Some(ITER_PER_MEMBER));

        // Get best members to the old population
        let best_members = pop.best_members(BEST_N_TO_KEEP);

        // Add parents Members
        new_pop.add_members(best_members.clone());

        // Add Crossovers Members
        new_pop.add_crossovers_members(best_members.clone(),CROSSOVER_N);
        
        // Add Random Members
        new_pop.add_random_members( RANDOM_N_TO_ADD);

        pop = new_pop; // Update the population to the new one
        
    }

    
}


