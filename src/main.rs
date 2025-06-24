mod member; // adds the member.rs file to the project
mod nn_architecture; // Import the nn_architecture module
mod snakegame;
mod point;
mod population;

use population::{Population};
use member::{Member};
use std::fs::File;
use std::io::Write;
use serde_json;

const GENS: usize = 3000;
const ITER_PER_MEMBER: usize = 10;

const POP_SIZE: usize = 100; // Population size
const BEST_N_TO_KEEP: usize = 10; // Number of best members to keep for the next generation
const CROSSOVER_N: usize = 89; // Number of crossovers to perform
const RANDOM_N_TO_ADD: usize = 1; // Number of random members to add

fn main() {
    let mut pop: Population = Population::new(POP_SIZE, Some(ITER_PER_MEMBER), 0);
    for generation in 1..GENS {
        println!("Generation {generation}");
        pop.update_fitness();

        // Create new empty population for the next generation
        let mut new_pop: Population = Population::new(0, Some(ITER_PER_MEMBER), generation);

        // Get best members to the old population
        let best_members: Vec<Member> = pop.best_members(BEST_N_TO_KEEP);

        // Save the best member's architecture to a file
        //let members_to_be_saved: Vec<Member> = pop.best_members(1);
        //if generation%500==0 {
        //    let formatted_string: String = format!("best_members_{}.json", generation);
        //    let _ = save_members_to_file(&members_to_be_saved, &formatted_string);
        //}

        // Add parents Members
        new_pop.add_members(best_members.clone());

        // Add Crossovers Members
        new_pop.add_crossovers_members(best_members.clone(), CROSSOVER_N, generation);
        
        // Add Random Members
        new_pop.add_random_members( RANDOM_N_TO_ADD, generation);

        pop = new_pop; // Update the population to the new one
    }
}

fn save_members_to_file(members: &Vec<Member>, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(members).unwrap(); // or to_string() for compact
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
