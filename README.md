# Steganography Fun

This project is a simple CLI tool that will enable users to ether encode secret messages into images, or read messages from an image. I built this to better my understanding of Rust programming and steganography.

Steganography is the practice of representing information within another message or physical object, in such a manner that the presence of the information is not evident to human inspection. It's a fun blend of art and science! For example, I used the code in this project to hide all the lines from William Shakespears Romeo and Juliet in the following image:

<img src="demo\output.png" alt="Alt Text" width="600"/>

## Usage
```
Usage: steganography-fun.exe [OPTIONS] --mode <MODE> --input-image <INPUT_IMAGE> --message-file <MESSAGE_FILE>

Options:
  -m, --mode <MODE>                  If the program should read or hide a message [possible values: read, write]
  -i, --input-image <INPUT_IMAGE>    The path to the image to use [must be .png]
  -o, --output-image <OUTPUT_IMAGE>  The path to output the image [must be in "write" mode, must be .png]
  -f, --message-file <MESSAGE_FILE>  The path to the file with the secret message [must be .txt]
  -h, --help                         Print help
  -V, --version                      Print version
```
To run a `write` mode with `cargo`, simply call `cargo -- -m write i <png you are reading from> -f <text file with a message> -o <output file that will hold the secret message>`. Example:
```
cargo run -- -m write -i demo\William_Shakespeare_by_John_Taylor.png -f demo\romeo-and-juliet.txt -o demo\output.png

[INFO] Writing the message to demo\output.png
[INFO] Image is large enough: message size is 1173680 bytes  and image allows for 29366400 bytes. Percent used: ~4%
[INFO] Encoding the message into the image
[SUCCESS] Image encoded successfully
```

To run a `read` mode with `cargo`, simply call `cargo run -- -m read -i <image with the message in it> -f <text file to write the message to>`. Example:
```
cargo run -- -m read -i demo\output.png -f demo\message.txt

[INFO] Reading the message from demo\output.png
[INFO] Message retrived successfully. Writing to demo\message.txt
[SUCCESS] File written successfully
```

## Building for Release
```
cargo build --release
```

## Credit

Much of this code was built using an article I found on Medium called [Let's Build a Steganography CLI From Scratch](https://betterprogramming.pub/lets-build-an-steganography-cli-from-scratch-f91e80de595c). Credit to much of this code should go to [ACR1209](https://github.com/ACR1209) and their example CLI [rust-steganography](https://github.com/ACR1209/rust-steganography) which is licenced under the GNU General Public License.

## Dependencies
* [clap](https://crates.io/crates/clap)
* [png](https://crates.io/crates/png)