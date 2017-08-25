# language-x

(A temporary placeholder name until I think of a better one.)

An experimental programming language project to investigate what a spiritual
successor to Java and C# might look like.


## Overview

Java and C# helped to move C++ application programmers away from direct memory
management onto managed abstract machines with concurrency, networking,
serialisation, and portability built-in.

What will be the next language and runtime after Java and .NET? Better
distributed and concurrent programming support would be a good starting point.
No nulls, ubiquitous immutability, no concrete inheritance, supervisor trees in
the standard library, and transparent asynchronous IO would acknowledge this era
of computing.

Hardware is no longer speeding up exponentially year-on-year and is becoming
increasingly parallel. Performance cannot be discarded out of hand like the rise
of dynamic languages throughout the previous decade. Reference types,
reflection, and dynamic typing are expensive, so they should be an opt-in rather
than the default.

Large semantic changes will likely be rejected by developers if not presented in
a syntax similar to existing languages; like Java took C++'s syntax, the next
language should take Java's. 


## TODO
  
While experimenting with the language, it will piggyback off another by being
translated into it from the parse tree. After that, we can use it to implement
itself.

* [X] Lex with Java.
* [ ] Parse with Java.
* [ ] Translate parse tree to Java to bootstrap the language.
* [ ] Rewrite the lexer and parser in itself.
* [ ] Wean translated code's dependency off the JVM by implementing
      runtime features in the language itself.
* [ ] Provide enough low-level features in language to allow translation to
      native code without forcing the GC or lightweight tasks, so that they
      can be implemented in the language itself.
* [ ] Once translatable to native code, leverage native libraries like
      libevent.
      

## The Language Proposal so Far

```
package main

// A single line comment.

/*
  A multiline comment.

  /*
    A nested multiline comment.
  */
*/

import io.{println, print}

interface ToString {
    public String toString()
}

interface Concatenate<T, Result = Self> {
    public Result concatenate(T y)
}

class Account implements ToString, Concatenate<Account> {
    public String firstName
    public String lastName
    public int ageInYears

    public Account(String firstName, String lastName) {
        println("instantiating an Account...")
        super(.firstName, .lastName, ageInYears = 35)
    }

    public override String toString() {
        "%s %s is %d years old".format(
                firstName,
                lastName,
                ageInYears
        )
    }

    public override Account concatenate(Account a) {
        var firstName = firstName.concat(a.firstName)
        var lastName = lastName.concat(a.lastName)

        Account(
            .firstName,
            .lastName,
            ageInYears = ageInYears + a.ageInYears,
        )
    }

    public String get name() {
        `{firstName} {lastName}`
    }
}

extends class Account implements Concatenate<Account, Result = String> {
    public override String concatenate(Account a) {
        `{firstName} {a.firstName}`
    }
}

class Person = Account
interface Showable = ToString

int maxBound = 5

int factorial(int n) {
    switch n {
        case 0, 1:
            1
        default:
            if n < 0 {
                throw Exception("n cannot be less than 0")
            }
            factorial(n * (n - 1))
    }
}

package counter {
    public class Increment { }
    public class Reset { }
    public class Get { }

    public Task create(int n = 0) {
        Task(() -> {
            for {
                select _ {
                    case Increment:
                        Counter(n + 1)
                    case Reset:
                        Counter(0)
                    case Get:
                        sender.send(n)
                    timeout seconds(10):
                        throw Exception("timed out!")
                }
            }
       })
    }
}

void allocationAndClosureDemo() {
    var x = 5

    var account1 = Account(
            firstName = "Tom",
            lastName = "Smith",
            ageInYears = 15
    )

    var firstName = "Tom"
    var lastName = "Smith"
    var age = 25
    var account2 = Account(.firstName, .lastName, ageInYears = age)

    var f = (a) -> {
        println(a.toString())
    }

    f(account1)
    f(account2(first_name = "Emma"))

    var g = (a) -> {
        println("returning an account")
        a
    }

    var z = g(account1)
}

void demoNumericLiterals() {
    int a = 5
    uint b = 5
    decimal c = 10.0

    byte d = 5u8
    uint16 e = 11u16
    uint32 f = 12u32
    uint64 g = 13u64
    int8 h = 15s8
    short i = 13s16
    int32 j = 7s32
    long k = 7s64
    float l = 12f16
    double m = 8f32
}

N double<N extends Add>(N n) N {
    n + n
}

void demoIteration() {
    List(1, 2, 3).forEach((n) -> {
        println(`{n}`)
    })

    List(1, 2, 3).map(double)

    var fact = for n = 20, result = 0 {
        if n <= 0 {
            result
        } else  {
            continue(n - 1, n * result)
        }
    }
    println(`factorial: {fact}`)
}

Optional<int> demoContexts() {
    do {
        var a <- some(5)
        doSomething()
        var b <- empty()
        willNotBeRun()
    }
}

void main() {
    var c = counter.create()
    times(5, () -> {
        c.send(counter.Increment())
    })

    c.send(counter.Get())
    c.send(counter.Increment())
    c.send(counter.Get())

    times(2, () -> {
        select n {
            case Int:
                println(`{n}`)
        }
    })

    var x = {
        println("Getting 5...")
        println("Setting it to x...")
        println("Set!")
        5
    }
    print(`{x}`)
}
```


