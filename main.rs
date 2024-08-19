fn main() {
    let mut state = State {
        quit: false,
        position: Point { x: 0, y: 0 },
        color: (0, 0, 0),
        message: "hello world".to_string(),
    };
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Echo(String::from("Hello world!")));
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Quit);

}
