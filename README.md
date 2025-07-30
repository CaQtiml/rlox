# Interpreters

I am now following CRAFTING INTERPRETERS book, but in Rust. Currently, I am doing **A TREE-WALK INTERPRETER**.

Objective
- Learn a compiler concept
- Learn how to actually implement compiler, umm I mean interpreter here.
- Learn Rust the hard way!

The way I learn from this book is that for each chapter
- I use LLM to write a code structure ask it to leave each function implementation to be my work.
- I implement a functionality for these functions by myself.

How to run:
- `cargo run -- test.lox` or `cargo run -- <file_name.lox>`to test from `.lox` file
- `cargo run` to test interactively
- `cargo run -- --test-control-flow` to test from a determined flag.

A language document has not been written yet, but some examples can be found from example scripts (xxx.lox).

Progress
- [x] Scanning
- [x] AST Support
- [x] Parsing
- [x] Evaluating Expression
- [x] Statements and State
- [x] Control Flow
- [x] Functions - May still contain some bugs. I'm not sure. But the current interpreter uses a dynamic scoping, which will be fixed in the next chapter.
- [ ] Resolving and Binding
- ...

Note to myself: I use Git on Windows System

If you want to navigate to the result of each chapter, you can use
```
git checkout <commit-hash>
```

If you want to navigate to the most recent commit, use
```
git checkout master
```

I have supplement blogs here.
- [Tree-walk Part 1](https://sivakornl.com/posts/rlox-tree-1/)
- [Tree-walk Part 2](https://sivakornl.com/posts/rlox-tree-2/)