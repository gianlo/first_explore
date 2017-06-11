extern crate first_explore;

fn read_args(){
    // example: reading from command line arguments
    let args = std::env::args();
    println!("{}", args.fold(String::from(""), |acc, arg| acc + " " + arg.as_str()));
}

fn function_in_module(){
    // example: use a function from a package
    use first_explore::utils::ints;
    let x = 4000000000;
    let y = 2;
    println!("{0} is larger than {1}: {2}", x, y, ints::is_larger(x, y));
}

fn trait_for_custom_type(){
    // example: use a trait on a custom defined type
    use first_explore::models::gdp::NationalGDP;
    use first_explore::typeclasses::Colorizable;
    let m: u32 = 12;
    let c = NationalGDP { in_local_currency: m * 16843009};
    println!("{:?} colorizes to {:?}", c, c.color());
    println!("RGB values above should all be {}",m);
}

fn main() {
    read_args();
    function_in_module();
    trait_for_custom_type();
}
