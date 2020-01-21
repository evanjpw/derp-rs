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
* Stick with Rust Stable, to ensure that I am working with the fully
baked language.
* Avoid ARC as much as I can. If I my intent is to write idiomatic code
(what's the equivalent of Pythonic for Rust, RUSTERIFIC?), I would
lean toward preferring stack allocation with fixed lifetimes.

### Non-Concerns
* Speed & Efficiency (but if it is built properly that should take care of itself)
* Perfect use of all of Rust and it's idioms. I can't absorb
the whole language in a day, and I am temperamentally incapable
of spending days reading without coding, so I will make mistakes
and miss things that I could have used. And that is OK.
* Doing this right the first time. I know that an experienced
Rust programmer will likely look at my code an grimace. That's
excellent! It means I have an interesting path to follow.

## The Process of Getting Here (A Boring Story)

Are you certain that you want to read this blather? Maybe. It is
the impressions of someone who was a C programmer, often in very

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

As such, I tried to structure `main.rs` as a client of '`lib.rs`
in a relatively clean way. And it seemed to me that the most sensible
division of labor was to have clients of the search library be
responsible for the display (or alternative use) of search
results from the library.

This militates that there be some clean mechanism for returning
these results. The most brute force such method is for the library
to return some collection that carries the results. This is a
wasteful approach even when done superbly, and it is relatively
simple to do it badly.

An iterator seems superior, but if it masks the afore mentioned collection
it is of little benefit. Some languages, Python for example, have
*generators*. These make it possible to return results as the
search computes them, while maintaining state transparently.

Rust Stable does not have generators, and building them from [stone
knives and bear skins][5] didn't seem reasonable. Fortunately, there was a Crate
for that: [genawaiter][3]. This looked fantastic, and it probably is. But I
was never able to chase out the referential peril that the compiler
protects us from with this approach (probably me being stupid), so I
looked elsewhere.

I decided to bite the bullet and do the brute force thing of
keeping a collection. This quickly became an unpleasant mess,
most likely because (as above) I am (relative to Rust) stupid.

Eventually, I decided to be old school. As a, venerable C programmer,
I tended to structure code to send data _down_ the call stack as
much as possible and to rarely send it _up_ the stack. The obvious
reason for this is that callers up the stack can manage their own
crap, so the callees don't need to worry about allocation and
cleanup.


## Future Work
 * There are *NO DOC COMMENTS*
 * Colo(u)rs using the [ANSI term crate][4]
 * Make to be more OO?
 * Did I mention _DOC COMMENTS_?
 * Unit tests?
 * Integration tests?

[1]: https://doc.rust-lang.org/book/ch12-00-an-io-project.html
[2]: https://docs.rs/clap/2.33.0/clap/
[3]: http://docs.rs/genawaiter/0.2.2/genawaiter/index.html
[4]: https://rust-lang-nursery.github.io/rust-cookbook/cli/ansi_terminal.html#ansi-terminal
[5]: https://images.app.goo.gl/e2gUffHdtj9WUKyZ8
