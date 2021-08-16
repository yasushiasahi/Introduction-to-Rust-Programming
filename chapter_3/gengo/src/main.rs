trait Tweet {
    fn tweet(&self);
    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }
    fn shout(&self) {
        println!("Uoooooooooooooooooooooooooooooooooooohhhhh!!!!");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}
impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn calc_data(data: String) -> String {
    println!("{}", data);
    data
}
fn calc_data_borrw(data: &String) {
    println!("{}", data);
}

struct Droppable;
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Resource will be released!");
    }
}

fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    let _t1 = make_tuple(1, 3);
    let _t2 = make_tuple("hello", "world");
    let _t3 = make_tuple(vec![1, 3, 3], vec![4, 5]);
    let _t4 = make_tuple(3, "years old");

    let c1 = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    let c2 = c1;
    println!("{} {} {}", c2.r, c2.g, c2.b);

    let mut importaint_data = "hello, world".to_string();
    importaint_data = calc_data(importaint_data);
    println!("{}", importaint_data);

    let importaint_data2 = "hello, world".to_string();
    calc_data_borrw(&importaint_data2);
    println!("{}", importaint_data2);

    let x = 5;
    let y = &x;
    let z = &x;
    dbg!(x);
    dbg!(y);
    dbg!(z);

    // let mut xx = 5;
    // let yy = &mut xx;
    // let zz = &mut xx;
    // dbg!(yy);
    // dbg!(zz);

    // let yyy;
    // {
    //     let x = 5;
    //     yyy = &x;
    //     dbg!(x);
    // }
    // dbg!(yyy);

    fn func() {
        let x = 5;
        let y = &x;
        // let z = &mut x;

        dbg!(x);
        dbg!(y);
    }
    func();

    {
        let d = Droppable;
    }
}
