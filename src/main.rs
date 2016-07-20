extern crate argparse;
extern crate byteorder;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

use argparse::{ArgumentParser, StoreTrue, Store};
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};


fn main() {
    let mut verbose = false;
    //default location of SR2 settings
    let mut settings_file = ".local/share/volition/saintsrow2/AppData/Saints Row 2/settings.dat".to_string();
    let mut x : u32 = 800;
    let mut y : u32 = 600;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Change the resolution of Saints Row 2 to unsupported resolutions.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut settings_file)
            .add_option(&["-f", "--file"], Store, "Settings file to change");
        ap.refer(&mut x)
            .add_option(&["-x", "--xRes"], Store, "Horizonal resolution").required();
        ap.refer(&mut y)
            .add_option(&["-y", "--yRes"], Store, "Vertical resolution").required();
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("settings_file is {}", settings_file);
        println!("x is {}", x);
        println!("y is {}", y);
    }
    
    let filepathstr = "./".to_string() + &settings_file;
    let filepath = Path::new(&filepathstr);

    //make a backup of old settings, in case something breaks
    match fs::copy(&filepath, "settings.dat.backup") {
        Err(why) => panic!("couldn't backup old settings: {}", why.description()),
        Ok(_) => println!("Wrote backup of settings file to setting.dat.backup")
    }


    let mut fd = match OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .open(&filepath) {
        Err(why) => panic!("couldn't read {}: {}", filepath.display(), why.description()),
        Ok(file) => file
    }; 


    let mut res_bytes = vec![];
    res_bytes.write_u32::<LittleEndian>(x); 
    res_bytes.write_u32::<LittleEndian>(y); 

    if verbose {
        println!("Writing {} to file.", to_hex_string(&res_bytes)); 
    }

    let x_start = 0x3C;
    fd.seek(SeekFrom::Start(x_start)); 

    match fd.write(&res_bytes) {
        Err(why) => panic!("Could not write to file : {}", why.description()),
        Ok(_) => println!("Wrote new resolution to file")
    }
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
  let strs : Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.join(" ")
}
