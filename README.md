# BIP-39 Answer Desk

**BIP-39 Answer Desk** is a lightweight Rust library and CLI tool that allows you to interact with the BIP-39 wordlist. It can convert between binary, decimal, and BIP-39 words efficiently.

## Features
- Convert **binary to word**.
- Convert **decimal to word**.
- Convert **word to binary**.
- Convert **word to decimal**.

## Usage
```bash
$ cargo run -- <COMMAND> [OPTIONS]
```

### Commands
1. **Binary to Word (`bw`)**
   ```bash
   $ cargo run -- bw 00000000000
   Word: abandon
   ```

2. **Decimal to Word (`dw`)**
   ```bash
   $ cargo run -- dw 5
   Word: absent
   ```

3. **Word to Binary (`wb`)**
   ```bash
   $ cargo run -- wb abandon
   Binary: 00000000000
   ```

4. **Word to Decimal (`wd`)**
   ```bash
   $ cargo run -- wd abandon
   Decimal: 0
   ```

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/rghollenbeck/bip39_answer_desk.git
   ```
2. Navigate to the project directory:
   ```bash
   cd bip39_answer_desk
   ```
3. Build the project:
   ```bash
   cargo build
   ```

### License
[MIT License](LICENSE)

### Contributions
Feel free to submit pull requests or open issues to contribute to the project.

---

# BIP-39_ANSWER_DESK(1) User Commands BIP-39_ANSWER_DESK(1)

NAME
    bip39_answer_desk - A CLI tool for BIP-39 wordlist conversion

SYNOPSIS
    bip39_answer_desk <COMMAND> [OPTIONS]

DESCRIPTION
    The bip39_answer_desk CLI allows you to convert between BIP-39 wordlist values and their binary or decimal representations.

COMMANDS
    bw <BINARY>       Convert binary to a BIP-39 word.
    dw <DECIMAL>      Convert decimal to a BIP-39 word.
    wb <WORD>         Convert a BIP-39 word to binary.
    wd <WORD>         Convert a BIP-39 word to decimal.

EXAMPLES
    Convert binary to a word:
        $ bip39_answer_desk bw 00000000000
        Word: abandon

    Convert decimal to a word:
        $ bip39_answer_desk dw 5
        Word: absent

    Convert word to binary:
        $ bip39_answer_desk wb abandon
        Binary: 00000000000

    Convert word to decimal:
        $ bip39_answer_desk wd abandon
        Decimal: 0

AUTHOR
    Written by Rich Hollenbeck.

COPYRIGHT
    This is free software; see the source for copying conditions.
