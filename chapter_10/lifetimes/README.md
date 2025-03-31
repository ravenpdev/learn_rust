#### Lifetime Rules

1. The first rule is that the compiler assigns a lifetime to each parameter that's a reference.\ In
   other words, a function with one pararmeter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a\
   function with two parameter gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32);\
   and so on.

2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned\
   to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

3. The third rrule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self\
   because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule \
   makes methods much nicer to read and write because fewer symbols are necessary.

#### The Static Lifetime

One special lifetime we need to discuss is 'static, which denotes that the affected reference can live\
for the entire duration of the program. All string literals have the 'static lifetime, which we can \
annotate as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```
