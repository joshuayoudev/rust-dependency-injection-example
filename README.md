# Dependency Injection Example In Rust

This example shows how to do dependency injection in Rust natively without 3rd party crates and without dynamic
dispatching.

To try out these examples, simply run these commands:

```sh
cargo run
```

```sh
cargo run --features beta
```

```sh
cargo test -- --nocapture
```

# Starting With The Naive Approach

Anyone who has tried to tackle this problem likely started with the same idea - use traits to create interfaces for
dependencies, and use trait-bound generics to pass dependencies to functions:

```rs
trait FooDependency {
    fn bar();
}

// `impl` is syntactic sugar for trait-bound generics: `fn do_the_thing<F: FooDependency>(foo: F)`
fn do_the_thing(foo: impl FooDependency) {
    foo.bar()
}
```

However, this approach has some glaring problems.

## Problem 1

Swapping out a dependency requires a code change. This approach does not allow us to define different dependencies for
different environments:

```rs
trait FooDependency {}

struct ProdFoo;
impl FooDependency for ProdFoo {}

struct BetaFoo;
impl FooDependency for BetaFoo {}

fn main() {
    let prod_foo = ProdFoo;
    let beta_foo = BetaFoo;

    do_the_thing(prod_foo); // How do I change this to `beta_foo` for beta environments?
}
```

## Problem 2

Depending on how you try to solve problem 1, using functions with trait-bound generics can lead to longer compile times
and a larger binary file. Rust will statically resolve these generic functions through
[monomorphization](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html):

> This means that compiler stamps out a different copy of the code of a generic function for each concrete type needed.

So something like this:

```rs
fn do_the_thing(foo: impl FooDependency) {}
```

Could become:

```rs
fn do_the_thing_prod_foo(foo: ProdFoo) {}

fn do_the_thing_beta_foo(foo: BetaFoo) {}
```

Now imagine you have 20 dependencies, each dependency has 3 different implementations, and most of your functions
require all 20 dependencies. The amount of extra-generated code can get out of hand quickly.

## Problem 3

Passing dependencies as function parameters can lead to bloated function definitions:

```rs
fn program_entry_point(
    a: impl DependencyA,
    b: impl DependencyB,
    c: impl DependencyC,
    d: impl DependencyD,
    e: impl DependencyE,
    f: impl DependencyF,
    g: impl DependencyG,
) {
    do_the_thing(a, b, c, d, e, f, g);
}
```

Naturally, we can attempt to fix this issue by creating a container struct to hold all the dependencies. But if we try
using generics, it will not solve the bloat problem:

```rs
struct Dependencies<A, B, C, D, E, F, G> {
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
}

fn program_entry_point<
    A: DependencyA,
    B: DependencyB,
    C: DependencyC,
    D: DependencyD,
    E: DependencyE,
    F: DependencyF,
    G: DependencyG,
>(
    dependencies: Dependencies<A, B, C, D, E, F, G>,
) {
    do_the_thing(dependencies);
}
```

We could try fixing the bloat problem using trait objects, but that means we are now locked into dynamic dispatching:

```rs
struct Dependencies {
    a: Box<dyn DependencyA>,
    b: Box<dyn DependencyB>,
    c: Box<dyn DependencyC>,
    d: Box<dyn DependencyD>,
    e: Box<dyn DependencyE>,
    f: Box<dyn DependencyF>,
    g: Box<dyn DependencyG>,
}

fn program_entry_point(dependencies: Dependencies) {
    do_the_thing(dependencies);
}
```

# The Solution

The naive approach is actually quite good. In fact, it's likely how you'd achieve basic dependency injection in other
languages where everything is dynamically dispatched. But when using Rust, do as the Rustaceans do.

We can achieve statically dispatched, compile-time dependency injection by clever use of Rust's **associated types**,
**feature flags**, and **export aliasing**.

First, let's create a new trait that will act as the container that holds all of our dependencies:

