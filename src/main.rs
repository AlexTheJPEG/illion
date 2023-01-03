use std::io;

macro_rules! log_of {
    ($val:expr, $base:expr, $type:ty) => {
        ($val as f32).log($base) as $type
    }
}

fn get_illion(num: u32) -> String {
    let mut illion = String::new(); 
    let base_prefixes: [&str; 10] = ["", "m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non"];

    let unit_prefixes: [&str; 10] = ["", "un", "duo", "tre", "quattuor", "quin", "se", "septe", "octo", "nove"];
    let tens_prefixes: [&str; 10] = ["", "dec", "vigint", "trigint", "quadragint", "quinquagint", "sexagint", "septuagint", "octogint", "nonagint"];

    match num {
       1..=9 => {
        illion.push_str(base_prefixes[num as usize]);
       },
       10..=99 => {
        let unit: u32 = num % 10;
        let ten: u32 = num / 10;

        let mut unit_prefix: String = String::from(unit_prefixes[unit as usize]);
        match unit {
            3 => {
                unit_prefix.push_str(if ten == 2 || ten == 3 { "s" } else { "" });
            },
            6 => {
                unit_prefix.push_str(if ten == 2 || ten == 3 { "s" } else { "x" });
            },
            7 => {
                unit_prefix.push_str(if ten == 2 { "m" } else { "n" });
            },
            9 => {
                unit_prefix.push_str(if ten == 3 { "n" } else { "m" });
            }
            _ => (),
        }

        illion.push_str(&unit_prefix);
        illion.push_str(tens_prefixes[(num / 10) as usize]);
       },
       _ => (), 
    };

    illion.push_str("illion");

    illion
}

fn main() {
    loop {
        println!("-illion number (type 0 to exit): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let x: u32 = input.trim().parse().expect("Error parsing number");

        if x == 0 { break; }
        println!("{}\n", get_illion(x));
    }
}
