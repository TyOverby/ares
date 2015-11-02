# Core

The core libraries are the fundamental building blocks of your Ares programs.
Because they are often used by other parts of the standard library, the core
library *must* be loaded if any other parts of the standard library are to
be loaded.  However, because of Ares' extendability, you can elect to not load
core, and forfeit the rest of the standard library.

# eval
Manually evaluates an Ares object.
#### Form: `(eval object)`
Evaluates a single Ares object and returns the result.
#### Examples:
```clojure
> (eval (list + 1 2 3))
6
> (eval 5)
5
```

# apply
Evaluates an Ares object as a function given a list of arguments.
#### Form `(apply f arg-list)`
Given a function and a list of arguments `apply` calls the function with
the argument list providing all the arguments.
Apply assumes that the arg-list contains pre-evaluated objects, and will
not evaluate them again.
#### Examples
```clojure
> (apply + (list 1 2 3))
6
> (apply + (list 0 (+ 1 2 3)))
ERROR: UnexpectedType: Expected Int, got List
```

# quote
Prevents an object from being evaluated
#### Form `(quote object)`
Given a single object, prevents that object from being evaluated.
This can be used as a way to keep an AST from being run.

The '\'' syntax can be used as an alias for quote.
#### Examples
```clojure
> (quote (+ 1 2 3))
[+ 1 2 3]
> (quote a)
'a
```

# if
Chooses between two control flow paths based on a condition
#### Form `(if condition true-branch false-branch)`
Evaluates the condition first, then based on the result, evaluates
either the true-branch or the false-branch.
#### Examples
```clojure
> (if true 1 2)
1
> (if false 1 2)
2
> (if (int? 5.0) "int" "not int")
"not int"
```

# let
Introduces local variable bindings for the duration of the body
#### Form `(let (bindings*) bodies*)`
Th#e bindings can reference previous bindings and may be recursive.
#### Examples
```clojure
> (let (a 5) a)
5
> (let (a 5 b 10) (+ a b))
15
> (let (a 5 b a) (+ a a))
10
> (let (a (lambda () (a))) (a))
ERROR: Stack Overflow
```

# define
Introduces a variable binding in the current scope.
#### Form `(define name value)`
Define can be called anywhere in an Ares program and defines
the value inside its current lexical scope.  The result of define
is the value that was passed in.

Defines can shadow eachother if in different scopes, but
can not overwrite one another if in the same scope.
#### Examples
```clojure
> (define x 5)
> x
5
> (define f (lambda ()
>     (define result 10)
>     result))
> (f)
10
> (define g (lambda ()
>     (define g 15)
>     g))
> (g)
15
```

# set
Changes a previously-defined variable to have a new value.
#### Form `(set name value)`
Sets the name at the closest scope to the set.  Set can not
define something that doesn't exist yet.
#### Examples
```clojure
> (define x 5)
> (set x 10)
> x
10
> ((lambda (x)
>     (set x 10)
>     x) 5)
10
```

# lambda
Creates a new anonymous function.
#### Form `(lambda (args*) bodies*)` OR `(lambda arg-list bodies*)`
The last body executed will be the one that is returned.
#### Examples
```clojure
> (define f (lambda (x y z) (+ x y z)))
> (f 1 2 3)
6
> (define g (lambda l (apply + l)))
> (g 1 2 3)
6
```

# gensym
Generates a unique anonymous symbol
#### Form `(gensym)` OR `(gensym string-prefix)`
These anonymous symbols can be used to generate code that has
requires named symbols.  They can also be used as keys for
dictionaries so that it is impossible for other programmers to access
the value.
### Examples
```clojure
> (gensym)
's98
> (gensym) # gives you a unique symbol
's99
> (gensym "some-prefix-")
'some-prefix-100
> (gensym "some-prefix-")
'some-prefix-101
```

