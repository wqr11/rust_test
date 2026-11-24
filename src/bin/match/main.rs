fn get_random_tuple() -> (i32, i32, i8, i128) {
    (3, 3, 3, 2)
}

fn get_random_number() -> i32 {
    3
}

enum SomeEvent {
    Noop,
    Empty,
    Click { x: i32, y: i32 },
    Random((i32,)),
}

fn main() {
    match get_random_number() {
        n @ 1..10 => println!("from 1 to 10 {n}"),
        n @ 5..100 => println!("from 5 to 100 {n}"),
        n @ (100 | 200 | 300 | 500) => println!("YO SS {n}"),
        _ => (),
    }

    let a = [1, 12, 312, 3, 12];

    let r = match get_random_tuple() {
        (3, 3, 3, 3) => println!("SPECIAL CASE"),
        (a, b, c, d) => println!("NORMAL CASE"),
    };

    match a {
        [a, tail @ .., b] => println!("{:?}", tail),
    }

    let event: SomeEvent = SomeEvent::Click { x: 512, y: 123 };
    // let event = SomeEvent::Noop;

    let e2 = SomeEvent::Empty;
    if let SomeEvent::Noop = e2 {
        println!("a is noop");
    } else {
        println!("a is not 10");
    }

    match event {
        SomeEvent::Noop => println!("NOOP"),
        SomeEvent::Click { x, y } if x > 500 => println!("X > 500 {x} {y}"),
        SomeEvent::Click { x, y } if x < 600 => println!("X < 600 {x} {y}"),
        SomeEvent::Click { x, y } => println!("CLICKED {x} {y}"),
        _ => println!("UNEXP EVENT"),
    }

    if let SomeEvent::Noop = e2 {
        println!("e2 is noop");
    } else {
        println!("e2 is not noop!")
    }
}
