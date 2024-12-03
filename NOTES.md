# Notes

> Every let statement in Rust is a pattern match.

## Fat pointers

Stores both the pointer to the start of the slice and the length of the slice in question. This is what happens behind the notation `&str`.

## Library design and performance

It is relevant to understand the use case for a determined library that you're developing, even more when you are constantly facing decisions that may cost performance. For instance, is it really not a big deal to use a heap-allocated string instead of a slice? What devices do you expect your code to be ran on?
