# Logical

The logical module contains basic logical operators: and, or, and xor.
All of these operators are short-circuiting, so they will return
early without evaluating further arguments if possible.

# and
Returns true if none of the arguments are false.
#### Form: `(and booleans*)`
Returns true when passed no arguments.  Will stop evaluating arguments
as soon as it encounters one `false` expression.
#### Examples
```clojure
> (and)
true
> (and true)
true
> (and false)
false
> (and true false true)
true
> (and true true true)
true
> (and false (print "not evaluated"))
false
```

# or
Returns true if any of the arguments are true.
#### Form: `(or booleans*)`
Returns false when passed no arguments.  Will stop evaluating arguments
as soon as it encounters one `true` expression.
#### Examples
```clojure
> (or)
false
> (or true)
true
> (or false)
false
> (or true false)
true
> (or false true)
true
> (or true (print "not evaluated"))
true
```

# xor
Returns true if at least one of the arguments is true, and at least one of the arguments is false.
#### Form: `(xor booleans*)`
Returns false when passed no arguments, or one argument.  Will stop
evaluating arguments as soon as it encounters one `true` and one `false`
#### Examples
```clojure
> (xor)
false
> (xor true)
false
> (xor false)
false
> (xor true false)
true
> (xor true true false)
true
> (xor false false true)
true
> (xor true true)
false
> (xor false false)
false
```
