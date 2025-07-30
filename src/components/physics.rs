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

#[derive(Component, Clone, Copy, Default, From, Into, Add, Mul, AddAssign, MulAssign, Constructor, PartialEq)]
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

pub fn point_in_shape(shape: &Shape, shape_position: Vec2, point: Vec2) -> bool {
    match shape {
        Shape::Circle(radius) => (point - shape_position).length() <= *radius,
        Shape::Polygon(polygon_points) => {

            let point = point - shape_position;
            let mut inside = false;
            let mut j = polygon_points.len() - 1;

            for i in 0..polygon_points.len() {
                let pi = polygon_points[i];
                let pj = polygon_points[j];

                if (pi.y > point.y) != (pj.y > point.y)
                    && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
                {
                    inside = !inside;
                }
                j = i;
            }

            inside
        },
    }
}


pub fn signed_distance_and_normal_to_sahpe(shape: &Shape, shape_position: Vec2, point: Vec2) -> (Vec2, f32) {
    match shape {
        Shape::Circle(radius) => ( point - shape_position, (point - shape_position).length() - radius),
        Shape::Polygon(polygon_points) => {
        
            let point = point - shape_position;

            let num_points = polygon_points.len();
            let mut min_dist: f32 = f32::INFINITY;
            let mut segment = (Vec2::ZERO, Vec2::ZERO);
            let mut t_of_min = 0.;

            for i in 0..num_points{

                let a = polygon_points[i];
                let b = polygon_points[(i + 1) % num_points];

                let t = (point - a).dot(b - a) / (b - a).dot( b - a);

                let closes_point = a.lerp(b, t.clamp(0., 1.));
                
                let distance = (point - closes_point).length();

                if distance < min_dist{
                    min_dist = distance;
                    segment = (a, b);
                    t_of_min = t;
                }
            }

            let ab = segment.1 - segment.0;
            // Vec2::new(ab.y, -ab.x)
            let normal = match t_of_min {
                x if x < 0. => point - segment.0,
                x if x > 1. => point - segment.1,
                _ => Vec2::new(ab.y, -ab.x)
            };

            return (normal, min_dist);
        }
    }
}

pub trait Coordinate : Sized + Copy{
    fn adjacent(&self) -> Vec<Self>;

    fn adjacent_inclusive(&self) -> Vec<Self> {
        let mut adjacent = self.adjacent();
        adjacent.push(*self);
        adjacent
    }
}

impl Coordinate for IVec2 {
    fn adjacent(&self) -> Vec<Self> {
        let directions = [
        IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1), // Top row (northwest, north, northeast)
        IVec2::new(-1,  0),                        IVec2::new(1,  0),  // Left, Right
        IVec2::new(-1,  1), IVec2::new(0,  1), IVec2::new(1,  1), // Bottom row (southwest, south, southeast)
    ];

    directions.iter()
        .map(|&dir| self + dir)
        .collect()
    }
}