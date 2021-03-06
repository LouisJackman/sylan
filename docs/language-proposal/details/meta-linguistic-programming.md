# Meta-linguistic Programming

* If a parameter is prefixed with `syntax`, it takes the quoted code as the
  argument rather than the result of evaluating it. These parameters can be one
  of three types: `Pipeline`, `AstPipeline`, and `AsymmetricPipeline`. It can
  only take these variants (default type arguments omitted for brevity):
  - `AstPipeline`
  - `AstPipeline[of: ParameterAst]`
  - `Pipeline[of: TokenTree]`
  - `Pipeline[of: Char]`
  - `Pipeline[of: TokenTree, readingFrom: ParameterTokenReader]`
  - `Pipeline[of: Char,      readingFrom: ParameterCharReader ]`
  - `AsymmetricPipeline[from: TokenReader,          to: AstWriter  ]`
  - `AsymmetricPipeline[from: CharReader,           to: AstWriter  ]`
  - `AsymmetricPipeline[from: CharReader,           to: TokenWriter]`
  - `AsymmetricPipeline[from: ParameterTokenReader, to: AstWriter  ]`
  - `AsymmetricPipeline[from: ParameterChar,        to: AstWriter  ]`
  - `AsymmetricPipeline[from: ParameterCharReader,  to: TokenWriter]`
* `syntax` is banned from non-`final` functions. Compiled artefacts are never
  burdened with the weight of meta-linguistic abstrations at runtime.
* A function taking a `syntax` parameter must return a `Throws` type.
* Therefore, the `!` for forcing a function at compile time doesn't suddenly
  allow a runtime function to work as a macro. This is intended.
* A final function with a parameter pertaining to ASTs is called a
  _pattern macro_. One that uses parameters receiving tokens is a _procedural
  macro_, and one receiving characters is called a _reader macro_. One mixing
  different pipelines takes the name of the most low-level pipeline, where
  characters are lower level than tokens, which are in turn lower level than
  ASTs.
* An AST has two fields: scope and data. Data is a raw AST data structure; scope
  is the scope automatically attached to it on definition.
* As reader and procedural macros can _produce_ ASTs even if they don't consume
  them, this gives ample oppertunity for macros to be hygenic. Producing raw
  token or character streams do not produce syntax scopes; they must manually
  use `gensym`.
* Producing raw token and character streams should be quite rare. There's no
  reason to do it for straightforward macros. The primary usecase is when a
  pipeline of Sylan languages has been set up, with Sylan only being the
  final destination language to be compiled. In this case, it can't apply scopes
  to character and token streams since their source languages might not even
  have the comprehendible concept of Sylan-style scopes.
* A procedural macro can only have one parameter which must be syntax, but it
  can emulate multiple arguments by just lexing commas.
* AST macros can take multiple syntax parameters. When emitting from their
  pipelines, the emissions can be interleaved in generation but always end up
  being emitted in final source in order: the first pipeline emissions, followed
  by the second pipeline emissions, etc.
* AST macros and procedural macros can be invoked like functions in expressions.
  They do not break standard lexing of Sylan, although procedural macros can
  break tooling's _parsing_ of Sylan; utilities like syntax highlighting are
  highly recommended to use lexing rather than parsing for this reason.
* AST macros pose no problem to lexing and parsing because they take valid ASTs
  and produce them. Their evalation is merely delayed, and their form
  potentially transformed to another valid one. They needn't type check or pass
  other _semantic_ analysis stages though, so long as the _final produced_ AST
  does.
* AST macros can be triggered even with no arguments, in which case the AST
  pipeline source returns nothing. This can be used for no-arg item macros that
  emit items for the developer.
* Procedural macros must take valid tokens. Furthermore, grouping tokens must be
  evenly balanced until the end of the call. Procedural macros expect an
  identifier to trigger the macro call, followed by one token tree. This tree
  can be one token or one token _group_ indicated by an opening grouping token.
  This means tooling always knows when procedural macros end. A non-grouping
  token therefore looks like `macro1 42`, whereas a procedural macro taking a
  grouping token looks like `macro1(1, 2, 3, 4)` or `macro1 { 1, 2, 3, 4 }`.
  The grouping tokens are kept, meaning a macro can change behaviour based on
  how it's called.
* Character-based macros can not be invoked directly as functions; instead they
  are imported as normal and then triggered with `with reader` forms, and
  reset with `with reader sylan.core.lang.previousReader`.
* Character macros can only trigger on the start of tokens. Furthermore, they
  must be careful about using grouping token characters to avoid confusing the
  parsing from tooling. These are dangerous, and can therefore be blocked in
  modules entirely by banning compile-time dynamic scoping rebinds thereby
  making the current readtable immutable.
* When a macro is prefixed with an `@`, it applies to the whole item which it is
  attached to. This looks like Java annotations but is actually closer to
  Elixir's `use`. In this use case, Sylan calls them _annotations_, just like
  Java. In this case, the whole item AST or token stream is passed as a _final_
  argument to the function.
* When annotation macros, using tokens or ASTs, are invoked with multiple
  parameters, the item itself is always the final syntax argument.
* Annotation macros can be NOP macros. In fact, this is a handy usecase: an
  annotation can preprocess other NOP macros inside an item they annotate to
  change their behaviour. This is vaguely similar to a class annotation enabling
  special understanding of field annotations in Java frameworks like Spring.
