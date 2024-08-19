pub enum Message {
    ChangeColor(u8,u8,u8),
    Echo(String),
    Move(Point),
    Quit,        
}

pub struct Tvec{ pub v: u8, pub w: String }


#[derive(Debug)]
pub struct Point (pub u8, pub u8);

#[derive(Debug)]
pub struct State {
    pub color: (u8, u8, u8),
    pub position: Point,
    pub quit: bool,
    pub message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    pub fn process(&mut self, message: Message) {
        let _ = match message {
            Message::ChangeColor(c1, c2, c3) => self.change_color((c1,c2,c3)),
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),    
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point(0, 0),
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point ( 10, 15 )));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.0, 10);
        assert_eq!(state.position.1, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }

    #[test]
    fn test_points() {
        let p1 = Point(1,4);
        let p2 = Point(6,p1.1);
        let p3 = Point(3,14);

        let p4: &Point = &p3;

        assert_eq!(p1.1, p2.1);
        assert_ne!(p1.0, p2.0);
        assert_eq!(p3.0, p4.0);
        assert_eq!(p3.1, p4.1);
    }

    #[test]
    fn test_t () {
        let t1 = Tvec{ v: 5, w: "9".to_string() };
        let t2 = Tvec{ v: 8, ..t1};

        let t3 = Tvec{ v: 3, w: "7".to_string() };
        let Tvec{ v: _, w: _ } = t3;

        assert_eq!(t1.w, t2.w);
        assert_ne!(t1.v, t2.v);
        assert_eq!(t3.v, 3 );
    }
}