## Goals

* Look as syntactically similar to Java and C# as possible.
* Support mixed-ability teams by not adding footguns or abstractions that do not
  scale; powerful features should have very little action-at-a-distance.
* Use null-free static types and increase type-system expressiveness over Java
  and C#.
* Make compiler and other components easy to work with; make tool and IDE
  integration as easy as possible. Perhaps an FFI into the final parser and
  compiler and an initial Language Server Protocol implementation.
* Easy distribution of applications; compile directly to native code, but use an
  easier method for the initial bootstrapped version.
* Use ubiquitous immutability to reduce unnecessary side-effects and coupling;
  invalid states should be unrepresentable.
* Allow distributed programming with message-passing.
* Transparently handle asynchronous IO.
* Make tasks cheap, preemptive, and killable; tasks should be a useful
  abstraction, not like threads which are a low-level OS-provided feature with
  endless edge cases.
* Remove or fix error-prone features from Java and C#, like
  assignments-as-expressions, pre and post decrement and increment, nulls,
  concrete inheritance, pervasive mutability, type erasure, statics,
  primitives and autoboxing, default memory sharing across tasks, and in-task
  catchable exceptions.
* Non-overflowable arithmetic should be default; machine-width arithmetic as an
  opt-in for performance.
* Encourage compile-time metaprogramming over runtime annotation and reflection;
  design it to scale in the large without becoming cryptic.
* Be mostly expression-based and improve conditional matching support.
* Focus less on reference semantics and more on value semantics; combined with
  proper type reification, data-heavy code should be more cache friendly and
  accesses with fewer indirections.
* Guarantee tail-call elimination and return-value optimisation.
  

## Detailed Proposals

Accessibility levels:
- Public, internal, and private; only public and internal have keywords.
- Private level is default.

Types:
* Built-ins and user-defined.
* No difference from the user's perspective between them except for literal
  support and built-ins being predefined by the compiler and runtime.
* Final classes and trait-like interfaces. No concrete inheritance or
  abstract classes.
* Constructors are special; this is done to allow function-style
  instantiations while avoiding things like statics, needing to detach
  constructors from class definitions, or having a more complicated
  initialisation syntax.
* `void` is an actual type, like `()` in Haskell. Defining a method as
  returning `void` is a special-case that discards the result of final
  non-void expression and returns the void value instead. This avoids
  special-cases when composing functions in various ways.
* `super` is to either disambiguate which interface's method to delegate to,
  or to fallback to the auto-generated constructor in user-defined
  constructors. It does not deal with concrete class inheritance.
* Generics like C#, as in no type erasure.
* Support higher-kinded types, but keep an eye on projects like Dotty to
  see what type-soundness issues they encounter. Perhaps implement a more
  restricted version of it.

Matching in switch and select:
* `switch` matches on value and supports multiple clauses with commas.
* `select` matches on type, and each branch handles the message variable as
  that type. This is like Erlang's `receive` and Go's type switch rolled
  into one. `timeout` clauses are available.

