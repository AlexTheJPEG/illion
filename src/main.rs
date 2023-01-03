use std::io;

fn get_illion(num: u32) -> String {
    let mut illion = String::new(); 
    let base_prefixes: [&str; 10] = ["", "m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non"];

    let unit_prefixes: [&str; 10] = ["", "un", "duo", "tre", "quattuor", "quin", "se", "septe", "octo", "nove"];
    let ten_prefixes: [&str; 10] = ["", "dec", "vigint", "trigint", "quadragint", "quinquagint", "sexagint", "septuagint", "octogint", "nonagint"];
    let hundred_prefixes: [&str; 10] = ["", "cent", "ducent", "trecent", "quadringent", "quingent", "sescent", "septingent", "octingent", "nongent"];

    match num {
       1..=9 => {
        illion.push_str(base_prefixes[num as usize]);
        illion.push_str("illion");
       },
       10..=99 => {
        let ten: u32 = num / 10;
        let unit: u32 = num % 10;

        let ten_prefix: String = String::from(ten_prefixes[(num / 10) as usize]);

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
        illion.push_str(&ten_prefix);
        illion.push_str("illion");
       },
       100..=999 => {
        let hundred: u32 = num / 100;
        let ten: u32 = num / 10 % 10;
        let unit: u32 = num % 10;

        let hundred_prefix: String = String::from(hundred_prefixes[hundred as usize]);

        let mut ten_prefix: String = String::from(ten_prefixes[ten as usize]);
        if ten != 0 {
            ten_prefix.push_str(if ten == 1 || ten == 2 { "i" } else { "a" });
        } 

        let mut unit_prefix: String = String::from(unit_prefixes[unit as usize]);
        unit_prefix.push_str(if unit == 6 { "x" } else { "" });
        unit_prefix.push_str(if unit == 7 { "n" } else { "" });
        unit_prefix.push_str(if unit == 9 { "m" } else { "" });

        illion.push_str(&unit_prefix);
        illion.push_str(&ten_prefix);
        illion.push_str(&hundred_prefix);
        illion.push_str("illion");
       },
       1000..=999_999 => {
        let mill: u32 = num / 1000;
        let rest: u32 = num % 1000;

        let mut mill_prefix: String = get_illion(mill);
        mill_prefix = String::from(mill_prefix.strip_suffix("on").unwrap());
        illion.push_str(&mill_prefix);

        if rest == 0 {
            illion.push_str("llion");
        } else {
            illion.push_str("-");
            illion.push_str(&get_illion(rest));
        }
       },
       _ => (), 
    };

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
