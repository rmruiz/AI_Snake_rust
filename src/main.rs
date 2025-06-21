mod member; // adds the member.rs file to the project
mod nn_architecture; // Import the nn_architecture module
mod snakegame;
mod point;
mod population;

use population::{Population};
use member::{Member};

const GENS: usize = 800;
const ITER_PER_MEMBER: usize = 10;

const POP_SIZE: usize = 100; // Population size
const BEST_N_TO_KEEP: usize = 50; // Number of best members to keep for the next generation
const CROSSOVER_N: usize = 30; // Number of crossovers to perform
const RANDOM_N_TO_ADD: usize = 20; // Number of random members to add

fn main() {
    let mut pop: Population = Population::new(POP_SIZE, Some(ITER_PER_MEMBER), 0);
    for generation in 1..GENS {
        println!("Generation {generation}");
        pop.update_fitness();

        // Save the best member's architecture to a file
        //TODO: save to disk

        // Create new empty population for the next generation
        let mut new_pop: Population = Population::new(0, Some(ITER_PER_MEMBER), generation);

        // Get best members to the old population
        let best_members: Vec<Member> = pop.best_members(BEST_N_TO_KEEP);

        // Add parents Members
        new_pop.add_members(best_members.clone());

        // Add Crossovers Members
        new_pop.add_crossovers_members(best_members.clone(), CROSSOVER_N, generation);
        
        // Add Random Members
        new_pop.add_random_members( RANDOM_N_TO_ADD, generation);

        pop = new_pop; // Update the population to the new one
    }
}


