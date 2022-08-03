use crate::particle::Particle;

#[derive(Debug, Default)]
pub struct PSO {
    swarm: Vec<Particle>,
    best: Particle,
    iterations: u32,
    inertia: f64,
    lr_personal: f64,
    lr_social: f64,
}

impl PSO {
    pub fn new(iterations: u32, inertia: f64, lr_personal: f64, lr_social: f64) -> PSO {
        PSO {
            iterations: iterations,
            lr_personal: lr_personal,
            lr_social: lr_social,
            ..Default::default()
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        for _ in 0..self.iterations {
            for particle in &mut self.swarm {
                if particle.fitness > particle.best_fitness {
                    particle.best_fitness = particle.fitness;
                    particle.best = particle.position.clone();
                }
                if particle.fitness > self.best.fitness {
                    self.best = particle.clone();
                }
                // Thus, looking under the hood of a PSO
                // we find that a perturbation vector is a velocity vector v and a new velocity vector v?
                // is defined as the weighted sum of three components: v and two vector differences.
                // The first points from the current position x to the best position y the given population member ever had in the past,
                // and the second points from x to the best position z the whole population ever had.
                // Formally, we have v? = w · v + φ1U1 · (y − x)+ φ2U2 · (z − x)
                for i in 0..particle.velocity.len() {
                    let nv_ith = self.inertia * particle.velocity[i]
                        + self.lr_personal * 0.05 * (particle.best[i] - particle.position[i])
                        + self.lr_social * 0.05 * (self.best.position[i] - particle.position[i]);
                    // updates position to save time
                    particle.velocity[i] = nv_ith;
                    particle.position[i] = particle.position[i] + nv_ith;
                }
            }
        }
        Ok(())
    }
}
