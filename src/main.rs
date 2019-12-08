use std::fs::File;
use std::io::{self, Lines, BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let mut cargo_fuelrec = 0;
    let mut total_fuelrec = 0;
    let mut total2_fuelrec = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let intv: u32 = ip.parse().unwrap();
                let cargo_fuel = fuel(intv);
                let total_fuel = total_fuel(intv);
                let total2_fuel = fuel_fuel(intv);
                // println!("{} {} {}", ip, intv, f);
                cargo_fuelrec = cargo_fuelrec + cargo_fuel;
                total_fuelrec = total_fuelrec + total_fuel;
                total2_fuelrec = total2_fuelrec + total2_fuel;
            }
        }
    }
    println!("{}", fuel(12));
    println!("{}", fuel(14));
    println!("{}", fuel(1969));
    println!("{}", fuel(100756));

    println!("solution1 = {} verified=3390596", cargo_fuelrec);
    println!("solution2 = {}", total_fuelrec);
    println!("solution2 = {}", total2_fuelrec);

//    fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
//    The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.

    println!("fuel_fuel(1969) = {} exp = 966", fuel_fuel(1962));
    println!("fuel_fuel(100756) = {} exp = 50346", fuel_fuel(100756));
    println!("total_fuel(1969) = {} exp = 966", total_fuel(1962));
    println!("total_fuel(100756) = {} exp = 50346", total_fuel(100756));
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn total_fuel(mass: u32) -> u32 {
    let cargo_fuel = fuel(mass);
    let extra_fuel = fuel_fuel(cargo_fuel);
    cargo_fuel + extra_fuel
}

fn fuel(mass: u32) -> u32 {
    let rest = mass % 3;
    let divx = mass - rest;
    if divx < 9 { return 0; };
    return divx / 3 - 2;
}

fn fuel_fuel(mass: u32) -> u32 {
    let ff = fuel(mass);
    println!("ff = {}", ff);
    if ff == 0 {
        0
    } else {
        ff + fuel_fuel(ff)
    }
}

// For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
// For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
// For a mass of 1969, the fuel required is 654.
// For a mass of 100756, the fuel required is 33583.