```rs
pub trait Dependencies {
    // Note: The type name can be the same as the trait it is bounded to.
    //       They are named differently here to minimize any confusion.
    type DepA: DependencyA;
    type DepB: DependencyB;

    fn new() -> Self;

    fn dependency_a(&self) -> &Self::DepA;
    fn dependency_b(&self) -> &Self::DepB;
}
```

This is a `trait` rather than a `struct` so that we can create different containers for different environments. The
**associated types** are how we avoid carrying the generics in the trait definition:

```rs
impl DependencyA for ProdDependencyA {}

impl DependencyB for ProdDependencyB {}

pub struct ProdDependencies {
    prod_dependency_a: ProdDependencyA,
    prod_dependency_b: ProdDependencyB,
}

impl Dependencies for ProdDependencies {
    type DepA = ProdDependencyA;
    type DepB = ProdDependencyB;

    fn new() -> Self {
        // initialize dependencies..
    }

    fn dependency_a(&self) -> &Self::DepA {
        &self.prod_dependency_a
    }

    fn dependency_b(&self) -> &Self::DepB {
        &self.prod_dependency_b
    }
}
```

```rs
impl DependencyA for BetaDependencyA {}

impl DependencyB for BetaDependencyB {}

pub struct BetaDependencies {
    beta_dependency_a: BetaDependencyA,
    beta_dependency_b: BetaDependencyB,
}

impl Dependencies for BetaDependencies {
    type DepA = BetaDependencyA;
    type DepB = BetaDependencyB;

    fn new() -> Self {
        // initialize dependencies..
    }

    fn dependency_a(&self) -> &Self::DepA {
        &self.beta_dependency_a
    }

    fn dependency_b(&self) -> &Self::DepB {
        &self.beta_dependency_b
    }
}
```

Now, we can use Rust's **feature flags** and **export aliasing** to make it so that there is only ever 1 dependencies
container available to the program:

```rs
#[cfg(feature = "beta")]
mod beta_dependencies;

mod prod_dependencies;

#[cfg(feature = "beta")]
pub use beta_dependencies::BetaDependencies as ServiceDependencies;

#[cfg(not(feature = "beta"))]
pub use prod_dependencies::ProdDependencies as ServiceDependencies;
```

```toml
# Cargo.toml

[features]
beta = []
```

That's it! Let's see what the usage looks like:

```rs
fn main() {
    let service_dependencies = ServiceDependencies::new();

    execute(service_dependencies);
}

fn execute(dependencies: impl Dependencies) {
    let dependency_a = dependencies.dependency_a();
    let dependency_b = dependencies.dependency_b();
    // ...
}
```

```sh
cargo run
cargo run --features beta
```

# Notes

There are some minor gotchas that come with this approach, mainly around developer experience. For visibility, I have
copy / pasted some comments from the code example:

```rs
// src/dependencies/mod.rs


// Note 1: Technically, we do not need to lock the module behind a feature flag. But it is best
//       practice to remove any dead code in production environments. Adding the feature flag here,
//       as well as on each non-production dependency, ensures that the compiler removes the code
//       from the binary.
//
// Note 2: Code locked behind feature flags will be grayed out / lose intellisense support. A
//         workaround is to use the `any()` check and include the `test` flag. IDEs by default seem
//         to use the `test` flag during development, thus keeping the intellisense.
#[cfg(any(feature = "beta", test))]
mod beta_dependencies;

mod prod_dependencies;

// Note: In this case, we cannot use the `any()` + `test` trick as it will cause the IDE to see 2
//       `ServiceDependencies`
#[cfg(feature = "beta")]
pub use beta_dependencies::BetaDependencies as ServiceDependencies;

// Note: We must include this `not()` check, otherwise the program will see multiple
//       `ServiceDependencies` for any non-production environments
#[cfg(not(any(feature = "beta")))]
pub use prod_dependencies::ProdDependencies as ServiceDependencies;
```
