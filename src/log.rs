use gtk::prelude::*;
use std::{fs,fs::File,io,io::Write};
use chrono::Utc;

fn textadd(text :&String) -> Result<bool,io::Error>{
    let log = fs::read_to_string("log.conf")?;
    let output = format!("{}{}\n",log,text);
    let mut f = File::create("log.conf")?;
    write!(f,"{}",output)?;
    Ok(true)
}

fn error(){
    let mesbox = gtk::MessageDialog::builder()
        .title("error")
        .text("ファイルエラー: 「log.conf」を作成しますか？")
        .buttons(gtk::ButtonsType::YesNo)
        .build();
    mesbox.run_async(|obj,ans|{
        obj.close();
        if ans == gtk::ResponseType::Yes{
            File::create("log.conf").unwrap();
        }
    });
}

fn timeparse(time :&String) -> String{
    let vec :Vec<&str> = time.split("!_!").collect();
    format!("{}/{}/{} {}:{}",vec[0],vec[1],vec[2],vec[3],vec[4])
}

pub fn add(list :&gtk::ListBox,input :&gtk::Entry){
    let val = input.text().to_string();
    let time = Utc::now().format("%Y!_!%m!_!%d!_!%H!_!%M").to_string();
    if let Ok(_) = textadd(&val){
        let logbox = gtk::Label::builder()
            .use_markup(true)
            .label(format!("<big>{}: {}</big>",timeparse(&time),val))
            .build();
        list.append(&logbox);
        input.set_text("");
    }else{
        error();
    }
}

pub fn check_conf() -> Result<Vec<String>,io::Error>{
    let text = fs::read_to_string("log.conf")?;
    let mut line :Vec<String> = text.split("\n").map(|x|x.to_string()).collect();
    line.pop().unwrap();
    Ok(line)
}

pub fn init(line :Vec<String>,list :&gtk::ListBox){
    for v in line{
        let label = gtk::Label::builder()
            .use_markup(true)
            .label(format!("<big>{}</big>",v))
            .build();
        list.append(&label);
    }
}

/*pub fn alldel(list :&gtk::ListBox){
    while let Some(x) = list.first_child(){
        list.remove(&x);
    }
    if let Ok(mut f) = File::create("log.conf"){
        write!(f,"").unwrap();
    }else{
        error();
    }
}*/