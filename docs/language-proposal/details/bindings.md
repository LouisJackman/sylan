# Bindings

* Types, variables, functions are all in the same namespace. Although types and
  values exist in two different phases, compile-time and runtime respectively,
  Sylan still won't allow identifiers between the two to clash.
* Types should start with capital letters, and values with lowercase letters.
* Methods are namespaced to their types, although just deeper in the namespacing
  hierarchy rather than in a completely different standalone global namespace.
* Shadowing is not allowed in the same block except for pseudoidentifiers, which
  use keyphrases. Shadowing is allowed between different blocks; a particular
  example is methods being able to shadow package-wide functions of the same
  name. Explicitly specifying the package lets subpackages and classes
  disambiguate which identifier they mean, which the `this.package`
  pseudoidentifier can help with.
* There are nine psuedoidentifiers: `...`, `_`, `continue`, `it`, `this`, `This`
  `this.module`, `this.package`, and `super`. `continue` and `it` are _almost_
  dynamically scoped, changing implicitly throughout scopes based on the
  context. `continue` binds to the innermost non-labelled `for` iteration
  function, `it` is the innermost syntactically-zero-parameter lambda's sole
  parameter, and `_` is an ignored value in a binding and a partial-application
  notation for the innermost invocation. `this` refers to the current object,
  `This` to the current type, `this.module` to the current module, and
  `this.package` to the current package.
* Types and variables can both be thought of as "bindings", just one at
  compile-time and another at runtime. Never the twain shall meet, at least
  until Sylan designs how they should interoperate if at all. This will depend
  on how compile-time metaprogramming is implemented and whether Sylan decides
  to implement any form of dependent typing.
* Classes ultimately can only expose methods and nothing else. As field
  declarations are always autogenerated getters.
* There are also _dynamically-scoped_ variables. They are declared in any
  package with the `bind final` or `bind var` forms.
* They are not allowed in functions or methods; they belong to packages. They
  can, however, be _rebound_ in functions or methods.
* They can be rebound with `bind`, which changes the whole value for the scope
  _dynamically_, i.e. even for any callstack going through it. These rebound
  values can be reset to what they used to be with `unbind`.
* Final dynamic bindings a dynamically bound to the _compile-time_ call stack,
  i.e. the flow of code as it goes through the sourcing phase, the lexing phase,
  and the parsing phase. This allows the variable to control how macros expand
  code and alter compile-time programming.
* Binding changes their value for a callstack, but they still conceptually
  belong to the package which defined them; they must still be accessed as such.
* Because they can be checked at compile time to change emitted code, such as
  generating different calls back into the runtime, or in macros, they don't
  themselves exist at runtime but can still drastically change how code ends up
  running.
* They are used to push and pop readtables for reader macros, and for other
  environment changes. Potentials could be be temporarily disabling garbage
  collection for compiled item instantiations in a callstack, not emitting task
  yield points, or temporarily allowing unchecked arithmetic wraparounds.
* Modules must whitelist which dynamically-scoped variables it allows rebinding
  within. This means a module, by default, can block hypothetical features such
  as unsafe code, unchecked array bounds access, reader macros, and
  externally-defined code. Given how risky yet powerful these features are, it
  really is for the best.
