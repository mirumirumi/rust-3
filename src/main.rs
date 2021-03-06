const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x: i32 = 5;
    println!("x: {}", x);

    x = 6;
    println!("x: {}", x);

    let sum = 5 + 10;

    let diff = 95.5 - 4.3;

    let product = 4 * 30;

    let division = 56.7 / 32.2;

    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'c';

    let z = '๐';

    println!("{} {} {} {} {} {} {} {} {}", sum, diff, product, division, remainder, t, f, c, z);

    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, _z) = tuple;
    let (f, c, z) = tuple;

    println!("{} {} {} {} {} {}", x, y, z, f, c, z);

    let b = tuple.1;

    println!("{}", b);

    let a = [1, 2, 3, 4, 5];

    println!("{}, {}", a[0], a[1]);

    let i = 10;
    // println!("{}, {}", a[0], a[i]);  // error

    another_fun(5);

    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);

    println!("{}", another_fun(50));

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("{}", number);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // ็บๅฐ๏ผ
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // ๅคใฏ{}ใงใ
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for elem in a {
        println!("{}", elem);
    }

    for n in (1..=4).rev() {
        println!("{}", n);
    }
}

// main()ใฎๅใงใใใ
fn another_fun(x: i32) -> i32 {
    println!("๐{}", x);
    // return x;

    if x < 10 {  // boolใใใชใใจใ?ใ๏ผ1ใจใใฏใชใ๏ผ
        x
    } else {
        x * 100
    }
}

