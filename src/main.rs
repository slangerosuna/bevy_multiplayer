use bevy::prelude::*;

mod networking;

use networking::*;

fn main () {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin 
            { max_players: 4, max_synced_objects: 1000, app_id: 480 })
        .run();
}
