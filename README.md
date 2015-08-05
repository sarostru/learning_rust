# learning_rust
Rust Tutorials and Small Projects

Most examples are from the [Rust Book] (https://doc.rust-lang.org/book/README.html)

Before digging in, checkout the syntax in the [Closures Section] (https://doc.rust-lang.org/book/closures.html) as they are used implicitly throughout.

##Collection of Links

* [Rust for the Functional Programmer] (http://science.raphael.poss.name/rust-for-functional-programmers.html)
 * No guaranteed tail call optimization, though they want to do it, [postpone discussion] (https://github.com/rust-lang/rfcs/pull/81)
* [Pointers in Rust] (http://words.steveklabnik.com/pointers-in-rust-a-guide), this is a quite complete description
* [Matching Blogpost] (http://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html), pretty essential information on matching
* Blogpost on using the [Option type in rust] (http://hoverbear.org/2014/08/12/option-monads-in-rust/)
 * The comments on the [related reddis thread] (http://www.reddit.com/r/rust/comments/2dnx7k/exploring_the_option_monad_with_rust/) explain something that I was finding confusing.  Rust doesn't have the generic Monad support like Haskell apparently due to a limitation in the current compiler implementation.
* [Macros] (https://doc.rust-lang.org/book/macros.html) in the rust book
* [Syntax Extensions] (https://doc.rust-lang.org/book/compiler-plugins.html) look useful.
 * The roman numeral macro example is nice too
* [Traits] (https://llogiq.github.io/2015/07/30/traits.html), detailed blog post covering all of the built in rust type traits (Haskell type classes)
* [Copy on Write] (https://llogiq.github.io/2015/07/09/cow.html), short blog post showing a use case for the COW functionality
* [Thread Safety] (http://manishearth.github.io/blog/2015/05/30/how-rust-achieves-thread-safety/), detailed post going through how rust achieves thread safety and what that means exactly 
* [Rust is Great, but only if you can pry C++ from my cold dead hands] (http://www.viva64.com/en/b/0324/)
* [Interview with Graydon] (http://www.infoq.com/news/2012/08/Interview-Rust)
 * Best part, Question: "Why would developers choose Rust?", Graydon: "our target audience is frustrated C++ developers."
* [Embedded Rust] (http://www.reddit.com/r/rust/comments/21qogc/im_making_a_note_here_huge_embedded_success/)
* [Rust for ARM] (http://antoinealb.net/programming/2015/05/01/rust-on-arm-microcontroller.html)
