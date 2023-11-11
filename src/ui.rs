use gtk::prelude::*;
use gtk::glib;

#[path="./log.rs"]
mod log;

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("NextToDo"));
    window.set_default_size(600, 600);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let mainbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let title = gtk::Label::builder()
        .use_markup(true)
        .label("<big>LogUnit</big>")
        .build();
    let navbar = gtk::Frame::builder()
        .child(&title)
        .margin_bottom(10)
        .build();

    let list = gtk::ListBox::new();

    let ctrlbox = gtk::Box::builder()
        .spacing(6)
        .homogeneous(true)
        .margin_bottom(12)
        .build();
    let input = gtk::Entry::builder()
        .placeholder_text("Logを入力")
        .build();
    let addbtn = gtk::Button::builder()
        .label("追加する")
        .build();
    addbtn.connect_clicked(glib::clone!(@weak list,@weak input=> move |_|{
        log::add(&list,&input);
    }));

    if let Ok(n) = log::check_conf(){
        log::init(n,&list);
    }
        
    ctrlbox.append(&input);
    ctrlbox.append(&addbtn);

    mainbox.append(&navbar);
    mainbox.append(&ctrlbox);
    mainbox.append(&list);

    vbox.append(&mainbox);
    window.set_child(Some(&vbox));
    window.show();
}