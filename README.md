# Intro


A first exploration of Rust. 

A quick tour of testing, creating modules, defining new traits and implementing these for custom types.

My overall impression is that Rust is a very interesting system's language (i.e. does not run on a VM).

The design choice of not relying on a garbage collector but compile time guarantees for memory safety is interesting.

This choice however bring in the language some ceremony such as borrowing that I did not have much time to explore too deeply.

Rust has definitely a more modern feel than other system languages like go considering its type system (e.g. traits and generics) and pervasive functional approach (e.g. immutability and combinators).




# Install and run

Clone this repo. Then assuming you have rust installed, use cargo for running the main app and/or run the tests.
Use the following to run the console app:
```
cargo run
```

Use the following to run the tests:
```
cargo test
```


# Notes on Rust

I found the rust [online book](https://doc.rust-lang.org/book/second-edition/) a good resource to start building this project.
Here follow a few observations about what I learnt using Rust for the first time.

## Code organisation into modules
The organisation of the code in modules is quite straightforward and well explained in the online book. As far as I understand, the whole project is considered a _crate_.
A module is either defined in the ```lib.rs``` file, or in a stand-alone file referenced in the ```lib.rs```, or in a folder that contains a ```mod.rs``` file describing what that module contains (whether code or other sub modules).
In this repo see for instance the ```typeclasses```, ```models``` or ```utils``` modules.

## Traits
I liked the idea of traits, that reminded me more of Haskell's type classes rather than Scala's traits.

Interestingly, Traits cannot be implemented for types that are not part of the project. Let's call these types _foreign_ types (i.e. foreign to your project). 
So for instance to implement a trait defined in your project, a foreign type must be wrapped with a type in your project.
See for instance, the _NationalGDP_ struct that wraps a _u32_. This type was created so that the trait _Colorizable_ could be implemented for _u32_.
This in turn was quite interesting, since it forces the developer to create _Domain_ specific wrappers of foreign types.

I liked the automatic _derive_ traits functionality, very useful for small projects like this one. See for example the _derive(Debug)_ annotation for both the _Color_ and _NationalGDP_ structs.
Once again this seems an idea borrowed from Haskell.

## IDE support

I started from [rust ides](https://forge.rust-lang.org/ides.html), quickly tested the VSCode plugin and them moved to my default IDE Intellij.

At the time of writing, I found Intellij's Rust plugin ([intellij-rust](https://github.com/intellij-rust/intellij-rust)) a bit underwhelming, being used to the great Scala plugin.
Refactoring is quite limited. There is a bit of support around moving modules to folders/files.
Code navigation is quite good regarding the code pertaining your project. But I found myself constantly flipping between the IDE and the documentation for Rust's standard libraries.

Unfortunately Rust itself has a long way to go into helping IDEs providing a better experience for developers.
There's definitely a desire from the community to improve on this. This is explcit in [rust roadmap]()https://github.com/aturon/rfcs/blob/roadmap-2017/text/0000-roadmap-2017.md).
Also the [rust language server source](https://internals.rust-lang.org/t/introducing-rust-language-server-source-release/4209) initiative is trying to improve IDEs Rust support.


## Testing
I haven't explored the testing functionality too deeply. 

It is quite straightforward to create a test suite though. Just create the ```tests``` folder, stick in Rust files in there and annotate any function with ```#test``` and you have a runnable test.
Don't forget to put asserts in there :-)

Basically there are no excuses not to write your software tests.

# Next to try

Borrowing and ownership: Rust's way of guaranteeing compile time memory safety.

Concurrent programming.

Error handling.