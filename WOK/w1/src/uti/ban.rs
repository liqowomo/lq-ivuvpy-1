extern crate yansi;
use yansi::Paint;

pub fn ban1() {
    println!(
        "Testing, {}, {}, {}!",
        Paint::new("Ready").bold(),
        Paint::new("Set").black().on_yellow().invert().italic(),
        Paint::new("STOP")
            .white()
            .on_red()
            .bright()
            .underline()
            .bold()
    );
}
