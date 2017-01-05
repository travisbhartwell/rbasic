Rust BASIC

-------------------------------------------------------------------------------

A BASIC interpreter in Rust, in the fashion of Apple BASIC or GWBASIC.

# Supported Language Features #

The following features are currently implemented:

  * Support for primitive data types:
    * Integers
    * Strings
    * Boolean values
  * The following operators in expressions
    * +, -, *, / for integers
    * - (unary minus) for integers
    * + or concatenation for strings
    * ! (Boolean not) for Boolean values
    * = (equals), <> (not equals), >, <, >=, <= 
  * Parentheses in expressions
  * Comments with the REM keyword
  * GOTO with line number targets
  * Conditional statements of the form:
    IF expression THEN line number to go to
  * PRINT to print values to the screen
  * INPUT to get input from the keyboard 
  * LET to assign values to variables

# Current Limitations #

Except for unary operators (unary minus and Boolean not) and parentheses, all
operators must be surrounded by spaces to be correctly parsed.

Error handling is limited, and some error messages are swallowed and not
propagated up. Additionally, when positional errors are shown they are shown
from 0-based indices, instead of 1-based which may be more natural.

I'm probably not handling string and integer conversions for all cases in all
operations.

# Running Examples #

Sample working BASIC programs can be found in the [examples](examples/)
directory.

The interpreter takes the filename of the program to run as a parameter.
Using [Cargo](http://doc.crates.io/guide.html), you can run them like the
following, from the main directory:

```shellsession
$ cargo run examples/test1.bas
```

# TODO Items #

This is my first project in Rust, so I'm sure there are a lot of non-idiomatic
things that I have done.  Additionally, particularly in the evaluator, I have a
lot of copy and pasted code that I didn't know how to make cleaner.

In addition to making things more idiomatic and cleaner, I'd like to add the
following features:

  * Floating point numbers
  * Additional operators, like % (modulus)
  * Built-in functions, like:
    * Trigonometric Functions (SIN, COS, TAN)
    * Random number generator (RAND(max value))
    * CHR() and ASC() for dealing with character values
  * Interactive interpreter to give similar experience to Apple BASIC or GWBASIC
   
I also want to increase the tests, outside of the current manual tests in the
`examples` directory and the few unit tests for the lexer. I'd like to make sure
that the error handling is working properly and all errors are reported to the
user. I'd like to also explore generative or property-based testing as well.

# Project History #

I first was exposed to programming when my uncle gave me the book [BASIC
Beginnings](https://www.amazon.com/gp/product/0380837749) and I had access to an
Apple \]\[ computer in my classroom when I was in the fourth grade. Recently, I
was talking about how I got started in tech and the nostalgia came back. I've
been wanting to learn the Rust language, as well as take on a language
implementation project, so this was the perfect unison of those ideas and goals.

I ran into some issues in my understanding of Rust during the project, so I took
the time to rewrite it in Python (see [PyBASIC](https://github.com/travisbhartwell/pybasic)) 
to explore the Shunting Yard algorithm and other ideas. I've since been able to
go back and fully implement things in Rust, but the PyBASIC project is a good
comparison. 

# Resources #

I used the following resources during the implementation:

  * [AppleSoft BASIC Quick Reference](http://www.calormen.com/jsbasic/reference.html)
  * "BASIC Interpreter" chapter of [Developing Applications with Objective Caml](https://caml.inria.fr/pub/docs/oreilly-book/html/book-ora058.html)
  * [Dijkstra's Shunting-Yard Algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) 
   
# License #

This project and source code is licensed under the MIT license:

MIT License

Copyright (c) 2016 Travis B. Hartwell

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
