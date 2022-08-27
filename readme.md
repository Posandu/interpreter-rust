# Simple interpreter written in Rust

I made this interpreter to learn Rust.

## Docs

```js
stack=[] // Stack looks like this 

p(value) -> push value to stack // ex: p(1) -> stack=[1]
o() -> pop value from stack // ex: o() -> stack=[]
s() -> print stack 
s(index) -> print stack[index]
o(index) -> pop value from stack[index]
```