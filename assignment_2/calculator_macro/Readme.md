## Calculator Macro 

### Description

This is a derive macro for calculator. This macro implements the functionalities of addition, subtraction, multiplication and modulus.

### Usage

create a necessary methods

```
#[derive(Calculator)]
#[Operation = Addition]
pub struct Task1{
    op1 : i32,
    op2 : i32,
}
```

### References

- [Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
