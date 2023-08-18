mod utils;
mod structs;
mod async_utils;
use structs::ParsedArgs;
use std::{env, process::exit};
use async_utils::{get_video_info, download_video};
use utils::{read_download_list, parse_args, convert_file};

#[tokio::main]
async fn main() {
  /*
  Entrypoint for binary
   */
    println!("WBPRO's Youtube Downloader\nCopyright 2023 All rights reserved.");

    // Get command line pass it to parse_args
    let args: ParsedArgs = parse_args(env::args().collect());

    // If we are running in filemode
    if args.filemode
    {
      /* 
      pass download list's filename to read_download list
      which will return a io::Result<io::Lines<io::BufReader<File>>>
      File must exist in the current path
      */
      if let Ok(lines) = read_download_list(args.filename.to_string()) 
      {
        // Consumes the iterator, returns an (Optional) String
        for line in lines 
        {

          if let Ok(url) = line 
          {
            let (video, video_info) = get_video_info(&url, &args).await;
            download_video(&video, &video_info, &args).await;
            println!("Moving onto next link in list...")
          }

        }

        println!("Done!\nExiting!");
        // We are done exit with code 0
        exit(0) 
      
      }
    }

    else
    {
      /* 
      if we are not running in file mode
      get the url from parsed_args and call download video 
      passing the rest of parsed_arg to it. It will take care of
      parsing command line args that pertain to the download
      */
      let (video, video_info) = get_video_info(&args.url, &args).await;
      download_video(&video, &video_info, &args).await;
      println!("Done!");
      // We are done exit with code 0
      exit(0);
    }
}