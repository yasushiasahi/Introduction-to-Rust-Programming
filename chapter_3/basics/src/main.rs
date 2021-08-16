fn main() {
    // println!("Hello, world!");

    // let result: Result<i32, String> = Ok(200);
    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("Err: {}", err),
    // }

    // let result2: Result<i32, String> = Ok(200);
    // if let Ok(code) = result2 {
    //     println!("code: {}", code)
    // }

    // let result3: Result<i32, String> = Ok(200);
    // println!("code: {}", result3.unwrap_or(-1));

    // fn func(code: i32) -> Result<i32, String> {
    // 	println!("code: {}", code);
    // 	Ok(100)
    // }

    // let result4: Result<i32, String> = Ok(200);

    // let next_resul = result4.and_then(func);
    // let v = vec![1,2].get(1);

    // let byte_array = [b'h', b'e',b'1',b'o'];
    // print(Box::new(byte_array))

    // let immut_val = 10;
    // let mut mut_val = 20;
    // mut_val += immut_val;
    // println!("mut_val: {}",mut_val)

    // let number = 1;
    // if 0 < number {
    // 	println!("0 < number")
    // } else if number < 0 {
    // 	println!("number > 0 ")
    // } else {
    // 	println!("number == 0")
    // }
    // let number = 1;
    // let _result = if 0 <= number {
    // 	number
    // } else {
    // 	-number
    // };

    // let mut count = 0;
    // let resut = loop {
    // 	println!("count: {}",count);
    // 	count += 1;
    // 	if count == 10 {
    // 	    break count;
    // 	}
    // };
    // println!("resut: {}",resut);

    // let mut count2 = 0;
    // while count2 < 10 {
    // 	println!("count2: {}", count2);
    // 	count2 += 1;
    // }

    // let _count3: i32;
    // for _count3 in 0..10 {
    // 	println!("count3: {}", _count3)
    // }

    // let array = [0,1,2,3,4,5,6,6,7,8,9];
    // for element in &array {
    // 	println!("element: {}", element)
    // }

    // 'main: loop {
    // 	println!("main loop start");
    // 	'sub: loop {
    // 	    println!("sub loop start");
    // 	    break 'main;
    // 	}

    // }

    // let i: i32 = 18;
    // match i {
    // 	1 => println!("1"),
    // 	2 => println!("2"),
    // 	_ => println!("misc")

    // }

    // enum Color {
    // 	Red,
    // 	Blue,
    // 	Green,
    // }
    // let c = Color::Red;
    // match c {
    // 	Color::Red => println!("Red"),
    // 	Color::Blue => println!("Blue"),
    // 	 Color::Green => println!("Green"),
    // }

    // let result:Result<i32,String> = Ok(100);
    // let resutl_number = match result {
    // 	Ok(number) => number,
    // 	Err(message) => {
    // 	    println!("Error: {}", message);
    // 		-1
    // 	},
    // };
    // println!("resutl_number: {}", resutl_number);

    // for number in 1..=5 {
    // 	println!("{}",number);
    // }

    // let it = Iter {
    // 	current:0,
    // 	max: 10,
    // };
    // for num in it {
    // 	println!("{}",num)
    // }

    let x = add(1, 2);
    println!("x = {}", x);

    let mut y = abs(9);
    println!("y = {}", y);

    y = abs(-9);
    println!("y = {}", y);

    let p = Person {
        name: String::from("Taro"),
        age: 20,
    };

    p.say_name().say_age();

    let p2 = Person::new("Aki", 33);
    p2.say_age().say_name();

    let py = std::f32::consts::PI;
    println!("{}",py)


}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }
    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }
    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old.", self.age);
        self
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn abs(number: i32) -> i32 {
    if number < 0 {
        return -number;
    }
    number
}

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}

// fn print(s:Box<[u8]>) {
//     println!("{:?}",s);
// }
