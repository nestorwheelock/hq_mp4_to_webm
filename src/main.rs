use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};
use std::thread;

fn convert_video_to_webm(path: &Path) -> Result<(), std::io::Error> {
    let output_file = path.with_extension("webm");

    // Adjusted settings to reduce pixelation and smearing
    println!("Converting file with optimized settings: {:?}", path);
    let status = Command::new("ffmpeg")
        .args(&[
            "-i", path.to_str().unwrap(),  // Input file
            "-c:v", "libvpx-vp9",          // Use VP9 codec
            "-b:v", "5M",                  // Adjusted video bitrate to 5M
            "-crf", "15",                  // CRF set to 15 for better balance
            "-pix_fmt", "yuv420p",         // Pixel format
            "-g", "120",                   // Keyframe interval set to 120
            "-qmin", "10",                 // Minimum quantizer set to 10
            "-qmax", "42",                 // Maximum quantizer set to 42
            "-threads", "4",               // Use multiple threads
            "-c:a", "libopus",             // Audio codec
            "-b:a", "192k",                // Audio bitrate at 192k
            "-tune", "psnr",               // Tune for PSNR for better visual quality
            output_file.to_str().unwrap()  // Output file
        ])
        .status()?;
    
    if !status.success() {
        eprintln!("ffmpeg failed to convert file: {:?}", path);
        exit(1);
    } else {
        println!("Conversion complete for file: {:?}", path);
    }

    Ok(())
}

fn convert_videos_to_webm(dir_name: &str) -> Result<(), std::io::Error> {
    let entries: Vec<_> = fs::read_dir(dir_name)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file() && entry.path().extension().unwrap_or_default() == "mp4")
        .collect();

    if entries.is_empty() {
        println!("No .mp4 files found in directory: {}", dir_name);
        return Ok(());
    }

    println!("Processing directory: {}", dir_name);

    let chunks = entries.chunks(2);  // Process 2 files at a time

    for chunk in chunks {
        let handles: Vec<_> = chunk.iter().map(|entry| {
            let path = entry.path();
            thread::spawn(move || {
                if let Err(e) = convert_video_to_webm(&path) {
                    eprintln!("Failed to convert file: {:?}, error: {:?}", path, e);
                }
            })
        }).collect();

        // Wait for both threads to finish before proceeding to the next pair
        for handle in handles {
            let _ = handle.join();
        }
    }

    println!("All conversions complete for directory: {}", dir_name);

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let dir_name = if args.len() > 1 {
        &args[1]
    } else {
        "."  // Default to current directory
    };

    println!("Processing directory: {}", dir_name);
    convert_videos_to_webm(dir_name)?;

    println!("All conversions complete.");

    Ok(())
}

