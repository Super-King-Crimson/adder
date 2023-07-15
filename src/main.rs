use rand::Rng;

fn main() {
    println!("Hello, packages! Thanks for coming!");
    
    //There's more than one toml here: 
        //one that configures the entire package, and one that just configures adder
    //The package toml doesn't have a workspace section, it has a package section
    //Also when you run cargo buil the target file goes into the root dir of the workspace
    //That's because all these crates are supposed to work together, 
        //so they should compile to the same target.
    //If crates in a package made their own targets and called each other,
        //they'd have to rebuild other crates and put them in their own targets to use them 

    //You need to explicitly list crates that share dependencies (check the toml)
    let x = 10;
    println!("{x} + 1 is {}", add_one::add_one(x));
    //Since this is the package's only bin, we can just cargo run
    //If we only want to compile + run this crate, we can use -p and put the dir name
        //cargo run -p adder

    println!("Notice how we only have one Cargo.lock file");
    //That's so we're using the same version of the external dependencies*
    //Even though notice the fact that if we add rand to add_one, we can't use it here
    // let random_number = add_one::rand::random::<f64>();

    //You could pub use, but just put rand in this crate's toml too
    let r_x = rand::thread_rng().gen_range(1..1000);
    println!("RNG says {r_x} becomes {}", add_one::add_random(r_x));

    //* We said that we use the one Cargo.lock file to ensure we only get each dependency once
    //However, if adder depends on rand 0.8.5 and add_one depends on 0.1.0 then uhh
        //yeah you're gonna have to double download


    //Remember the crates in your package are separate crates
    //They can be tested separately and are published separately
    //They also have different gits so that'll be fun to manage :D:D:D:D:D:D:D:D
}
