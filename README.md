## `impl Trait` not allowed outside of function and method return types

**… but it is now!**

This library gives you one macro, and one macro only: `#[desugar_impl]`.

Annotate any struct, enum, or union with `#[desugar_impl]`
to allow the use of `field_name: impl SomeTrait` in their declaration. E.g.

```
#[desugar_impl]
struct Test {
    a: impl Clone + PartialOrd,
    b: impl Clone + PartialOrd,
    c: impl Hash,
}
```

desugars to

```
struct Test<Ty1, Ty2, Ty3>
where
    Ty1: Clone + PartialOrd,
    Ty2: Clone + PartialOrd,
    Ty3: Hash,
{
    a: Ty1,
    b: Ty2,
    c: Ty3,
}
```

You can still place any `#[derive(…)]` macros just below `#[desugar_impl]`, and they'll see
the desugared code.
