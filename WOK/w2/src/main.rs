mod f1 {
    pub mod f1;
}

mod f2 {
    pub mod f21 {
        pub mod f21;
    }
    pub mod f2;
}

fn main() {
    println!("Hello, world!");
    f1::f1::f1func();
    f2::f21::f21::f2f21();
    f2::f2::f2func();
}
