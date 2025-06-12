enum Direction{
    North,
    South,
    East,
    West,
}

enum Message{
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn is_quit (&self) -> bool{
        matches!(self, Message::Quit)
    }

    fn quit () -> Self {
        Message::Quit
    }
}
fn main() {

}

fn show_direction(direction: &Direction) {
    match direction {
        Direction::North => println!("You are heading North!"),
        Direction::South => println!("You are heading South!"),
        Direction::East => println!("You are heading East!"),
        Direction::West => println!("You are heading West!"),
    }
}

fn process_message(message: Message) {
    match message {
        Message::Quit => println!("Quitting..."),
        Message::Move { x, y } => println!("Moving to coordinates ({}, {})", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}