* An annotation macro not attached to an item can instead occupy its own `item`
  slot, in which case it generates one or more items to take its place in the
  compiled code.
* An annotation macro where an expression is expected is an error.
* Symmetric pipelines have extra utility methods from the fusing of readers and
  writers to the same type: _passthroughs_, an ability when extending languages
  rather than building them from scratch.
* `Ast`s can never be symmetric pipelines because single ASTs are consumed as
  parameters, yet multiple AST can be emitted as a result. As they are eagerly
  evaluated, immutable trees rather than mutable streams, passthrough
  functionality makes little sense.
* `quote` takes all of the valid code in its block and turns it into an AST. It
  need only be valid according to the parser; it can use unbound identifiers and
  fail typechecking as well as using an item rather an expression. So long as the
  macro _produces_ valid code, it's OK. Like all quoting keywords, it only works
  in `final` functions, but needn't be used with `syntax`; a reusable
  compile-library can manipulate ASTs without directly emitting them themselves.
* The `unquote` keyword unquotes a part inside, allowing arbitrary logic to
  continue. `unquote` only supports two types: `Ast` and `List[Ast]`. Unquote
  takes either a single item or expression, or a block of them. This means both
  `unquote 42` and `unquote { 42 "foobar" }` work.
* Macros _consume_ their trigger to avoid accidental infinite loops. For
  example, a procedural macro consumes its triggering identifier before giving
  control to the function, a reader macro consumes its dispatching characters
  before giving control, an item macro removes the item from the item syntax
  before handing it over, a language pipeline is strictly left-to-right in
  a syntactical import, and a shebang import deletes the leftmost mentioned
  language from its `-l` argument list before feeding it back into the reader
  macro as the first line.
* Asymmetric pipelines can only convert upwards, i.e. character to token and
  token to AST. A macro can generate, say, tokens safe in the knowledge
  that a reader macro won't step in and reinterpret token's based on their
  starting characters. Likewise, a macro can emit an AST knowing that procedural
  will never trigger, creating malformed ASTs.
* Pipelines can also manually convert syntax parameters upwards inside their
  bodies, e.g. character stream to token stream and token stream to ASTs.
  This is useful when you want to trigger based on one type of macro form but
  then process it with higher-level tools. For example, `if` could trigger a
  reader macro that produces a call to `sylan.lang.if` _procedure_ macro, which
  works because Sylan guarantees to never reenter earlier phases for
  already-processed code. This means the reader macro is concerned only with
  grouping the tokens together for the `if` procedural macro call. That reader
  macro could then upgrade to the character stream to a token stream, skip over
  the whole `if/else` and put it inside the procedure macro call, which would
  then reinterpret it as an AST that boils it down to a `switch` over
  `sylan.lang.Boolean.True` and `sylan.lang.Boolean.False`.
* The `gensym` function can be used with `unquote` to generate symbols
  that cannot be literally mentioned, like `gensym` in Common Lisp, i.e.
  `unquote gensym`. Both `gensym` and `symbol(of: "SymbolName")` just delegate
  to the `Symbol(of name: Optional[String])` argument, an empty symbol being
  "unmentionable" and used in many places internally and not just macros.
* Macro expansion occurs after simplification. This might be simplified to just
  simplification itself being just macro expansions at earlier stages, we'll
  see. Anyway, this means it needn't parse many complex constructs to work out
  whether a symbol is being defined, thus needing _gensym_, or being looked up
  in an outer scope. If `var x = 5` is compiled down to `(-> x { })(5)` as
  currently planned, and pattern matching is compiled away to manual
  destructing and switching over enum tags, then only lambdas and other AST
  macros need to be considered when auto-producing `gensym`.
* `gensym` is mostly used internally; new bindings in AST types will use it
  behind the scenes for its new bindings anyway.
* Macros have the ability to produce either `ExpandedSymbol` tokens as well as
  the usual `Symbol`, the former being dynamically scoped on the callsite rather
  than the latter's being lexically scoped at the syntax's definition. This
  allows explicit opting out of hygine when desired.
* Special shebang formulations do more than just ease execution on some
  Unix-like OSes; they are interpreted especially by Sylan to run the entire
  file under a macro, creating new languages under Sylan.
* For external languages with no knowledge of Sylan, such as a Lua script,
  Sylan can import them as packages with invocations like:
  `import module1.`file.lua`syntax(luaLanguagePackage.read)`.
  So long as a macro can understand the syntax, Sylan can import it as if it
  were Sylan.
* Compile-time "reflection" will be based on macros, with an API inspired by
  Newspeak and Dart mirrors rather than traditional reflection APIs. Because
  macros give the ability for _anyone_ to write a compile-time reflection
  library, one shouldn't be blessed so much as to be fused onto the types
  themselves such as Java's `instance.getClass()` notation.
* Crazy idea: what if lambdas were just reader macros that produced `fun`s with
  `gensym`d names? Free identifiers would be captured in the AST, and translated
  into parameters, which would happen recursively downwards until lexical
  scoping had been "compiled" into the code. This would mean that only static
  item symbols and lambda parameters would exist.
* Another crazy idea: produce reader macros for all item definition forms that
  produce new item definition procedure macros, but _gensym the name based on
  the accessibility modifier_. That means accessibility is no longer a language
  concept either.
* And another: what if `ignorable` translated a function into a macro that
  dropped down to the underlying function call but then emitted a `|> ignore`
  on call sites automatically if the value is not used?
