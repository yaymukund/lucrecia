mod library;
mod player;
mod settings;
mod ui;
mod util;

use library::Library;
use mpv::Mpv;
use player::{Player, PlayerEventHandler as _};
use settings::Settings;
use ui::create_application;

fn main() {
    let settings = Settings::new();
    let library = Library::from_path(settings.metadata_path());

    let song_path = "http://localhost:3000/song.mp3";
    let mpv = Mpv::new().unwrap();
    let mut player = Player::new(&mpv);
    let mut app = create_application(&player);

    player.play(song_path);

    while app.is_running() {
        player.poll_events();
        app.step();
    }
}
