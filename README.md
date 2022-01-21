# Five Letter Words

Find the best words to guess in a game like
[Wordle](https://www.powerlanguage.co.uk/wordle/).

## Installation

    cargo install --path .

## Usage

Command Line Arguments:

    flw --help

Get the best starting words:

    flw | head

The higher-ranked words will be the most likely to narrow the search space.

Narrow the guesses based on hints:

    flw | grep -v -E '[iae]' \
      | grep r | grep t \
      | grep -v -E ' .r...$' \
      | grep -v -E ' ...t.$' \
      | head

This excludes letters that are not in the target word. It includes only
words that have the required letters. It excludes any words that have the
correct letter in the wrong position.

    flw | grep -v -E '[sniae]' \
      | grep r | grep t | grep o \
      | grep -v -E ' .r...$' \
      | grep -v -E ' ...t.$' \
      | grep -E 't$' \
      | grep -v -E ' ..o..$' \
      | grep -v -E ' ...r.$'

In addition to the previous example, this includes only words that have a
required letter in its known position.

## About

[Wordle](https://www.powerlanguage.co.uk/wordle/) is a game by
[Josh Wardle](https://www.powerlanguage.co.uk/). This utility uses
[letter frequencies](https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html)
from the _Concise Oxford Dictionary_ (9th edition, 1995). It assumes you
have list of words available at `/usr/share/dict/words`. To specify an
alternative word list, use the `--word-list` or `-w` argument.

I created this as an exercise for myself. I imagine there are other
implementations. I purposely did not seek out prior art before writing this.

## License

MIT License

Copyright (c) 2022 Carlos Macasaet

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
