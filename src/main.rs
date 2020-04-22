extern crate clap;
extern crate flate2;
extern crate tar;

use tar::Archive;
use flate2::read::GzDecoder;
use clap::{Arg, App};
use std::process;
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::path::Path;
use std::path::*;
use std::ffi::OsStr;

fn main() {
    
    let matches = App::new("rus-tar")
                        .version("1.0")
                        .author("abankalarm")
                        .about("easy compressing and decompressing tar")
                        .arg(Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .takes_value(true)
                                .help("A  file"))
                        .arg(Arg::with_name("compress")
                                .short("c")
                                .long("compress")
                                .takes_value(false)
                                .help("compress a file into tar"))
                        .arg(Arg::with_name("decompress")
                                .short("d")
                                .long("decompress")
                                .takes_value(false)
                                .help("decompress a file from tar"))
                        .get_matches();

    let path = matches.value_of("file").unwrap();
    let filepath = Path::new(path);
    let parent = filepath.parent();
    let file_stem = filepath.file_stem();
    print!("{:?}{:?}",parent,file_stem);
    
    

    
    let dc = matches.value_of("decompress");
    println!("{:?}",dc);

    let c = matches.value_of("compress");
    println!("{:?}",c);
    
    let myfile = File::open(path).unwrap();

    if dc != None{

        let tar= GzDecoder::new(myfile);
        let mut archive = Archive::new(tar);
        archive.unpack(".").unwrap();
        process::exit(1);
    }
   
    if c != None{
        let tar_gz = File::create("archive.tar.gz").unwrap();
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        let x = format!("./{}", path);
        tar.append_dir_all(x, filepath).unwrap();
    }
    



}
