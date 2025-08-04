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

pub fn draw_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid_size: Res<GridSize>,
    windows: Query<&Window>,
) {
    commands.spawn(Camera2d);
    for window in windows {
        draw_grid_lines(
            &mut commands,
            &mut meshes,
            &mut materials,
            *grid_size,
            Dimensions {
                width: window.width(),
                height: window.height(),
            },
        )
    }
}

fn draw_grid_lines(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    size: GridSize,
    available_space: Dimensions,
) {
    let cell_size = get_cell_size(size, available_space);
    draw_grid_lines_of_axis(commands, meshes, materials, cell_size, size, Axis::X);
    draw_grid_lines_of_axis(commands, meshes, materials, cell_size, size, Axis::Y);
}

fn get_cell_size(grid_size: GridSize, available_space: Dimensions) -> f32 {
    let dimension_cell_size = |axis: Axis| -> f32 {
        let dimension_cells = grid_size.get(axis);
        let spaces = dimension_cells - 1;
        available_space.get(axis) / dimension_cells as f32 - GRID_STROKE * spaces as f32
    };
    dimension_cell_size(Axis::X).min(dimension_cell_size(Axis::Y))
}

fn draw_grid_lines_of_axis(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    cell_size: f32,
    grid_size: GridSize,
    orientation: Axis,
) {
    let length = cell_size * grid_size.get(orientation) as f32;
    let offset = cell_size * grid_size.get(orientation.reversed()) as f32 / 2.0;
    for cell in 1..grid_size.get(orientation.reversed()) {
        draw_grid_line(
            commands,
            meshes,
            materials,
            orientation,
            length,
            cell_size * cell as f32 - offset,
        );
    }
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
