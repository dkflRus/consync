use reqwests;
use std::error::Error;

fn main() {
    let paths;
    for q in paths{
        print()
    }
//      print!("{:?}",sumRep()));
}

fn sumRep(path:str)->Result<str,Box<dyn Error>>{
        Ok(reqwests::get(path)?.text()?)
}
