// 1
use gtk::prelude::*;

mod ui;

fn main() {
    // 2
    let application =
        gtk::Application::new(Some("com.github.keens.gtk-examples.basic"), Default::default());
    // 3
    application.connect_activate(ui::build_ui);
    // 4
    application.run();
}