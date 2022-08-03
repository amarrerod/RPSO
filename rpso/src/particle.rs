#[derive(Debug, Default, Clone)]
pub struct Particle {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub fitness: f64,
    pub best: Vec<f64>,
    pub best_fitness: f64,
}

impl Particle {
    fn new() -> Particle {
        Particle {
            fitness: 0.0,
            best_fitness: 0.0,
            ..Default::default()
        }
    }
}
