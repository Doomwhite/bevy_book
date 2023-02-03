use bevy::{ecs::system::Commands, prelude::*};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}

fn hello_world() {
    println!("Hello World!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaine".to_string())));
    commands.spawn((Person, Name("Renzo".to_string())));
    commands.spawn((Person, Name("Zayna".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
