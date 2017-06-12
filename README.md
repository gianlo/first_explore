# A first exploration of _Rust_



A quick tour of testing, creating modules, defining new traits and implementing these for custom types.

My overall impression is that _Rust_ is a very interesting systems language (i.e. does not run on a VM).

The design choice of not relying on a garbage collector but compile time guarantees for memory safety is interesting.

This choice however brings in the language some ceremony such as borrowing that I did not have much time to explore too deeply.

_Rust_ has definitely a more modern feel than other recent system languages like _go_ considering its type system (e.g. traits and generics) and pervasive functional approach (e.g. immutability, pattern matching and combinators for collections).




# Install and run

Clone this repo. Then assuming you have _Rust_ installed, use cargo for running the main app and/or run the tests.
Use the following to run the console app:
```
cargo run
```

Use the following to run the tests:
```
cargo test
```


# Notes on _Rust_

I found the _Rust_ [online book](https://doc._Rust_-lang.org/book/second-edition/) a good resource to start building this project.
Here follow a few observations about what I learnt using _Rust_ for the first time.

## Code organisation into modules
The organisation of the code in modules is quite straightforward and well explained in the online book. As far as I understand, the whole project is considered a _crate_.
A module is either defined in the ```lib.rs``` file, or in a stand-alone file referenced in the ```lib.rs```, or in a folder that contains a ```mod.rs``` file describing what that module contains (whether code or other sub modules).
In this repo see for instance the ```typeclasses```, ```models``` or ```utils``` modules.

## Traits
I liked the idea of traits, that reminded me more of _Haskell_'s type classes rather than _Scala_'s traits.

Interestingly, Traits cannot be implemented for types that are not part of the project. Let's call these types _foreign_ types (i.e. foreign to your project). 
So for instance to implement a trait defined in your project, a foreign type must be wrapped with a type in your project.
See for instance, the _NationalGDP_ struct that wraps a _u32_. This type was created so that the trait _Colorizable_ could be implemented for _u32_.
This in turn was quite interesting, since it forces the developer to create _Domain_ specific wrappers of foreign types.

I liked the automatic _derive_ traits functionality, very useful for small projects like this one. See for example the _derive(Debug)_ annotation for both the _Color_ and _NationalGDP_ structs.
Once again this seems an idea borrowed from _Haskell_.

## IDE support

I started from [_Rust_ ides](https://forge._Rust_-lang.org/ides.html), quickly tested the _VSCode_ plugin and them moved to my default IDE: _Intellij_.

At the time of writing, I found _Intellij_'s _Rust_ plugin ([intellij-_Rust_](https://github.com/intellij-_Rust_/intellij-_Rust_)) a bit underwhelming, being used to the great _Scala_ plugin.
Refactoring is quite limited. There is a bit of support around moving modules to folders/files.
Code navigation is quite good regarding the code pertaining your project. But I found myself constantly flipping between the IDE and the documentation for _Rust_'s standard libraries.

Unfortunately _Rust_ itself has a long way to go into helping IDEs providing a better experience for developers.
There's definitely a desire from the community to improve on this. This is explicitly mentioned in the [_Rust_ roadmap](https://github.com/aturon/rfcs/blob/roadmap-2017/text/0000-roadmap-2017.md).
Also the [_Rust_ language server](https://internals._Rust_-lang.org/t/introducing-_Rust_-language-server-source-release/4209) initiative is trying to improve IDEs _Rust_ support.


## Testing
I haven't explored the testing functionality too deeply. 

It is quite straightforward to create a test suite. Just create the top level ```tests``` folder (i.e. same level as ```src```), stick in _Rust_ files in there and annotate any function with ```#test``` and you have a runnable test.
Don't forget to put asserts in there :-) You can use the various available ```assert``` macros.

Basically there are no excuses not to write your software tests.

# Next to try

Borrowing and ownership: _Rust_'s way of guaranteeing compile time memory safety.

Concurrent programming.

Error handling.