use bevy::prelude::*;
use game::GamePlugin;
use loading::LoadingPlugin;
use scene::ScenePlugin;

#[cfg(debug_assertions)]
mod debug;

mod scene;
mod loading;
mod game;

fn main() {
    
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenePlugin)
        .add_plugins(GamePlugin)
        .add_plugins(LoadingPlugin)
    ;
    
    
    //This is for running the game in a debug mode
    #[cfg(debug_assertions)]
    {
        app.add_plugins(debug::DebugPlugin);
    }
    
    app.run();
}
