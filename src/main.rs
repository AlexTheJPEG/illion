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

    let precede_tens: [&str; 10] = ["", "n", "ms", "ns", "ns", "ns", "n", "n", "mx", ""];
    let precede_hundreds: [&str; 10] = ["", "nx", "n", "ns", "ns", "ns", "n", "n", "mx", ""];

    let hundred: u32 = num.parse::<u32>().unwrap() / 100;
    let ten: u32 = num.parse::<u32>().unwrap() / 10 % 10;
    let unit: u32 = num.parse::<u32>().unwrap() % 10;

    let precedent: &str = if ten == 0 { precede_hundreds[hundred as usize] } else { precede_tens[ten as usize] };
    
    let mut name: String = String::from(unit_prefixes[unit as usize]);
    match unit {
        3 => { name.push_str(if precedent.contains("s") || precedent.contains("x") { "s" } else { "" }); },
        6 => { name.push_str(if precedent.contains("s") { "s" } else { "x" }); },
        7 | 9 => { name.push_str(
            if precedent.contains("m") { "m" }
            else if precedent.contains("n") { "n" }
            else { "" }
        ); },
        _ => (),
    }

    name.push_str(ten_prefixes[ten as usize]);
    name.push_str(hundred_prefixes[hundred as usize]);

    name[..name.len() - (!last_letter as usize)].to_string()
}

fn separate_into_chunks(num: String) -> Vec<String> {
    num.chars()
        .collect::<Vec<char>>()
        .rchunks(3).rev()
        .map(|c| c.into_iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn get_chunk_tier_two_prefix(chunk: String, index: usize) -> String {
    let base_prefixes: [&str; 10] = ["", "", "du", "tre", "quadri", "quinti", "sexti", "septi", "octi", "noni"];
    let unit_prefixes: [&str; 20] = ["", "milli", "micro", "nano", "pico", "femto", "atto", "zepto", "yocto", "xono",
                                     "veco", "meco", "dueco", "treco", "tetreco", "penteco", "hexeco", "hepteco", "octeco", "enneco"];

    let chunk_prefix: String;
    let chunk_value: u32 = chunk.parse().unwrap();
    
    if index != 0 {
        match chunk_value {
            0 => { return String::from(""); },
            1..=9 => { chunk_prefix = String::from(base_prefixes[chunk_value as usize]); }, 
            _ => { chunk_prefix = get_hundreds_prefix(chunk.clone(), true); },
        }
        format!("{}{}", chunk_prefix, unit_prefixes[index])
    } else {
        match chunk_value {
            0 => { return String::from(""); },
            1..=9 => { get_common_prefix(chunk) }, 
            _ => { get_hundreds_prefix(chunk, true) },
        }
    }
}

fn get_entire_tier_two_prefix(num: String, last_letter: bool) -> String {
    let chunks: Vec<String> = separate_into_chunks(num);

    let mut prefixes: Vec<String> = vec![];

    for (index, chunk) in chunks.iter().enumerate() {
        let prefix = get_chunk_tier_two_prefix(chunk.to_string(), chunks.len() - index - 1);
        if !prefix.is_empty() {
            prefixes.push(prefix);
        }
    }

    let entire_prefix: String = prefixes.join("-");
    entire_prefix[..entire_prefix.len() - (!last_letter as usize)].to_string()
}

/// Gets the num-th illion
fn get_illion(num: String) -> String {
    let big_num: BigUint = str_to_biguint(&num);

    if biguint_in_range(&big_num, "1", "9") {
        return format!("{}llion", get_common_prefix(num));
    } else if biguint_in_range(&big_num, "10", "999") {
        return format!("{}illion", get_hundreds_prefix(num, false));
    } else if biguint_in_range(&big_num, "1000", &"9".repeat(60)){
        return format!("{}illion", get_entire_tier_two_prefix(num, false));
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
