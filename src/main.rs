mod structs;
use structs::ParsedArgs;
use std::{io::{self, BufRead}, process::exit};
use sanitize_filename::sanitize;
use std::{fs, env, path::Path, process::Command};
use rusty_ytdl::{Video, VideoInfo, VideoOptions, VideoQuality, VideoSearchOptions};

#[tokio::main]
async fn main() {
    println!("WBPRO's Youtube Downloader\nCopyright 2023 All rights reserved.");

    let args: ParsedArgs = parse_args(env::args().collect());

    if args.filemode
    {
      // File hosts.txt must exist in the current path
      if let Ok(lines) = read_download_list(args.filename.to_string()) 
      {
        // Consumes the iterator, returns an (Optional) String
        for line in lines 
        {
          if let Ok(url) = line 
          {
            dbg!(&url);
            let (video, video_info) = get_video_info(&url, &args).await;
            download_video(&video, &video_info, &args).await;
            println!("Moving onto next link in list...")
          }
        }
        println!("Done!\nExiting!");
        exit(0) 
      }
    }
    else
    {
      let (video, video_info) = get_video_info(&args.url, &args).await;
      download_video(&video, &video_info, &args).await;
      println!("Done!")
    }

}

async fn get_video_info(url: &String, args: &ParsedArgs) -> (rusty_ytdl::Video, VideoInfo)
{
  let mut video_options = VideoOptions {
    quality: VideoQuality::LowestAudio,
    filter: VideoSearchOptions::Audio,
    ..Default::default()
  };

    if args.with_video
    {
      video_options.quality = VideoQuality::Highest;
      video_options.filter = VideoSearchOptions::VideoAudio;
    }
    else if args.video_only 
    {
      video_options.quality = VideoQuality::Highest;
      video_options.filter = VideoSearchOptions::Video;
    };

    let video = Video::new_with_options(url, video_options).unwrap();
    let video_info = video.get_basic_info().await.unwrap();
    println!("title: {}\nUpload Date: {}",video_info.video_details.title, video_info.video_details.upload_date);
    return (video, video_info);
}

async fn download_video(video: &Video, video_info: &VideoInfo, args: &ParsedArgs)
{
    println!("Downloading Video: {}", video_info.video_details.title);
    // sanitize the title
    let sanitized_title = sanitize(&video_info.video_details.title);
    // do this twice cuz of some weird type conversion issue
    let mut clean_tmp_file_title: String = format!("{}.mp4", sanitized_title);
    // do this thrice cuz command line args
    if args.as_m4a{clean_tmp_file_title = format!("{}.m4a", sanitized_title);}
    // create a path object
    let tmp_file_path: &Path = Path::new(clean_tmp_file_title.as_str());
    // download video
    video.download(tmp_file_path).await.unwrap();
    
    // Yes I know I write if statements like a madman but its 1 thong happening in them I aint wasting 3 line for that
    if args.as_m4a {println!("--as-m4a specified skipping conversion step")}
    else if args.video_only {println!("--video-only specified skipping conversion step")}
    else if args.with_video {println!("--with-video specified skipping conversion step")}
    else{convert_file(&clean_tmp_file_title, &format!("{}.mp3", sanitized_title));}
}

fn convert_file(filename: &String, converted_file_name: &String)
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

fn parse_args(input_args: Vec<String>) -> ParsedArgs
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

fn read_download_list<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}