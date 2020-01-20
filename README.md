# derp-rs

A crappy knockoff of "grep" that exists only as a way to teach myself Rust.

I have been paying attention to Rust for years, and have seen it
evolve over time, but never actually tried to write any Rust code.

For personal reasons, it has become of more interest recently, so
this seemed an excellent first program, since it had many dimensions
for a little program.

(Amusingly, [one of the Rust books/online resources][1]
uses building `grep` as an exercise. When I saw that, I avoided
reading that book further, so as to not contaminate my experience.
But it is entertaining that they decided on the same little program
to teach Rust. I must be onto something.)

## Usage
```
USAGE:
    derp [FLAGS] <PATTERN> [PATH]...

FLAGS:
    -d               Sets the level of debugging information
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PATTERN>    The pattern (regular expression) to search for
    <PATH>...    The path to search for the pattern
```

## Why This Is Here

The purpose of the project is, as I said above, to teach myself
Rust. As such, I had some goals, and some non-concerns:

###Goals
* Make `derp` reasonably fully featured. So, full command line parsing
with many options, the ability to specify no paths or many, full
regular expression search.
* Build it in as "difficult" a way as possible, by which I mean build
it to require moving data, errors and state around. That is "difficult"
because of the need to work within the language's value lifetime
system and reference checking.
* Make it as clean, readable & clear as possible.
* Try to be as idiomatic as possible for the language.

### Non-Concerns
* Speed & Efficiency (but if it is built properly that should take care of itself)
* Perfect use of all of Rust and it's idioms. I can't absorb
the whole language in a day, and I am temperamentally incapable
of spending days reading without coding, so I will make mistakes
and miss things that I could have used. And that is OK.
* Doing this right the first time. I know that an experienced
Rust programmer will likely look at my code an grimace. That's
excellent! It means I have an interesting path to follow.

## The Process of Getting Here

I began with an empty `fn main` and began simultaneously reading
online resources and writing code.

At first I just did command line processing, and I very quickly found
a crate that had me covered ([`clap`][2]). That was deceptively simple,
the only mildly difficult thing being determining how to get access
to the optional, potentially multiple `PATH` arguments.

This is where I needed to make architectural decisions.

Having read of the very sensible convention in the Rust community
of putting most of the code in `lib.rs`, and having `main.rs` act as a
client of that library, I decided to structure regular expression
search as a library that the `main` function could make use of to
build a command line application. That, then, suggests that the
`main` code be responsible for printing the matched lines, and
the `lib` be a file searching line matcher.

This has the additional salutary effect of requiring data structures
to be moved around, which means dealing with Rust being Rust and
enforcing referential stricture.

As such, Crate [genawaiter][3]
## Future Work
 * There are *NO DOC COMMENTS*
 * Colo(u)rs using the [ANSI term crate][4]
 * Make to be more OO?

[1]: https://doc.rust-lang.org/book/ch12-00-an-io-project.html
[2]: https://docs.rs/clap/2.33.0/clap/
[3]: http://docs.rs/genawaiter/0.2.2/genawaiter/index.html
[4]: https://rust-lang-nursery.github.io/rust-cookbook/cli/ansi_terminal.html#ansi-terminal