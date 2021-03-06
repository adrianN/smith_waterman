[![Build Status](https://travis-ci.org/adrianN/smith_waterman.svg?branch=master)](https://travis-ci.org/adrianN/smith_waterman)

Smith-Waterman
==============

This is a simple implementation of the Smith-Waterman algorithm (or something like it) for sequence alingment.
It works on byte-sequences. Unicode would be nice, but is not needed for my use case and seems to be a lot of
trouble to get right.

I want to use for matching file paths in a fuzzy-finding application, so unlike bioinformatics applications I
don't have similarity matrix for letters. Either two bytes match, or they don't. Matches get a bonus when they
occur at the start of words (e.g. after a space or a slash).

Since I want to use this for a fuzzy finder, characters are processed one by one and it is possible to delete 
letters from the search string without having to recompute everything.

Usage
-----

Run `cargo doc` to generate some documentation, or check the comments in the code. The tldr is that you create
a `Matcher` struct and add pattern characters with `add_pchar`. You query the score with `score`.

You can also check out [MovieThingy](https://github.com/adrianN/MovieThingy), the tool for which I use this.

Licence
-------

This is released under AGPL. See the LICENCE file.

Contributing
------------

Feel free to open pull requests. Only contribute code to this project that won't get me in trouble, e.g. because 
of patents, or because it's not your own creation that you're submitting, or your employer claims rights to everything
your code. Any code you contribute must be under the same license as this repository.
