mod leetcode;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new(components: Vec<Box<dyn Draw>>) -> Self {
        Self { components }
    }

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    width: usize,
    height: usize,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("I am a Button: {:?}", self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    width: usize,
    height: usize,
    label: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("I am a SelectBox: {:?}", self);
    }
}

fn main() {
    // println!("Hello, world!");
    // let screen = Screen::new(vec![
    //     Box::new(Button {
    //         width: 10,
    //         height: 10,
    //         label: String::from("button"),
    //     }),
    //     Box::new(SelectBox {
    //         width: 10,
    //         height: 10,
    //         label: vec![String::from("1"), String::from("2")],
    //     }),
    // ]);
    // screen.run();
    println!("{}", leetcode::Solution::interpret(String::from("(al)G(al)()()G")));
}
