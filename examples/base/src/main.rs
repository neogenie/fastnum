use fastnum::*;

// Default example output:
//
// Hello, fastnum!
// Input (0.8)
// square 0.64
// square root 0.8
// From float 3.3: 3.29999999999999982236431605997495353222
// 24.0 + 34.0 = 58.0
// 0 IS equal to zero
// 24.00000000000000 / 1.00000000000000 = 24
// 24.0 / 1.000000000000000 = 24
// 24.0 / 1.5 = 16
// 24.000 / 1.50 = 16.0

fn main() {
    println!("Hello, fastnum!");
    let dec = dec128!(0.8);
    println!("Input ({dec})");

    println!("square {}", dec * dec);
    println!("square root {}", (dec * dec).sqrt().reduce());

    println!("From float 3.3: {}", D128::from(3.3));

    println!(
        "{} + {} = {}",
        dec128!(24.0),
        dec128!(34.0),
        dec128!(24.0) + dec128!(34.0)
    );

    if D128::from(0.0).is_zero() {
        println!("{} IS equal to zero", D128::from(0.0));
    } else {
        println!("{} is not equal to zero", D128::from(0.0));
    }

    println!(
        "{} / {} = {}",
        dec128!(24.00000000000000),
        dec128!(1.00000000000000),
        dec128!(24.00000000000000) / dec128!(1.00000000000000)
    );
    println!(
        "{} / {} = {}",
        dec128!(24.0),
        dec128!(1.000000000000000),
        dec128!(24.0) / dec128!(1.000000000000000)
    );
    println!(
        "{} / {} = {}",
        dec128!(24.0),
        dec128!(1.5),
        dec128!(24.0) / dec128!(1.5)
    );
    println!(
        "{} / {} = {}",
        dec128!(24.000),
        dec128!(1.50),
        dec128!(24.000) / dec128!(1.50)
    );
}
