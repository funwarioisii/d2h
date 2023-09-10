fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        args.iter().skip(1).for_each(|arg| {
            println!("{:x}", arg.parse::<u32>().unwrap());
        });
    }

    let mut color_code = String::from("#");
    for arg in args.iter().skip(1) {
        let parsed = arg.parse::<u8>();
        match parsed {
            Ok(value) => {
                color_code.push_str(&format!("{:x}", value));
            }
            Err(_) => {
                println!("Invalid argument: {}", arg);
                std::process::exit(1);
            }
        }
    }
    println!("{}", color_code);
}
