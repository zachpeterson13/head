# head
A worse implementation of gnu head but in rust.

This really sucks right now because I am basically loading the entire file into memory in order to support negative `NUM` values. Its an interesting problem because you want to stream the file and load it line-by-line or byte-by-byte but with this approach you can't know how many lines a file has before hand.

Either way it mostly works...

```
Usage: head [OPTIONS]... [FILE]...
Print the first 10 lines of each FILE to standard output.
With more than on FILE, precede each with a header giving the file name.

With no FILE, or when FILE is -, read standard input.

Arguments:
  [FILE]...  The file(s) to print

Options:
  -c, --bytes <[-]NUM>   Print the first NUM bytes of each file;
                           with the leading '-', print all but the last
                           NUM bytes of each file
  -n, --lines <[-]NUM>   Print the first NUM lines instead of the frist 10;
                           with the leading '-', print all the but the last
                           NUM lines of each file [default: 10]
  -q, --quiet            Never print headers giving file names
  -v, --verbose          Always print headers giving file names
  -z, --zero-terminated  Line delimiter is NUL, not newline
  -h, --help             Print help
  -V, --version          Print version

NUM may have a multiplier suffix:
b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,
GB 1000*1000*1000, G 1024*1024*1024
```
