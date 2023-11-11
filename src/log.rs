use gtk::prelude::*;
use std::{fs,fs::File,io,io::Write};

fn textadd(text :&String) -> Result<bool,io::Error>{
    let log = fs::read_to_string("log.conf")?;
    let output = format!("{}{}\n",log,text);
    let mut f = File::create("log.conf")?;
    write!(f,"{}",output)?;
    Ok(true)
}

fn error(){
    let mesbox = gtk::MessageDialog::builder()
        .text("ファイルの書き込みに失敗しました")
        .build();
    mesbox.show();
}

pub fn add(list :&gtk::ListBox,input :&gtk::Entry,push :&gtk::Button){
    let val = input.text().to_string();
    if let Ok(_) = textadd(&val){
        let logbox = gtk::Label::builder()
            .use_markup(true)
            .label(format!("<big>{}</big>",val))
            .build();
        list.append(&logbox);
        input.set_text("");
        push.set_sensitive(true);
    }else{
        error();
    }
}

pub fn alldel(list :&gtk::ListBox){
    while let Some(x) = list.first_child(){
        list.remove(&x);
    }
    if let Ok(mut f) = File::create("log.conf"){
        write!(f,"").unwrap();
    }else{
        error();
    }
}