mod particle;
mod pso;

pub use particle::*;

fn main() {
    let mut pso = pso::PSO::new(10000, 0.05, 0.25, 0.75, 100, 10);
    let solution = pso.run();
    println!("Best solution found: {:#?}", solution);
}
