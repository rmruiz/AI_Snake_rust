const DEFAULT_ITERATIONS: usize = 10;
    
use crate::member::Member;
use rand::{Rng,rng};
use rayon::prelude::*;

const MIX_TYPE_ALL_PERCENTAGE: usize = 30;
const MIX_TYPE_HALF_PERCENTAGE: usize = 60;

const MIX_WEIGHTS_PERCENTAGE: usize = 50;
const MIX_BIASES_PERCENTAGE: usize = 50;

const MIX_MUTATE_PERCENTAGE: usize = 1;

#[derive(Debug, PartialEq)]
pub enum MixType {
    All,
    //Perc(usize), // por ejemplo, 50 para 50%
    Percentage,
    Single,
}

#[derive(Debug, PartialEq)]
pub enum MixTarget {
    Weights,
    Biases,
    Both,
    Random,
}

//#[derive(Debug, Clone)]
pub struct Population {
    members: Vec<Member>,
    iterations: usize,
    killed_by_wall: usize,
    killed_by_myself: usize,
    killed_by_hunger: usize,
    apples_eaten: usize,
    average_fitness: f64
}

impl Population {

    pub fn new(size:usize, iterations:Option<usize>, generation: usize ) -> Self {
        let members: Vec<Member> = (0..size).map(|_| Member::new(None, None, None, generation)).collect();

        Population { 
            members: members,
            iterations: iterations.unwrap_or(DEFAULT_ITERATIONS),
            killed_by_wall: 0,
            killed_by_myself: 0,
            killed_by_hunger: 0,
            apples_eaten: 0,
            average_fitness: 0.0,
        }
    }

    pub fn add_members(&mut self, members: Vec<Member>) {
        self.members.extend(members);
    }

    pub fn add_random_members(&mut self, quantity: usize, generation: usize) {
        let new_members: Vec<Member> = (0..quantity)
            .map(|_| Member::new(None, None, None, generation))
            .collect();
        self.members.extend(new_members);
    }

