# look - Utility for searching content in files 

look - utility for searching strings in files, located in the content specified in the command template. There are many options for use.

## Manifesto

Look is a Grep analogue written in Rust. It has fewer functions, is more optimized and has a smaller code volume. It is written quite declaratively and modularly.

## Installation

```bash
git clone https://github.com/Ad4ndi/look
cd look
cargo build
```

## Usage
```
  -i  Ignore case
  -q  Quiet mode (exit with 0 if match found)
  -o  Show only matching part
  -c  Count matching lines
  -v  Invert match (show non-matching lines)
  -n  Show line numbers
  -r  Recursive directory search
  -l  Show only names of files with matches
  -L  Show only names of files without matches
  -h  Show this help
```
