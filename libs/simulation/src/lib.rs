use nalgebra as na;
use rand::{Rng,RngCore};
pub struct Simulation {
    world: World,
}
pub struct World{
    animals: Vec<Animal>,
    foods: Vec<Food>,  
}
pub struct Animal{
    position:na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}
pub struct Food{
    position:na::Point2<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self{
        Self{
        world: World::random(rng),
        }
    }
}
impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self{
}