use yansi::Paint as _;

pub fn verse1() {
    println!("{}", "This is verse 1".bold().green())
}

pub fn verse11() {
    println!("{}", "This is verse 11".bold().blink())
}
