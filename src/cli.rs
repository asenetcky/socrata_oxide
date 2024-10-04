use crate::data::*;
use anyhow::Result;
use clap::Parser;
use polars::prelude::*;
use polars_io::ipc::IpcWriter;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// My Rust version of RSocrata written as a cli app
pub struct Args {
    /// URL
    #[arg(value_name = "URL")]
    dataset_url: String, // work on vectors later

    /// Api Key
    #[arg(short = 'k', long = "key", default_value = "-", value_name = "API_KEY")]
    api_key: String,

    /// Output File
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Username
    #[arg(
        short = 'n',
        long = "username",
        default_value = "-",
        value_name = "USERNAME"
    )]
    username: String,

    /// Password
    #[arg(
        short = 'p',
        long = "password",
        default_value = "-",
        value_name = "PASSWORD"
    )]
    password: String,
    // maybe some helpers for the page vs row number attribute in the api
    // ^ in case folks dont want everything by default
}

pub fn run(args: Args) -> Result<()> {
    let url = args.dataset_url;
    let _api_key = args.api_key;
    let _username = args.username;
    let _password = args.password;

    // let output = OutFile::new(args.out_file);
    // let mut data = Data::new(&url)?;

    let lim_off = LimitOffset::new(&url);
    println!("{:?}", lim_off);

    // match output.out_type {
    //     OutType::Arrow => {
    //         let filename = output.file_name.unwrap().to_string();
    //         let mut file = File::create(filename).expect("could not create file");
    //         IpcWriter::new(&mut file).finish(&mut data.df)?;
    //     }
    //     OutType::Csv => {
    //         let filename = output.file_name.unwrap().to_string();
    //         let mut file = File::create(filename).expect("could not create file");
    //         CsvWriter::new(&mut file).finish(&mut data.df)?;
    //     }
    //     OutType::Stdout => {
    //         println!("{:?}", data.df);
    //     }
    // }
    Ok(())
}
