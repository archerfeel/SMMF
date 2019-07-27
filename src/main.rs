pub mod ai;
pub mod engine;

fn main() {

    let mut war = engine::War::new();
    war.display();
    let mut ghost = ai::Ghost::new(&war);
}
