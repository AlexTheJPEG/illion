use num::bigint::BigUint;
use std::io;

/// Turns a string representation of a number to a BigUint
fn str_to_biguint(str: &str) -> BigUint {
    BigUint::parse_bytes(str.as_bytes(), 10).unwrap()
}

/// Determines if a BigUint is within a range of numbers
fn biguint_in_range(num: &BigUint, start: &str, end: &str) -> bool {
    &str_to_biguint(start) <= num && num <= &str_to_biguint(end)
}

fn get_common_prefix(num: String) -> String {
    let common_prefixes: [&str; 10] = ["", "mi", "bi", "tri", "quadri", "quinti", "sexti", "septi", "octi", "noni"];
    common_prefixes[num.parse::<usize>().unwrap()].to_string()
}

fn get_hundreds_prefix(num: String, last_letter: bool) -> String {
    let unit_prefixes: [&str; 10] = ["", "un", "duo", "tre", "quattuor", "quin", "se", "septe", "octo", "nove"];
    let ten_prefixes: [&str; 10] = ["", "deci", "viginti", "triginta", "quadraginta", "quinquaginta", "sexaginta", "septuaginta", "octoginta", "nonaginta"];
    let hundred_prefixes: [&str; 10] = ["", "centi", "ducenti", "trecenti", "quadringenti", "quingenti", "sescenti", "septingenti", "octingenti", "nongenti"];

    let precede_tens: [&str; 10] = ["", "", "ms", "s", "s", "s", "", "", "mx", ""];
    let precede_hundreds: [&str; 10] = ["", "x", "", "s", "s", "s", "", "", "mx", ""];

    let hundred: u32 = num.parse::<u32>().unwrap() / 100;
    let ten: u32 = num.parse::<u32>().unwrap() / 10 % 10;
    let unit: u32 = num.parse::<u32>().unwrap() % 10;

    let precedent: &str = if ten == 0 { precede_hundreds[hundred as usize] } else { precede_tens[ten as usize] };
    
    let mut name: String = String::from(unit_prefixes[unit as usize]);
    match unit {
        3 => { name.push_str(if precedent.contains("s") || precedent.contains("x") { "s" } else { "" }); },
        6 => { name.push_str(if precedent.contains("s") { "s" } else { "x" }); },
        7 | 9 => { name.push_str(if precedent.contains("m") { "m" } else { "n" }); },
        _ => (),
    }

    name.push_str(ten_prefixes[ten as usize]);
    name.push_str(hundred_prefixes[hundred as usize]);

    name[..name.len() - (!last_letter as usize)].to_string()
}

/// Gets the num-th illion
fn get_illion(num: String) -> String {
    let big_num: BigUint = str_to_biguint(&num);

    if biguint_in_range(&big_num, "1", "9") {
        return format!("{}illion", get_common_prefix(num));
    } else if biguint_in_range(&big_num, "10", "999") {
        return format!("{}illion", get_hundreds_prefix(num, false));
    }

    String::from("")
}

fn main() {
    loop {
        println!("-illion number (type 0 to exit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_string();

        if input == "0" { break; }

        if input.chars().all(|c| c.is_ascii_digit()) {
            println!("{}\n", get_illion(input));
        } else {
            println!("Not a valid number\n");
        }
    }
}
