# LZ77 algorithm
An implementation of the LZ77 algorithm in rust.

## info
I chose to implement the LZ77 compression algorithm for one of my university classes.
The size of the look-ahead and the search buffer is 1 byte.
The program reads the entire given file and then divides it into chunks, which it then feeds to the algorithm.

This implementation is not fully optimized, but it is functional.

## commands
```
A program for compressing and decompressing files using the LZ77 algorithm

Usage: lz77_project --input <INPUT> --output <OUTPUT> <COMMAND>

Commands:
  singlethreaded  compresses the input file using a single thread
  multithreaded   compresses the input file using multiple threads
  decompress      decompresses the given input file
  help            Print this message or the help of the given subcommand(s)

Options:
  -i, --input <INPUT>    path to a file that the program reads data from
  -o, --output <OUTPUT>  path to a file that the program uses for storing output
  -h, --help             Print help
  -V, --version          Print version
```

***this code should not be used in production. It is not optimized.***
