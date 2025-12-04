# LZ77 algorithm
An implementation of the LZ77 algorithm in rust.

## info
I chose to implement the LZ77 compression algorithm for one of my university classes.
The size of the look-ahead and the search buffer is 1 byte.
The program reads the entire given file and then divides it into chunks, which it then feeds to the algorithm.

This implementation is not fully optimized, but it is functional.

***this code should not be used in production. It is not optimized.***
