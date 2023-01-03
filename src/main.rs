use std::io;
    
fn get_illion(num: u32) -> String {
    let mut illion = String::new(); 
    let base_prefixes: [&str; 9] = ["m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non"];

    match num {
       1..=9 => illion.push_str(base_prefixes[(num - 1) as usize]),
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
