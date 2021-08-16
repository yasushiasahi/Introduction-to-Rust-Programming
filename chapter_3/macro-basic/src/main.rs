use std::io::Write;

fn main() {
    let s = concat!("A", "b2", 3);
    println!("s: {}", s);
    let ss = format!("{}-{:?}", s, ("D", 5));
    println!("ss: {}", ss);
    let sss = format!("{}{}", "abc", "def");
    println!("sss: {}", sss);

    print!("hello");
    println!("Hello {}", "world");
    eprint!("hello {}", "error");
    eprintln!("hello");

    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    writeln!(&mut w, " is 123");
    dbg!(w);

    let v = vec![1, 2, 3];
    println!("v: {:?}", v);

    println!("difined in file: {}", file!());
    println!("difined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    let mut p = HappyPerson {
        name: "Takashi".to_string(),
        state: Emotion::Happy,
    };
    println!("{}", p.get_anger());

    panic!("it will panic");
}

enum Emotion {
    Anger,
    Happy,
}
trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}
struct HappyPerson {
    name: String,
    state: Emotion,
}
impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
        unimplemented!()
    }
    fn get_happy(&mut self) -> String {
        format!("{} is always happy.", self.name)
    }
    fn tell_state(&self) -> String {
        todo!()
    }
}
