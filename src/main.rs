use bevy::{
    core_pipeline::clear_color::ClearColorConfig::Custom,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

/**
 * Implementing base units as 10*GameUnits
 */
static GUM: f32 = 10.;

pub struct MVP0;
impl Plugin for MVP0 {
    fn build(&self, app: &mut App) {
        app.add_startup_system(MVP0::startup);
    }
}
impl MVP0 {
    fn startup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        // Create camera bundle defaults
        let mut camera = Camera2dBundle::default();
        // camera.camera.viewport = Some(bevy::render::camera::Viewport {
        //     physical_position: UVec2::new(0, 0),
        //     physical_size: UVec2::new(500, 500),
        //     depth: (0.)..(1.),
        // });
        // Set clear color to Black
        camera.camera_2d.clear_color = Custom(Color::BLACK);
        commands.spawn_bundle(camera);

        // Spawn Base

        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(5., 5.) * GUM)))
                .into(),
            transform: Transform::default(),
            material: materials.add(ColorMaterial::from(Color::TOMATO)),
            ..default()
        });

        // Spawn Soldier
        // let mut shape = shape::Quad::new(Vec2::new(1., 1.) * GUM);

        fn create_triangle() -> Mesh {
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [1.0, 1.0, 0.0]],
            );
						mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[-1.0, -1.0, 0.0], [-1.0, -1.0, 0.0], [-1.0, -1.0, 0.0]]);
						mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]]);
            mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
            mesh
        }

        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(create_triangle()).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(10., 0., 0.))
                .with_scale(Vec3::splat(128. / 5.)),
            material: materials.add(ColorMaterial::from(Color::TEAL)),
            ..default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MVP0)
        // .add_plugin(HelloPlugin)
        .run();
}

// People Hello World!
// pub struct HelloPlugin;
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//             .add_startup_system(add_people)
//             .add_system(greet_people);
//     }
// }

// #[derive(Component)]
// struct Person;

// #[derive(Component)]
// struct Name(String);

// fn add_people(mut commands: Commands) {
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Ruben".to_string()));
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Dani".to_string()));
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Davi".to_string()));
// }

// struct GreetTimer(Timer);

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in query.iter() {
//             println!("hello {}!", name.0);
//         }
//     }
// }
