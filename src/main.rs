use std::io;

/* Sources:
* https://googology.fandom.com/wiki/-illion
* https://kyodaisuu.github.io/illion/
* https://sites.google.com/site/pointlesslargenumberstuff/home/1/bowersillions
* */

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

    let poly_unit_prefixes: [&str; 20] = ["", "me", "due", "trio", "tetre", "pente", "hexe", "hepte", "octe", "enne",
                                          "vece", "mece", "duece", "trece", "tetrece", "pentece", "hexece", "heptece", "octece", "ennece"];
    let ten_prefixes: [&str; 10] = ["", "", "icos", "triacont", "tetracont", "pentacont", "hexacont", "heptacont", "octacont", "ennacont"];
    let hundred_prefixes: [&str; 10] = ["", "hect", "dohect", "triahect", "tetrahect", "pentahect", "hexahect", "heptahect", "octahect", "ennahect"];

    let mut chunk_prefix: String;
    let chunk_value: u32 = chunk.parse().unwrap();
    
    if index != 0 {
        let hundred: u32 = index as u32 / 100;
        let ten: u32 = index as u32 / 10 % 10;
        let unit: u32 = index as u32 % 10;

        match chunk_value {
            0 => { return String::from(""); },
            1..=9 => { chunk_prefix = String::from(base_prefixes[chunk_value as usize]); }, 
            _ => { chunk_prefix = get_hundreds_prefix(chunk.clone(), true); },
        }

        match ten {
            0 | 1 => { chunk_prefix.push_str(unit_prefixes[(unit + (10 * ten)) as usize]); },
            2..=9 => {
                chunk_prefix.push_str(poly_unit_prefixes[unit as usize]);
                chunk_prefix.push_str(ten_prefixes[ten as usize]);
                chunk_prefix.push_str(if hundred == 0 { "o" } else { "e" });
            }
            _ => (),
        }

        chunk_prefix.push_str(hundred_prefixes[hundred as usize]);
        chunk_prefix.push_str(if hundred != 0 { "o" } else { "" });

        chunk_prefix
    } else {
        match chunk_value {
            0 => { String::from("") },
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

// Gets the num-th illion
fn get_illion(num: String) -> String {
    match num.len() {
        1 => { return format!("{}llion", get_common_prefix(num)); },
        2..=3 => { return format!("{}illion", get_hundreds_prefix(num, false)); },
        4..=2703 => { return format!("{}illion", get_entire_tier_two_prefix(num, false)); }
        _ => { return String::from(""); }
    }
}

fn main() {
    loop {
        println!("-illion number (type 0 to exit):");

        let mut input: String = String::new();
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
