mod common_key_events;
mod error_menu;
mod help_menu;
mod home;
mod input;
mod playlist;
mod search_results;
mod select_device;
mod song_table;

use super::app::{ActiveBlock, App};
use termion::event::Key;

pub fn handle_app(app: &mut App, key: Key) {
    // Match events for different app states
    match app.active_block {
        ActiveBlock::Input => {
            input::handler(key, app);
        }
        ActiveBlock::MyPlaylists => {
            playlist::handler(key, app);
        }
        ActiveBlock::SongTable => {
            song_table::handler(key, app);
        }
        ActiveBlock::HelpMenu => {
            help_menu::handler(key, app);
        }
        ActiveBlock::Error => {
            error_menu::handler(key, app);
        }
        ActiveBlock::SelectDevice => {
            select_device::handler(key, app);
        }
        ActiveBlock::SearchResultBlock => {
            search_results::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
    }
}