    pub fn best_members(&self, quantity: usize) -> Vec<Member> {
        let mut sorted_members = self.members.clone();
        sorted_members.sort_by(|a, b| {
            b.fitness
                .partial_cmp(&a.fitness)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted_members.into_iter().take(quantity).collect()
    }

    pub fn select_proportional_by_fitness(members: &[Member]) -> Member {
        let total_fitness: f64 = members.iter().map(|m| m.fitness).sum();

        // Genera un número aleatorio entre 0 y total_fitness (exclusivo)
        let mut random_fitness_wheel: f64 = rand::rng().random_range(0.0..total_fitness);

        for member in members {
            if random_fitness_wheel < member.fitness {
                return member.clone(); // Asume que Member implementa Clone
            }
            random_fitness_wheel -= member.fitness;
        }

        panic!("No se pudo seleccionar un miembro proporcionalmente. Verifica los datos de entrada.");
    }

    pub fn add_crossovers_members(&mut self, best_members:Vec<Member>, quantity: usize, generation: usize) {
        let mut rng = rand::rng();

        let mut new_members: Vec<Member> = Vec::with_capacity(quantity);

        for _ in 0..quantity {
            let roll: usize = rng.random_range(0..100);
            let mix_type: MixType = 
                if roll < MIX_TYPE_ALL_PERCENTAGE {
                    MixType::All
                } else if roll < MIX_TYPE_HALF_PERCENTAGE {
                    MixType::Percentage
                } else {
                    MixType::Single
                };
            
            let rollw: usize = rng.random_range(0..100);
            let rollb: usize = rng.random_range(0..100);
            let mix_target: MixTarget = 
                if rollw < MIX_WEIGHTS_PERCENTAGE && rollb < MIX_BIASES_PERCENTAGE {
                    MixTarget::Both
                } else if rollw < MIX_WEIGHTS_PERCENTAGE{    
                    MixTarget::Weights
                }else if rollb < MIX_BIASES_PERCENTAGE {    
                    MixTarget::Biases
                } else {
                    MixTarget::Random
                };
            
            let mutate: bool = rng.random_bool(MIX_MUTATE_PERCENTAGE as f64 / 100.0);

            // Selecciona dos miembros aleatorios
            let mem1: Member = Self::select_proportional_by_fitness(&best_members);
            let mem2: Member = Self::select_proportional_by_fitness(&best_members);

            // Crea un nuevo miembro cruzando los dos seleccionados
            let new_member = Population::cross_members(&mem1, &mem2, mix_type, mix_target, mutate, generation);
            new_members.push(new_member);
        }

        self.add_members(new_members);
    }

    pub fn cross_members(
        mem1: &Member,
        mem2: &Member,
        mix_type: MixType,
        mix_target: MixTarget,
        mutate: bool,
        generation: usize
    ) -> Member {
        let mut rng = rng();

        let mut new_mem = Member::new(Some(mem1.weights.clone()), Some(mem1.biases.clone()), None, generation);

        // Determinar si se cambian pesos y/o biases
        let (change_weights, change_biases) = match mix_target {
            MixTarget::Weights => (true, false),
            MixTarget::Biases => (false, true),
            MixTarget::Both => (true, true),
            MixTarget::Random => (rng.random_bool(0.5), rng.random_bool(0.5)),
        };

        match mix_type {
            MixType::All | MixType::Percentage => {
                let perc: usize = match mix_type {
                    MixType::All => 100,
                    MixType::Percentage => 50,
                    MixType::Single => 50 //not reached
                };

                if change_weights {
                    for i in 0..new_mem.weights.len() {
                        for j in 0..new_mem.weights[i].nrows() {
                            for k in 0..new_mem.weights[i].ncols() {
                                if rng.random_range(1..=100) <= perc {
                                    new_mem.weights[i][[j, k]] = mem2.weights[i][[j, k]];
                                }
                            }
                        }
                    }
                }

                if change_biases {
                    for i in 0..new_mem.biases.len() {
                        for j in 0..new_mem.biases[i].nrows() {
                            for k in 0..new_mem.biases[i].ncols() {
                                if rng.random_range(1..=100) <= perc {
                                    new_mem.biases[i][[j, k]] = mem2.biases[i][[j, k]];
                                }
                            }
                        }
                    }
                }
            }

            MixType::Single => {
                if change_weights {
                    let i = rng.random_range(0..new_mem.weights.len());
                    let j = rng.random_range(0..new_mem.weights[i].nrows());
                    let k = rng.random_range(0..new_mem.weights[i].ncols());
                    new_mem.weights[i][[j, k]] = mem2.weights[i][[j, k]];
                }

                if change_biases {
                    let i = rng.random_range(0..new_mem.biases.len());
                    let j = rng.random_range(0..new_mem.biases[i].nrows());
                    let k = rng.random_range(0..new_mem.biases[i].ncols());
                    new_mem.biases[i][[j, k]] = mem2.biases[i][[j, k]];
                }
            }
        }

        // Mutación
        if mutate {
            if change_weights {
                let i = rng.random_range(0..new_mem.weights.len());
                let j = rng.random_range(0..new_mem.weights[i].nrows());
                let k = rng.random_range(0..new_mem.weights[i].ncols());
                new_mem.weights[i][[j, k]] = rng.random_range(-1.0..1.0);
            }

            if change_biases {
                let i = rng.random_range(0..new_mem.biases.len());
                let j = rng.random_range(0..new_mem.biases[i].nrows());
                let k = rng.random_range(0..new_mem.biases[i].ncols());
                new_mem.biases[i][[j, k]] = rng.random_range(-1.0..1.0);
            }
        }

        new_mem
    }

    pub fn update_fitness(&mut self) {
        // reset stats
        self.killed_by_wall = 0;
        self.killed_by_myself = 0;
        self.killed_by_hunger = 0;
        self.apples_eaten = 0;
        self.average_fitness = 0.0;

        // Use par_iter_mut and collect intermediate results
        let stats: Vec<(usize, usize, usize, usize, f64)> = self
            .members
            .par_iter_mut() // PARALEL .par_iter_mut(), NOT PARALEL .iter_mut()
            .map(|member| {
                member.iterate_to_update_fitness(self.iterations);
                (
                    member.killed_by_wall,
                    member.killed_by_myself,
                    member.killed_by_hunger,
                    member.apples_eaten,
                    member.fitness, //average score through iterations
                )
            })
            .collect();

        // Aggregate all stats after parallel work
        let mut total_fitness = 0.0;
        let mut max_fitness = 0.0;
        for (wall, myself, hunger, apples, fitness) in stats {
            self.killed_by_wall += wall;
            self.killed_by_myself += myself;
            self.killed_by_hunger += hunger;
            self.apples_eaten += apples;
            total_fitness += fitness;
            if fitness > max_fitness {
                max_fitness = fitness;
            }
        }

        self.average_fitness = total_fitness / self.members.len() as f64;

        println!(
            "[Population] max(Fit): {:.0}, avg(Fit): {:.0}",
            max_fitness,
            self.average_fitness,
        );
    }

    /*
    pub fn update_fitness(&mut self) {
        // reset stats
        self.killed_by_wall = 0;
        self.killed_by_myself = 0;
        self.killed_by_hunger = 0;
        self.apples_eaten = 0;
        self.average_fitness = 0.0;

        let mut total_fitness: f64 = 0.0;

        for member in self.members.iter_mut() {
            member.iterate_to_update_fitness(self.iterations);
            
            self.killed_by_wall += member.killed_by_wall;
            self.killed_by_myself += member.killed_by_myself;
            self.killed_by_hunger += member.killed_by_hunger;
            self.apples_eaten += member.apples_eaten;
            total_fitness += member.fitness;
        }
        self.average_fitness = total_fitness / self.members.len() as f64;
        
        println!("[Population] avg(Fitness): {:.3}, K by wall: {}, K by myself: {}, K by hunger: {}, Apples eaten: {}",
            self.average_fitness, 
            self.killed_by_wall, 
            self.killed_by_myself, 
            self.killed_by_hunger, 
            self.apples_eaten);
    }
    */
}
    
#[cfg(test)]
mod tests {
    use super::*;
    use crate::member::Member;
    //use ndarray::Array2;

    fn generate_random_u8_32() -> [u8; 32] {
        let mut rng = rand::rng();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        bytes
    }

    #[test]
    fn test_population_new_with_default_iterations() {
        let pop = Population::new(5, None, 0);
        assert_eq!(pop.members.len(), 5);
        assert_eq!(pop.iterations, DEFAULT_ITERATIONS);
    }

    #[test]
    fn test_population_new_with_custom_iterations() {
        let pop = Population::new(3, Some(42), 0);
        assert_eq!(pop.members.len(), 3);
        assert_eq!(pop.iterations, 42);
    }

    #[test]
    fn test_add_members() {
        let mut pop = Population::new(2, None, 0);

        let extra_members = vec![
            Member::new(None, None, Some(generate_random_u8_32()), 0),
            Member::new(None, None, Some(generate_random_u8_32()), 0),
        ];

        pop.add_members(extra_members);
        assert_eq!(pop.members.len(), 4);
    }

    #[test]
    fn test_add_random_members() {
        let mut pop = Population::new(1, None, 0);
        pop.add_random_members(3, 0);
        assert_eq!(pop.members.len(), 4);
    }

    #[test]
    fn test_best_members_sorted_by_fitness() {
        let mut pop = Population::new(0, None, 0);

        let mut m1 = Member::new(None, None, Some(generate_random_u8_32()), 0);
        m1.fitness = 10.0;

        let mut m2 = Member::new(None, None, Some(generate_random_u8_32()), 0);
        m2.fitness = 50.0;

        let mut m3 = Member::new(None, None, Some(generate_random_u8_32()), 0);
        m3.fitness = 30.0;

        pop.add_members(vec![m1, m2, m3]);
        let best = pop.best_members(2);

        assert_eq!(best.len(), 2);
        assert_eq!(best[0].fitness, 50.0);
        assert_eq!(best[1].fitness, 30.0);
    }

    #[test]
    fn test_best_members_limited_by_quantity() {
        let mut pop = Population::new(0, None, 0);

        for i in 0..10 {
            let mut m = Member::new(None, None, Some(generate_random_u8_32()), 0);
            m.fitness = i as f64;
            pop.members.push(m);
        }

        let best = pop.best_members(5);
        assert_eq!(best.len(), 5);
        assert_eq!(best[0].fitness, 9.0);
        assert_eq!(best[4].fitness, 5.0);
    }

    fn generate_dummy_member(seed: [u8; 32]) -> Member {
        Member::new(None, None, Some(seed), 0)
    }

    #[test]
    fn test_cross_all_weights() {
        let mem1 = generate_dummy_member([1; 32]);
        let mem2 = generate_dummy_member([2; 32]);

        let child = Population::cross_members(&mem1, &mem2, MixType::All, MixTarget::Weights, false, 0);

        // Should be mostly equal to mem2 in weights, and equal to mem1 in biases
        assert_ne!(child.weights, mem1.weights);
        assert_eq!(child.biases, mem1.biases);
    }

    #[test]
    fn test_cross_single_biases() {
        let mem1 = generate_dummy_member([3; 32]);
        let mem2 = generate_dummy_member([4; 32]);

        let child = Population::cross_members(&mem1, &mem2, MixType::Single, MixTarget::Biases, false, 0);

        assert_eq!(child.weights, mem1.weights); // weights unchanged
        assert_ne!(child.biases, mem1.biases); // at least one bias changed
    }

    #[test]
    fn test_cross_both_with_mutation() {
        let mem1 = generate_dummy_member([5; 32]);
        let mem2 = generate_dummy_member([6; 32]);

        let child = Population::cross_members(&mem1, &mem2, MixType::All, MixTarget::Both, true, 0);

        assert_ne!(child.weights, mem1.weights);
        assert_ne!(child.biases, mem1.biases);

        assert_ne!(child.weights, mem2.weights);
        assert_ne!(child.biases, mem2.biases);
    }

}
