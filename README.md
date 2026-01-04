Vigenere cipher from the command line, for whatever encoding and decoding cases you may need.

This is a program that can encrypt any given text file or decrypt an encrypted file from the command line.

To encode or decode a file you will need to provide an alphabet, a key, and a destination file.

These options can be passed using the following flags:

```
Usage: v1 -a ALPHABET.txt -k KEY -w WRITE.txt [OPTIONS...]

Options:
    -a, --alphabet  ALPHABET.txt    Alphabet that will be used to encode/decode the file
    -k, --key       KEY             Key that will be used to encode/decode the file
    -w, --write     WRITE.txt       Destination for output

    -e, --encode    ENCODE.txt      The file that will be encoded
    -d, --decode    DECODE.txt      The file that will be decoded

    -p, --print                     Prints the board used for encoding/decoding to the terminal

    -h, --help                      Displays usage options
```

I don't know what will happen if you run -d and -e at the same time and I am too tired to try.
If you find out please let me know.

If you would like to improve on this file the give me a PR and I'll try to figure it out.
Just annotate your code with comments so I don't have to guess what everything does.

Yes the code isn't optimized I made this whole thing by accident.

More features in the works.
