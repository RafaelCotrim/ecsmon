use std::ops::AddAssign;

use bevy::prelude::*;
use derive_more::{Add, AddAssign, Constructor, From, Into, Mul, MulAssign};

#[derive(Component, Clone, Copy, Default, From, Into, Add, Mul, AddAssign, MulAssign, Constructor)]
pub struct Speed(Vec2);


impl Speed {
    
    pub fn value(&self) -> Vec2 {
        self.0
    }

    pub fn set_value(&mut self, value: Vec2){
        self.0 = value;
    }
}

#[derive(Component, Clone, Copy, Default, From, Into, Add, Mul, AddAssign, MulAssign, Constructor)]
pub struct Position(Vec2);

impl Position {
    
    pub fn value(&self) -> Vec2 {
        self.0
    }

    pub fn set_value(&mut self, value: Vec2){
        self.0 = value;
    }
}

#[derive(Component)]
pub enum Shape {
    Circle(f32),

    // Points have to be counterclockwise
    Polygon(Vec<Vec2>)
}

impl Shape {
    pub fn get_rectangle_with_center(&self, center: Vec2) -> Rect{
        match self {
            Shape::Circle(r) => Rect::from_center_half_size(center, Vec2::new(*r, *r)),
            Shape::Polygon(points) => {
                let max = points.iter().fold(Vec2::NEG_INFINITY, |acc, x| x.max(acc));
                let min = points.iter().fold(Vec2::INFINITY, |acc, x| x.min(acc));

                Rect::from_corners(min + center, max + center)
            },
        }
    }
}