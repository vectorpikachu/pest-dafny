# pest-dafny

A parser for Dafny language.

TODO: I haven't noticed that pest is a **Non-backtracking** parser generator.

```PEG
Stmt -> 
```

To eliminate the left recursion, we need to add a new rule:

