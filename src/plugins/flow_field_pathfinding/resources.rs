use bevy::prelude::*;
use std::{collections::HashMap, fmt::{self, Debug}, hash::Hash};

#[derive(Resource)]
pub struct GridCellSize{
    pub rows: usize,
    pub columns: usize,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShowGridState{
    HideGrid,
    ShowGrid
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathFindingOverlayState{
    ShowNone,
    ShowTargets,
    ShowObstacles,
    ShowProimity,
    ShowVectorField,
    ShowDensityField,
}

#[derive(Resource)]
pub struct SelectedItem(pub u32);



pub struct Grid<T> where T: Clone{
    values: Vec<T>,
    columns: usize,
    rows:usize,
}

impl<T> Grid<T> where T: Clone{
    pub fn new(columns: usize, rows: usize, starting_value: T) -> Self {
        Self { 
            values: vec![starting_value; columns * rows], 
            columns, 
            rows 
        }
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.columns {
                let index = row * self.columns + col;
                write!(f, "{} ", self.values[index])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub trait Grid2D<T>{
    fn as_vec(&self) -> &Vec<T>;
    fn as_vec_mut(&mut self) -> &mut Vec<T>;
    fn get_columns(&self) -> usize;
    fn get_rows(&self) -> usize;

    fn len(&self) -> usize {
        self.get_columns() * self.get_rows()
    }

    fn get_by_index(&self, index: usize) -> Option<&T>{
        return self.as_vec().get(index);
    }

    fn get(&self, pos: &IVec2) -> Option<&T>{
        if pos.x < 0 || pos.x >= self.get_columns() as i32|| pos.y < 0 || pos.y >= self.get_rows() as i32 {
            return None;
        }
        
        return self.get_by_index(pos.x as usize + pos.y as usize * self.get_columns());
    }

    fn set_by_index(&mut self, index: usize, value: T) -> Result<(), ()>{
        if index >= self.len(){
            return Result::Err(());
        }

        self.as_vec_mut()[index] = value;

        return Result::Ok(());
    }

    fn set(&mut self, pos: IVec2, value: T) -> Result<(), ()>{
        self.set_by_index(pos.x as usize + pos.y as usize* self.get_columns(), value)
    }
}

impl <T> Grid2D<T> for Grid<T> where T: Clone {
    fn as_vec(&self) -> &Vec<T> {
        &self.values
    }

    fn as_vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.values
    }

    fn get_columns(&self) -> usize {
        self.columns
    }

    fn get_rows(&self) -> usize {
       self.rows
    }
}

#[derive(Resource)]
pub struct Field<T> where T : Clone{
    grid: Grid<T>,
    cell_dimentions: Vec2,
    area: Rect,
}

impl<T> Field<T>
where T : Clone
{
    pub fn new(columns: usize, rows: usize, area: Rect, starting_value: T) -> Self {
        let lengths = (area.max - area.min).abs();

        let cell_length = lengths.x / columns as f32;
        let cell_height = lengths.y / rows as f32;

        let cell_dimentions = Vec2::new(cell_length, cell_height);

        let grid = Grid::new(columns, rows, starting_value);

        Self { 
            grid,
            cell_dimentions,
            area,
        }
    }

    pub fn get_grid(&self) -> &Grid<T>{
        &self.grid
    }
    
    pub fn get_grid_mut(&mut self) -> &mut Grid<T>{
        &mut self.grid
    }

    pub fn get_area(&self) -> Rect {
        self.area
    }

    pub fn get_cell_dimentions(&self) -> Vec2 {
        self.cell_dimentions
    }

    pub fn reset(&mut self, value: T){
        for i in 0..self.grid.len(){
            self.set_by_index(i, value.clone()).ok();
        }
    }

    pub fn get_cells_within(&self, search_area: Rect) -> Option<IRect>{

        let search_center = self.get_cell_at_unbound(search_area.center());
        
        let size = (search_area.size() / (self.cell_dimentions))
        .ceil()
        .as_ivec2()
        .clamp(IVec2::ZERO, IVec2::new(self.get_columns() as i32, self.get_rows() as i32));

        if size.length_squared() == 0{
            return None;
        }

        Some(IRect::from_center_size(search_center, size + IVec2::ONE))
    }

    pub fn get_coord(&self, cell: IVec2) -> Vec2{
        let mim_coord = (cell.as_vec2() - Vec2::new(self.get_columns() as f32, self.get_rows() as f32) / 2.) * self.cell_dimentions * 2.;
        
        let half_cell_offset = self.cell_dimentions / 2.;

        self.area.center() + mim_coord / 2. + half_cell_offset
    }

    pub fn get_cell(&self, pos: &Vec2) -> Option<IVec2>{

        let coords = self.get_cell_unsafe(pos);

        self.check_bounds(IVec2::new(coords.x, coords.y))
    }

    fn get_cell_unsafe(&self, pos: &Vec2) -> IVec2 {
        let relative_pos = pos - self.area.min;

        (relative_pos / self.cell_dimentions).floor().as_ivec2()
    }

    fn check_bounds(&self, pos: IVec2) -> Option<IVec2>{
        
        if pos.x < 0 || pos.x >= self.get_columns() as i32|| pos.y < 0 || pos.y >= self.get_rows() as i32 {
            return None;
        }

        Some(pos)
    }

    fn get_cell_at_unbound(&self, pos: Vec2) -> IVec2 {
        let relative_pos = pos - self.area.min;

        (relative_pos / self.cell_dimentions).floor().as_ivec2()
    }

    
}

impl<T> Grid2D<T> for Field<T> where T:Clone {
    fn as_vec(&self) -> &Vec<T> {
        self.get_grid().as_vec()
    }

    fn as_vec_mut(&mut self) -> &mut Vec<T> {
        self.get_grid_mut().as_vec_mut()
    }

    fn get_columns(&self) -> usize {
        self.get_grid().columns
    }

    fn get_rows(&self) -> usize {
        self.get_grid().rows
    }
}


pub type EntityMultiField<V> = MultiField<Entity, V>;

#[derive(Resource)]
pub struct MultiField<K, V> where V : Clone , K : Eq + Hash{
    map: HashMap<K, Field<V>>,
    columns: usize, 
    rows: usize, 
    area: Rect, 
    starting_value: V
}

impl<K, V> MultiField<K, V> where V : Clone , K : Eq + Hash{
    pub fn new(columns: usize, rows: usize, area: Rect, starting_value: V) -> Self {
        Self {  
            map: HashMap::new() , 
            columns, 
            rows, 
            area, 
            starting_value 
        }
    }

    pub fn get(&self, key: &K) -> Option<&Field<V>>{
        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut Field<V>>{
        self.map.get_mut(key)
    }

    pub fn get_all(&self, coord: &IVec2) -> Vec<(&K, Option<&V>)>{
        self.map.iter()
        .map(|(key, field)| (key, field.get(coord)))
        .collect()
    }

    pub fn ensure(&mut self, key: K) {

        if !self.map.contains_key(&key){
            self.map.insert(key, self.construct_field());
        }
    }

    pub fn remove(&mut self, key: &K){
        self.map.remove(key);
    }

    pub fn reset(&mut self, value: V){
        for (_, field) in self.map.iter_mut() {
            field.reset(value.clone());
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, Field<V>>{
        return self.map.iter();
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, K, Field<V>>{
        return self.map.iter_mut();
    }

    fn construct_field(&self) -> Field<V>{
        return Field::new(self.columns, self.rows, self.area, self.starting_value.clone());
    }

    
}