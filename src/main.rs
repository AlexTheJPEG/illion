use num::bigint::BigUint;
use num::ToPrimitive;
use std::io;

/// Turns a string representation of a number to a BigUint
fn str_to_biguint(str: &str) -> BigUint {
    BigUint::parse_bytes(str.as_bytes(), 10).unwrap()
}

/// Determines if a BigUint is within a range of numbers
fn biguint_in_range(num: &BigUint, start: &str, end: &str) -> bool {
    &str_to_biguint(start) <= num && num <= &str_to_biguint(end)
}

/// Gets the num-th illion
fn get_illion(num: String) -> String {
    let big_num: BigUint = str_to_biguint(&num);
    let mut illion: String = String::new();

    let common_prefixes: [&str; 10] = ["", "m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non"];
    let unit_prefixes: [&str; 10] = ["", "un", "duo", "tre", "quattuor", "quin", "se", "septe", "octo", "nove"];
    let ten_prefixes: [&str; 10] = ["", "dec", "vigint", "trigint", "quadragint", "quinquagint", "sexagint", "septuagint", "octogint", "nonagint"];
    let hundred_prefixes: [&str; 10] = ["", "cent", "ducent", "trecent", "quadringent", "quingent", "sescent", "septingent", "octingent", "nongent"];

    let tier_two_prefixes: [&str; 10] = ["", "milli", "micro", "nano", "pico", "femto", "atto", "zepto", "yocto", "xona"];
    let tier_two_unit_prefixes: [&str; 10] = ["", "", "du", "tri", "quadri", "quin", "sex", "septi", "octi", "noni"];

    if biguint_in_range(&big_num, "1", "9") {
        illion = format!("{}illion", common_prefixes[big_num.to_u32().unwrap() as usize]);
    } else if biguint_in_range(&big_num, "10", "99") {
        let ten: u32 = big_num.to_u32().unwrap() / 10;
        let unit: u32 = big_num.to_u32().unwrap() % 10;

        let ten_prefix: &str = ten_prefixes[ten as usize];
        let mut unit_prefix: String = String::from(unit_prefixes[unit as usize]);
        match unit {
            3 => { unit_prefix.push_str(if ten == 2 || ten == 3 { "s" } else { "" }); },
            6 => { unit_prefix.push_str(if ten == 2 || ten == 3 { "s" } else { "x" }); },
            7 => { unit_prefix.push_str(if ten == 2 { "m" } else { "n" }); },
            9 => { unit_prefix.push_str(if ten == 3 { "n" } else { "m" }); },
            _ => (),
        }

        illion = format!("{}{}illion", unit_prefix, ten_prefix);
    } else if biguint_in_range(&big_num, "100", "999") {
        let hundred: u32 = big_num.to_u32().unwrap() / 100;
        let ten: u32 = big_num.to_u32().unwrap() / 10 % 10;
        let unit: u32 = big_num.to_u32().unwrap() % 10;

        let hundred_prefix: &str = hundred_prefixes[hundred as usize];

        let mut ten_prefix: String = String::from(ten_prefixes[ten as usize]);
        if ten != 0 {
            ten_prefix.push_str(if ten == 1 || ten == 2 { "i" } else { "a" });
        } 

        let mut unit_prefix: String = String::from(unit_prefixes[unit as usize]);
        match unit {
            6 => { unit_prefix.push_str("x"); },
            7 => { unit_prefix.push_str("n"); },
            9 => { unit_prefix.push_str("m"); },
            _ => (),
        }

        illion = format!("{}{}{}illion", unit_prefix, ten_prefix, hundred_prefix);
    } else if biguint_in_range(&big_num, "1000", "999999999999999999999999999999") {
        let big_num_string: String = big_num.to_str_radix(10);
        let big_num_chunks: Vec<String> = big_num_string.chars().rev()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|c| c.iter().rev().collect::<String>()).rev()
            .collect::<Vec<String>>();
        let mut prefixes: Vec<String> = vec![];

        for (i, c) in big_num_chunks.iter().enumerate() {
            if i != big_num_chunks.len() - 1 {
                let hundred: u32 = c.parse::<u32>().unwrap() / 100;
                let ten: u32 = c.parse::<u32>().unwrap() / 10 % 10;
                let unit: u32 = c.parse::<u32>().unwrap() % 10;
                
                if !(hundred == 0 && ten == 0 && unit == 0) {
                    let tier_two_prefix_index: usize = big_num_chunks.len() - 1 - i;
                    let tier_two_prefix: &str = tier_two_prefixes[tier_two_prefix_index];

                    let mut hundred_prefix: String = String::from(hundred_prefixes[hundred as usize]);
                    if hundred != 0 {
                        hundred_prefix.push_str("i");
                    }

                    let mut ten_prefix: String = String::from(ten_prefixes[ten as usize]);
                    if ten != 0 {
                        ten_prefix.push_str(if ten == 1 || ten == 2 { "i" } else { "a" });
                    } 

                    let mut unit_prefix: String = String::new();
                    if hundred == 0 && ten == 0 {
                        unit_prefix.push_str(tier_two_unit_prefixes[unit as usize]);
                    } else {
                        unit_prefix.push_str(unit_prefixes[unit as usize]);
                        match unit {
                            6 => { unit_prefix.push_str("x"); },
                            7 => { unit_prefix.push_str("n"); },
                            9 => { unit_prefix.push_str("m"); },
                            _ => (),
                        }
                    }

                    let item = format!("{}{}{}{}-", unit_prefix, ten_prefix, hundred_prefix, tier_two_prefix);
                    if item != "" { prefixes.push(item); }
                }
            } else {
                let item = get_illion(c.to_string());
                if item != "" { prefixes.push(item); }
            }
        }

        illion = prefixes.join("");

        if illion.chars().last().unwrap() == '-' {
            illion = String::from(&illion[..illion.len() - 2]);
            illion.push_str("illion");
        }
    }

    illion
}

fn main() {
    loop {
        println!("-illion number (type 0 to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_string();

        if input == "0" { break; }
        if input.chars().all(|c| c.is_ascii_digit()) { println!("{}\n", get_illion(input)); }
    }
}
