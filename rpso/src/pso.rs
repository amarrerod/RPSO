use crate::particle::Particle;
use rand::distributions::{Distribution, Uniform};

#[derive(Debug, Default)]
pub struct PSO {
    swarm: Vec<Particle>,
    best: Particle,
    iterations: u32,
    inertia: f64,
    lr_personal: f64,
    lr_social: f64,
    p_size: u32,
    dim: u32,
}

impl PSO {
    pub fn new(
        iterations: u32,
        inertia: f64,
        lr_personal: f64,
        lr_social: f64,
        p_size: u32,
        dim: u32,
    ) -> PSO {
        PSO {
            iterations: iterations,
            lr_personal: lr_personal,
            lr_social: lr_social,
            p_size: p_size,
            dim: dim,
            ..Default::default()
        }
    }

    fn initialize_population(&mut self) {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(-5.12..5.13);
        let speed = Uniform::from(0.0..1.01);
        self.swarm = vec![Particle::new(self.dim as usize); self.p_size as usize];
        let mut best_f = f64::MAX;

        for i in 0..self.p_size as usize {
            for j in 0..self.dim as usize {
                self.swarm[i].velocity[j] = speed.sample(&mut rng);
                self.swarm[i].position[j] = dist.sample(&mut rng);
            }
            let f: f64 = self.swarm[i].position.iter().map(|x| x * x).sum();
            self.swarm[i].fitness = f;
            // Starts at best so far
            self.swarm[i].best_fitness = f;
            self.swarm[i].best = self.swarm[i].position.clone();
            if f < best_f {
                self.best = self.swarm[i].clone();
                best_f = f;
            }
        }
    }

    fn evaluate_population(&mut self) {
        for particle in &mut self.swarm {
            particle.fitness = particle.position.iter().map(|x| x * x).sum();
        }
    }

    pub fn run(&mut self) -> Result<&Particle, ()> {
        self.initialize_population();
        for _ in 0..self.iterations {
            self.evaluate_population();
            for particle in &mut self.swarm {
                if particle.fitness < particle.best_fitness {
                    particle.best_fitness = particle.fitness;
                    particle.best = particle.position.clone();
                }
                if particle.fitness < self.best.fitness {
                    self.best = particle.clone();
                }
                // Thus, looking under the hood of a PSO
                // we find that a perturbation vector is a velocity vector v and a new velocity vector v?
                // is defined as the weighted sum of three components: v and two vector differences.
                // The first points from the current position x to the best position y the given population member ever had in the past,
                // and the second points from x to the best position z the whole population ever had.
                // Formally, we have v? = w · v + φ1U1 · (y − x)+ φ2U2 · (z − x)
                let mut rng = rand::thread_rng();
                let u = Uniform::from(0.0..1.0);
                for i in 0..particle.velocity.len() {
                    let nv_ith = self.inertia * particle.velocity[i]
                        + self.lr_personal
                            * u.sample(&mut rng)
                            * (particle.best[i] - particle.position[i])
                        + self.lr_social
                            * u.sample(&mut rng)
                            * (self.best.position[i] - particle.position[i]);
                    // updates position to save time
                    particle.velocity[i] = nv_ith;
                    particle.position[i] = particle.position[i] + nv_ith;
                }
            }
        }
        Ok(&self.best)
    }
}
