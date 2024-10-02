use crate::tex::e1::print_table; // Update function name
use yansi::Paint as _;

fn main() {
    println!(
        "
{}
{}",
        "Pany".magenta(),
        "Sniff".magenta().blink()
    );
    print_table(); // Call the updated function
}
