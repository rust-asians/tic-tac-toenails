use crate::dimension::Axis;
use crate::dimension::Dimensions;
use bevy::prelude::*;
use domain::grid::GridSize as DomainGridSize;

#[derive(Resource, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct GridSize {
    pub width: u8,
    pub height: u8,
}

impl GridSize {
    fn get(&self, axis: Axis) -> u8 {
        match axis {
            Axis::X => self.width,
            Axis::Y => self.height,
        }
    }
}

impl DomainGridSize for GridSize {
    fn width(&self) -> u8 {
        self.width
    }

    fn height(&self) -> u8 {
        self.height
    }
}

const GRID_COLOR: Color = Color::hsl(0.0, 0.0, 0.5);
const GRID_STROKE: f32 = 10.0;

#[derive(Component)]
struct GridLine;

fn draw_grid(size: GridSize, available_space: Dimensions) {
    let cell_size = get_cell_size(size, available_space);
}

fn get_cell_size(grid_size: GridSize, available_space: Dimensions) -> f32 {
    let dimension_cell_size = |axis: Axis| -> f32 {
        let dimension_cells = grid_size.get(axis);
        let spaces = dimension_cells - 1;
        available_space.get(axis) / dimension_cells as f32 - GRID_STROKE * spaces as f32
    };
    dimension_cell_size(Axis::X).min(dimension_cell_size(Axis::Y))
}

fn draw_grid_line(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orientation: Axis,
    length: f32,
    offset: f32,
) {
    let grid_line_material = materials.add(GRID_COLOR);
    let vertical_line = meshes.add(create_grid_line_rectangle(orientation, length));
    commands.spawn((
        GridLine,
        Mesh2d(vertical_line),
        MeshMaterial2d(grid_line_material),
        get_transform(orientation, offset),
    ));
}

fn create_grid_line_rectangle(orientation: Axis, length: f32) -> Rectangle {
    match orientation {
        Axis::X => Rectangle::new(length, GRID_STROKE),
        Axis::Y => Rectangle::new(GRID_STROKE, length),
    }
}

fn get_transform(orientation: Axis, offset: f32) -> Transform {
    match orientation {
        Axis::X => Transform::from_xyz(0.0, offset, 0.0),
        Axis::Y => Transform::from_xyz(offset, 0.0, 0.0),
    }
}
