mod uti {
    pub mod ban;
}

fn main() {
    println!("Hello, world!");
    uti::ban::ban1(); // Call the ban1 function from ban.rs file using the ban sub-module within the uti module.
}
