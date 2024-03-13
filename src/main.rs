mod one_level_mod {
    pub fn print_letter() {
        for c in 'a'..='z' {
            print!("{}", c);
        }
        for c in 'A'..='Z' {
            print!("{}", c);
        }
        println!();
    }
}

mod two_level_mod {
    pub mod two_level_inner {
        pub fn print_letter() {
            for c in 'A'..='z' {
                if c.is_alphabetic() {
                    print!("{}", c);
                }
            }
            println!();
        }
    }
}

fn main() {
    println!("One level module!");
    one_level_mod::print_letter();
    println!("Two level module!");
    two_level_mod::two_level_inner::print_letter();
}
