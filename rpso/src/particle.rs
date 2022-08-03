#[derive(Debug, Default, Clone)]
pub struct Particle {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub fitness: f64,
    pub best: Vec<f64>,
    pub best_fitness: f64,
}

impl Particle {
    pub fn new(dim: usize) -> Particle {
        Particle {
            fitness: 0.0,
            best_fitness: 0.0,
            best: vec![0.0; dim],
            position: vec![0.0; dim],
            velocity: vec![0.0; dim],
        }
    }
}
