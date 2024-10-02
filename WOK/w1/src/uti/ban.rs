use yansi::Paint;

pub fn ban1() {
    println!(
        "Testing, {}, {}, {}!",
        "Ready".bold(),
        "Set".black().on_yellow().invert().italic(),
        "STOP".white().on_red().bright().underline().bold()
    );
}
