use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (create_dummy_grid, setup))
        .add_systems(Startup, (create_dummy_grid, draw_grid))
        .add_systems(Update, greet_marks)
        .run();
}

#[derive(Component)]
struct Mark {
    id: u8,
}

#[derive(Component)]
struct Position {
    x: u8,
    y: u8,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let shape = meshes.add(Rectangle::new(50.0, 50.0));
    commands.spawn((Mesh2d(shape), MeshMaterial2d(materials.add(Color::hsl(360f32, 0.95, 0.7)))));
}

fn create_dummy_grid(mut commands: Commands) {
    commands.spawn((Mark { id:1 }, Position { x: 0, y: 0 }));
    commands.spawn((Mark { id:2 }, Position { x: 0, y: 1 }));
    commands.spawn((Mark { id:2 }, Position { x: 0, y: 2 }));

    commands.spawn((Mark { id:2 }, Position { x: 1, y: 0 }));
    commands.spawn((Mark { id:1 }, Position { x: 1, y: 1 }));
    commands.spawn((Mark { id:1 }, Position { x: 1, y: 2 }));

    commands.spawn((Mark { id:1 }, Position { x: 2, y: 0 }));
    commands.spawn((Mark { id:2 }, Position { x: 2, y: 1 }));
    commands.spawn((Mark { id:1 }, Position { x: 2, y: 2 }));
}

fn draw_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid_line_material = materials.add(Color::hsl(360.0, 0.95, 0.7));
    let vertical_line = meshes.add(Rectangle::new(10.0, 200.0));
    commands.spawn((
        Mesh2d(vertical_line),
        MeshMaterial2d(grid_line_material),
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));
}

fn greet_marks(query: Query<(&Mark, &Position)>) {
    for (mark, position) in query {
        println!("{} ({}, {})", mark.id, position.x, position.y);
    }
}
