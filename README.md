
# High Quality Video Converter (MP4 to WebM)

**Video Converter** is a Rust-based tool that converts `.mp4` files to `.webm` format using the `ffmpeg` command. The tool applies optimized settings for video quality and performance, processing multiple files in parallel using multi-threading.

## Features

- **Batch Conversion**: Converts all `.mp4` files in a directory to `.webm` format.
- **Optimized Settings**: Uses the VP9 codec, tunes for PSNR, and applies optimized video and audio bitrate settings.
- **Multi-threading**: Converts multiple files in parallel for better performance.
- **Chunk Processing**: Processes two video files at a time to run on a dual processor laptop. This can be adjusted in the code if you have more cores.

## Requirements

- **ffmpeg**: The program uses `ffmpeg` for video conversion, so make sure it is installed on your system.
- **Rust toolchain**: The tool is written in Rust, so you'll need Rust installed to compile and run the program.

## Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/your-username/video_converter.git
   cd video_converter
   ```

2. **Build the project using Cargo**:

   ```bash
   cargo build --release
   ```

## Usage

1. **Run the program** to convert all `.mp4` files in a directory to `.webm` format:

   ```bash
   ./target/release/video_converter /path/to/directory
   ```

   If no directory is specified, the tool will default to the current directory (`.`).

2. **Example**:

   ```bash
   ./target/release/video_converter /home/user/videos
   ```

   This will find all `.mp4` files in `/home/user/videos` and convert them to `.webm` format.

### Conversion Details:

- **Video codec**: VP9
- **Audio codec**: Opus
- **Bitrate**: Video at 5M, Audio at 192k
- **Quality**: CRF set to 15 for balanced quality
- **Threads**: Utilizes 4 threads for faster processing
- **Tune**: Optimized for PSNR to improve visual quality

### Example Output:

```bash
Processing directory: /home/user/videos
Converting file with optimized settings: "example.mp4"
Conversion complete for file: "example.mp4"
...
All conversions complete for directory: /home/user/videos
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
