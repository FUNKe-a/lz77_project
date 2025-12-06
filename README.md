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
Singlethreaded - compresses the input file using a single thread
multithreaded  - compresses the input file using multiple threads
decompress     - decompresses the given input file


Usage: lz77_project [OPTIONS] --output <OUTPUT> --input <INPUT> <MODE>

Arguments:
  <MODE>  [possible values: singlethreaded, multithreaded, decompress]

Options:
  -o, --output <OUTPUT>          path to a file that the program uses for storing output
  -i, --input <INPUT>            path to a file that the program reads data from
  -c, --chunk-size <CHUNK_SIZE>  size of data chunks in bytes for file processing [default: 65536]
  -h, --help                     Print help
  -V, --version                  Print version
```

***this code should not be used in production. It is not optimized.***
