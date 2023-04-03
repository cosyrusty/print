# print
Cat command in rust. Prints each file to stdout.

## Usage
Concatenate FILE(s) to standard output.
With no FILE, or when FILE is -, read standard input.

- using binary
  ```
  print [FILE]...
  ```
- read stdin as input to print
  ```
  print -
  ```
- using cargo
  ```
  cargo run -- [FILE]...
  ```

## DESCRIPTION

Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

```
-b, --number-nonblank
      number nonempty output lines, overrides -n

-E, --show-ends
      display $ at end of each line

-n, --number
      number all output lines

-s, --squeeze-blank
      suppress repeated empty output lines

-T, --show-tabs
              display TAB characters as ^I

--help display this help and exit

--version
      output version information and exit

```

### EXAMPLES

`cargo run -- f - g`
* Output f's contents, then standard input, then g's contents.

`cargo run --` 
* Copy standard input to standard output.