use player::Player;
use shape::Shape;

mod shape;
mod player;
mod game;
mod utils;

fn main() {
    let game_amount = utils::num_parse(&utils::get_input());
    game::game_loop(game_amount);
}
