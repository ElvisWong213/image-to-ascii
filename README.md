# Image To ASCII

## About This Project

Convert Image to ASCII character

## Build And Install

Clone this prject and run the command
```
cargo install --path .
```

## Usage

```
Usage: image-to-ascii [OPTIONS] -i <INPUT_PATH>

Options:
  -i <INPUT_PATH>       Input image path
  -o <OUTPUT_NAME>      Output file name (Optional)
  -w <WIDTH>            Output width (Optional)
  -s                    Save file (Optional)
  -p                    Print out result to terminal (Optional)
  -h, --help            Print help
```

Convert image and save to txt file

```
image-to-ascii -i image.jpg -s
```

Convert image and print the result to terminal

```
image-to-ascii -i image.jpg -p
```

Define output file name
```
image-to-ascii -i image.jpg -o new_file_name -s
```

Define output width
```
image-to-ascii -i image.jpg -s -w 100
```

## Development
```
cargo run -- -i image.jpg
```
