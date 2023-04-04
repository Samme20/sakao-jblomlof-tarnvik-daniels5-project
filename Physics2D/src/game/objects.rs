use graphics::types::Vec2d;

use super::traits::{Object, collisionRecord};

pub struct Rectangle{
    center: Vec2d,
    height: isize,
    width: isize,
    mass: usize,
    velocity: f64,
    potnrg: f64,
}

pub struct Circle{
    center: Vec2d,
    radius: isize,
    mass: usize,
    velocity: f64,
    potnrg: f64,
}

impl Object for Rectangle {
    fn collisions(&self, other: &Box<dyn Object>, record: Option<collisionRecord>) -> Option<super::traits::collisionRecord> {
        //Vi kommer behöva göra en if statement eller en case switch som kollar följande
        //Om other är en circel
        //Om other är en rectanglel
        //Osv
        //Då jag inte lyckas hitta någon generell formel för alla möjliga former.
        return record;
    }
    fn update(&self) {
        
    }
    fn draw(&self) {
        // TODO: Draw the rectangle. writing to demonstrate that I'm working on it.
    }
    fn getcenter(&self) -> Vec2d {
        return self.center;
    }
}

impl Object for Circle {
    fn collisions(&self, other: &Box<dyn Object>, record: Option<collisionRecord>) -> Option<collisionRecord> {
        return record;
    }
    fn update(&self) {
        
    }
    fn draw(&self) {
        
    }
    fn getcenter(&self) -> Vec2d {
        return self.center
    }
}