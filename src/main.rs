use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup_system)
        .add_system(rotate_triangle)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component)]
struct Enemy;

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::CYAN)),
            Transform::default().with_rotation(Quat::from_axis_angle(Vec3::Z, 1.0)),
        ))
        .insert(Enemy);
}

fn rotate_triangle(time: Res<Time>, mut query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(Vec3::Z, time.delta_seconds()));
    }
}