Invoking methods:
* Methods, classes, and objects can be invoked with `()`.
* Invoking a defined method does as one would expect; invoking a class
  constructs an instance; invoking a object allows non-destructive updates.
* Arguments can have default values.
* Any argument can be invoked as either positional or keyword; it's up to the
  caller.
* Prefixing an argument with a dot is a shortcut for assigning a keyword
  argument from a binding of the same name, e.g. `Account(.firstName)` is
  `Account(firstName = firstName)`.

Language versioning:
* Keyword `v` can start a file.
* Has a version after it, e.g. `v1.0`
* If not present, assume to be the earliest stable release of the language.

Compile-time metaprogramming:
* No `constexpr`, templating, or `static if`. Should be the same language 
  as runtime.
* Derive from Lisp and Jai but reduce footguns like automatic variable
  captures.
* Do not copy D or C++.
* Will eliminate the need for reflection.
* What are the security implications of running arbitrary code from the
  compiler?
* CL's `defmacro` is too low-level; a Java-like annotation syntax could be
  used for a more controlled subset, perhaps hygienic macro system a la
  Scheme.

Runtime structure information:
* No reflection.
* No runtime annotations.
* Use compile-time programming instead.
* Reduces magic, as compile-time metaprogramming cannot happen at random
  points during a running application unless `eval` exists.
* Improves performance as metaprogramming is done at compile-time.
* Reduces native binary size, since such information does not need to be
  bundled.

The compiler:
* Write it in itself, so no runtime needed to run compiler or REPL.
* Bootstrap the first version by translating to Java.
* Slowly reimplement needed runtime features in the language itself; removing
  ties from the JVM to allow translation to lower-level forms.
* Use LLVM to compile to native code in the proper version.
* The GC and runtime system will need to be disabled for the implementing the
  runtime in itself. Access to native APIs like `malloc` and `free` will be
  needed.
* Consider the interplay between userland preemptive switching and native
  compilation. If the VM must manage opcode execution counts, it might _need_
  to be an interpreter. Could we add yield points into generated code? Go's
  approach might be more pragmatic here, but its loss of in-function
  preemption and goroutine stopping is an issue.
  
The VM:
* Non-existent for the first bootstrapping version of the language. Just
  borrow the JVM.
* It will probably be heavily BEAM-inspired.
* Must do tail call elimination.
* No mutable data except the execution of tasks over time.
* Lightweight processes. Immutability means changes can only be sent via
  messages.
* Initial toy implementation to use JVM threads. Real implementation can
  use userland scheduler with remote process support.
* To handle remote processes, Tasks need node IDs.
* ...and nodes need network addresses or localhost.
* Per-task GC in theory; probably global GC in first implementation for
  simplicity. (Perhaps that's OK if only a single task's world gets stopped
  at any time.)
* Look at leveraging existing GCs via native interop, like Boehm-GC. However,
  they might be unsuitable for many lightweight tasks collecting
  concurrently.
* Persistent data structures.
* Async IO; use a library like libevent. OK to block in the prototype, but
  don't add any language features that break compatibility with async in
  the proper version.

The build system:
* Go-style; just derive information from the source files rather than using
  separate configurations.
* If we must have config files, consider TOML.

Interop:
* Lightweight tasks will be awkward with POSIX/WinNT threads for native
  interop; see Erlang and Go's issues here. Not sure of a better alternative
  if we're using userland stacks and scheduling.

Standard lib:
* Standard lib should be modular, like Java 9's JRE. Implementations can
  opt-in to each, similar to C11 features like VLAs.
  
To consider later on: 
* Tuples.
* Destructuring composite types.
* Sequence destructuring.
* Easier sum types; could be existing interface/class mechanism combined with
  `switch` enhancements.
* Parameterisable packages, perhaps a less powerful version of ML functors.
* Structs for stack-allocated value types.
* Unsafe mode for writing its own runtime: native implementations, access to
  C libraries like `malloc` and `free`, copying from structs to arbitrary
  heap memory addresses.
* Matrix operations to implement for user types, even if builtins do not use
  them. See Python 3 for a version of this.
* Variadics solely to deal with function forwarding; or could this be done
  differently?