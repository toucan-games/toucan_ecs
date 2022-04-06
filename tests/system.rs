use std::fmt::Display;
use std::ops::Add;

use toucan_ecs::system::System;

fn hello_world() {
    println!("Hello, World");
}

fn increment(integer: i32) {
    println!("Result of increment is {}", integer + 1)
}

fn sum_two<T>(first: T, second: T)
where
    T: Add<Output = T>,
    T::Output: Display,
{
    println!("Result of sum of 2 is {}", first + second)
}

fn sum_12<T>(t1: T, t2: T, t3: T, t4: T, t5: T, t6: T, t7: T, t8: T, t9: T, t10: T, t11: T, t12: T)
where
    T: Add<Output = T>,
    T::Output: Display,
{
    let result = t1 + t2 + t3 + t4 + t5 + t6 + t7 + t8 + t9 + t10 + t11 + t12;
    println!("Result of sum of 12 is {}", result)
}

fn make_10(integer: &mut i32) {
    *integer = 10;
}

fn run_system<Args, S>(mut system: S, args: Args)
where
    S: System<Args>,
{
    system.run(args)
}

#[test]
fn test() {
    run_system(hello_world, ());
    run_system(increment, (1,));
    run_system(sum_two, (1, 2));
    run_system(sum_12, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));

    let log_system = |message: &str| println!(r#"Message is "{}""#, message);
    run_system(log_system, ("something happened",));

    struct StructSystem;

    impl System<()> for StructSystem {
        fn run(&mut self, _: ()) {
            println!("Hello from the struct system")
        }
    }

    run_system(StructSystem, ());

    let mut integer = 0;
    run_system(make_10, (&mut integer,));
    assert_eq!(integer, 10);
}
