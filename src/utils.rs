/*
Blocking functions that perform various thing that might need to be done more than once
*/
use std::fs::File;
use std::path::Path;
use crate::ParsedArgs;
use std::{io::{self, BufRead}, fs, process::Command};

pub fn read_download_list<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_args(input_args: Vec<String>) -> ParsedArgs
{
  let mut parsed_args = ParsedArgs
  {
    url: "".to_string(),
    filemode: false,
    filename: "".to_string(),
    with_video: false,
    video_only: false,
    as_m4a: false
  };

  for (iter, arg) in input_args.iter().enumerate()
  {
    if iter == 0 {continue;}                         // should be the executable name
    if iter == 2 && parsed_args.filemode {parsed_args.filename = arg.to_string()} // if filemode was specified earlier than filename shoud be here
    if iter == 1 
    {
      if arg == "--file" // if we are in filemode
      {
        parsed_args.filemode = true; // set the switch
        println!("Running in file mode");
        continue; // and terminate iteration early we dont need to check for a url
      } 
      else 
      {    
        parsed_args.url = arg.to_string()
      }
    } // should be the link or id or --file to specify filemode

    if arg == "--with-video" {parsed_args.with_video = true;} // we are downloading MP4s
    if arg == "--video-only" {parsed_args.video_only = true;} // only download the video not the audio
    if arg == "--as-m4a" {parsed_args.as_m4a = true;} // dont perform conversion to MP3
  }
  return parsed_args;
}

pub fn convert_file(filename: &String, converted_file_name: &String)
{
  println!("Converting to mp3...");

  // invoke ffmpeg on the command line to convert it to a proper mp3
  // NOTE: Dont even ask me why i didnt use the bindings crate that thing is a mess
  Command::new("ffmpeg") //program to invoke
  .arg("-i") // add the -i argument
  .arg(filename) // infile
  .arg(converted_file_name) // outfile
  .output() // start process
  .expect("Failed to do convert"); // handle errors

  // remove the temp file
  println!("Cleaning up!");
  fs::remove_file(filename).unwrap();
}
