Vigenere cipher from the command line, for whatever use cases you may need.

This is a program that can encrypt any given text file or decrypt an encrypted file from the command line.

Since there is no manual yet here are the flags as of now:

```
-e [file]          sets the file as the target for encrytion
-d [file]          sets the file as the target for decryption
-k                 sets a key for encrypting or decrypting the file
-s                 sets the shift for generating the encryption board
-w [file]          sets the target file to write to
```
There is no help flag because I still didn't make one.

I don't know what will happen if you run -d and -e at the same time and I am too tired to try.
If you find out please let me know.

If you would like to improve on this file the give me a PR and I'll try to figure it out.
Just annotate your code with comments so I don't have to guess what everything does.

Yes the code isn't optimized I made this whole thing by accident.

More features in the works.
