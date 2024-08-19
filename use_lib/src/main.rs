extern crate test_lib;

/*
struct Vec2 {
   x: f64,
   y: f64,
}
*/

fn main() {

  /*  let mut a_vec: Vec2 = Vec2 { 
        x: 2.0, 
        y: 1.8 
    };
    a_vec.y = 2.5;
    let new_vec = Vec2{ x: 1.0, y: a_vec.y };
    println!{ "The position is x = {} and y = {}", new_vec.x, new_vec.y};

*/
    let mut state = test_lib::State {
        quit: false,
        position: test_lib::Point ( 0, 0 ),
        color: (0, 0, 0),
        message: "hello world".to_string(),
    };
    state.process(test_lib::Message::ChangeColor(255, 0, 255));
    state.process(test_lib::Message::Echo(String::from("Hello world!")));
    state.process(test_lib::Message::Move(test_lib::Point( 10, 15 )));
    state.process(test_lib::Message::Quit);

}
