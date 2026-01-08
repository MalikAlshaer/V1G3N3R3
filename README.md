Vigenere cipher from the command line, for whatever encoding and decoding cases you may need.

This is a program that can encrypt any given text file or decrypt an encrypted file from the command line.

To encode or decode a file you will need to provide a read file, a write file, an alphabet text file, and a key.

These options can be passed using the following flags:

```
Usage: v1 -r READ.txt -w WRITE.txt -a ALPHABET.txt -k KEY [OPTIONS...]

Options:
    -a, --alphabet  ALPHABET.txt    Alphabet that will be used to encode/decode the file
    -k, --key       KEY             Key that will be used to encode/decode the file
    -r, --read      READ.txt        Target file for encoding/decoding
    -w, --write     WRITE.txt       Destination for output

    -e, --encode                    Encode read file
    -d, --decode                    Decode read file

    -p, --print                     Prints the board generated for encoding/decoding

    -h, --help                      Displays usage options
```

If you have any suggestions for code optimization please don't hesitate to reach out.

TUI in the works.
