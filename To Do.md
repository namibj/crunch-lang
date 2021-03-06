# To Do

## Syntax/Parser

- [ ] Graphing ASTs
- [ ] Type construction `Type is \n field := val \n field2 := val2 \n end`
  - [ ] Commas for fields
- [ ] Declaring/constructing unit types with `struct Type is unit`
- [ ] Make everything an expression
  - [ ] Nested declarations
- [x] Add patterns for match bindings
  - http://noamz.org/thesis.pdf
  - [ ] Tuple patterns
  - [ ] Slice patterns
  - [ ] Destructure patterns
  - [ ] Binding on patterns `<ident> @ <pattern>`
  - [ ] Array patterns?
  - [ ] Type patterns
- [ ] `if let <pattern> = <expr>` for oneshot matches
  - [ ] `while let <pattern> = <expr>`
- [ ] Bit-level matching http://erlang.org/doc/programming_examples/bit_syntax.html
- [ ] Named function arguments
- [ ] Commas with enum decls
- [ ] Commas with struct field decls
- [ ] Typed variadic function arguments `fn t(variadic: ..i32)` (Either `..` or `...`)
  - [ ] Sugar for slices
  - [ ] Limited to the last function arg
  - [ ] Add a "spread" operator (Reuse `..`, add `...`?) to allow passing slices to variadics?
  - [ ] Named args will also allow passing slices in a variadic position
- [ ] Dependent typing
  - [ ] Types as values
- [ ] Add reference types `&` and `&mut`
- [ ] Destructure anonymous enums (`&str | i32`) via matches e.g. `match val: &str | i32 string: &str => ..` or `if let`
  - [ ] Part of patterns
- [ ] Unique types, each instance is incompatible with any other
- [ ] Postfix macros
- [ ] Unions
- [ ] Raw pointers
  - [ ] `*mut T`/`*const T`?
- [ ] Blocks `block \n <stmt>* \n end`
- [ ] Async
  - [ ] `async fn`
  - [ ] `async do`
  - [ ] `async block`
- [ ] Const
  - [ ] `const var: T = ...`
  - [ ] `const do`
  - [ ] `const block`
- [ ] In-file modules `module <ident> \n <ast>* \n end`
- [ ] With blocks `with <expr> as <ident> \n <stmt>* \n end`
  - [ ] Sugar for a normal block
  - [ ] Useful for scoped allocators, locks, etc.
- [ ] Closures `do (<param>*) \n <stmt>* \n end`
- [ ] Use something more ergonomic for holding statements and expressions
  - [ ] Look at what rust does https://rustc-dev-guide.rust-lang.org/memory.html
  - [ ] Possibly use `lasso` with arbitrary internment?
- [ ] Reflection/Metaprogramming
- [ ] String Formatting
  - [ ] Inlined string variables e.g. `"{var}"`
  - [ ] String format specifiers e.g. `"{:?}"`
- [ ] Char escapes in char literals
- [ ] Allow one-liner match arms delimited by commas
- [ ] Effects?
  - [ ] Postfix `.suspend`
- [ ] Annotate *all* parse functions with grammar rules
- [ ] Split Pratt sub-functions into methods on `Parser`
- [ ] {De}serializable arena
- [ ] Achievements
- [ ] Bidirectional type checking
- [ ] Switch tests to do two-way serialization, check equality structurally and not by strings
- [ ] Generic mutability
- [ ] Function arguments are patterns with required types
- [ ] Variable declarations are patterns
  - [ ] Function parameters are patterns w/ required types?
- [ ] Switch to structured logging via `slog`
  - PITA because you have to pass it around, alternatives?
- [x] Stop using `.deref()`, instead use `*`
- [x] Re-add function generics
  - [x] `fn t[T, E, F](t: T, e: E, f: F)`
- [x] Macro for inserting stack frame counters
- [x] Add mutability to decls `let mut Ident`
- [x] Remove usages of `.data()` since `Deref` is implemented for `Locatable`
  - [x] Implement `DerefMut` for `Locatable`
  - [x] `AsRef` & `AsMut` for `Locatable`
- [x] Change imports into `paths.like.this` instead of `"this.bullshit"`
- [x] `loop` doesn't need `then` clauses
- [x] Else clause for loops that executes on breakage?
- [x] Fix if parsing
- [x] Switch to `lexical-core` for all float parsing
- [ ] AST Visitor
  - [x] Trait
  - [x] Semantic checker
  - [x] Ladder
  - [x] Typecheck
  - [ ] Symbol table
  - [ ] Pretty printer
    - [ ] Remove this entirely in favor of better testing methods?
- [ ] Debug blocks for debug assertions and stuff
    - https://dlang.org/spec/version.html#debug

## AST -> HIR lowering

- [ ] Propagate location information through AST & HIR
- [ ] Separate namespace for variables and types/enums/traits/functions
- [ ] Add all fields of structs and variants of enums to the symbol table
- [ ] Make the HIR accept more ast constructs

## Type checking

- [ ] Make type errors better
- [ ] Bidirectional checking
- [ ] Infer structural sums automatically and verify w/ bidirectional checks

## Backend

- [ ] SMT Solver
  - [ ] DYI or Z3?
  - [ ] Could do double-duty for optimization and trait/type solving/inference
  - [ ] SIMD?
  - https://arxiv.org/pdf/1809.02161v1.pdf
- [ ] LLVM wrapper
  - [ ] DIY?
- [ ] File checksums for knowing what needs to be recompiled https://apenwarr.ca/log/20181113
  - [ ] Hash compiler version, git rev and LLVM version together as part of cache fingerprint
- [ ] Symbolic execution
- [ ] No dependencies on crt or libc
- [ ] MIR for majority of passes
- [ ] SSA IR for compilation to LLVM
- [ ] https://github.com/rust-lang/rust/pull/45920
- [ ] https://github.com/rust-lang/rust/tree/master/src/librustc_codegen_llvm/llvm
- [ ] Size, align and stride

### Optimizations

- [ ] Unused field removal on an instance basis
- [ ] Dedup and overlap all static strings into one large static string
    - Create lib for this, `lasso` can also make use of it

## Architecture

- [ ] Look into a query-based arch
  - [ ] Enables incremental compilation
  - [ ] Can reduce workload
  - [ ] Makes concurrency easier
    - [ ] With mutexes lock contention is incredibly high, requires more advanced data structures to be efficient (concurrent skiplist?)
- [ ] Look into actor-based compiler
  - [ ] Works well with query system
  - [ ] Distributed compilation
- [ ] Incremental compilation
- [ ] Concurrent compilation
- [ ] First-class async
  - [ ] Streams & Channels
  - [ ] Structured concurrency
  - [ ] Std-provided runtime
    - [ ] Executor/Stream/Future traits for modularity to allow seamless switching of runtimes
- [ ] Effects?
  - http://ps.informatik.uni-tuebingen.de/publications/schuster19zero.pdf
  - https://www.unisonweb.org/docs/language-reference
  - https://github.com/pandaman64/effective-rust
  - https://github.com/effect-handlers/effects-rosetta-stone
  - [ ] Static dispatch of effects with defunctionalization/CPS
- [ ] Opt-in UB
- [ ] More instrumenting
- [ ] Module-level parallelism
- [ ] Ability to test compile errors

## Refinement types

- [ ] Integrate with contracts so that a refined type that meets contracts elides runtime contract checking
    - Allows predictable check elision
- [ ] Optimize out overflow checks on operations if it's impossible for something to overflow
