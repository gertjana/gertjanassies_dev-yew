---
title: An Optional type in Go - an experiment
author: Gertjan Assies
date: "2022-05-28"
category: code
tags: go, functional, option, experiment, featured
image: "/static/images/optional_go_top.jpg"
summary: "This blog post is an thought experiment in how a functional programming concept could be implemented in the Go language."
published: true

---

A common pattern in Go is to have your function return an error next to the result to signify a failure inside the function like so:

```go
func Divide(a float64, b float64) (float64, error) {
    if b == 0 {
        return 0, errors.New("Divide by zero, really?")
    }
    return a/b
}
```

What if we could capture this in a Type with a couple of methods like so:

```go
package optional

Type Maybe[T] struct {
    Value T
    Error error
}

type Optional interface {
    New()       // Constructor
    Failed()    // true if Error
}
```

Then our Divide function can become:

```go
func Divide(a float64, b float64) Maybe[float64] {
    if b == 0 {
        return optional.New(Value: 0, Error: errors.New("Divide by zero, really?"))
    }
}

maybeDivived := Divide(2,4)
if maybeDivided.Failed() {
    fmt.Printf("Divide failed with %v", maybeDivided.Error)
}
```

Now this is even more verbose then the original example, that's not improving on the situation. So lets try to implement a Map, where we can apply a function on the Value inside the Maybe and return a Maybe with the result of the function.

```go
func Map[S any, T any](m Maybe[S], f func(S) (T, error)) Maybe[T] {
    if m.Error != nil {
        var unit T
        return New(unit, m.Error)
    } else {
        r, err := f(m.Value)
        return New(R, err)
    }
}
```

So here we take a:

* Maybe[S] and unwrap the S
* apply a function from S -> T on the inside Value
* wrap again and return the Maybe[T]

When there's an Error in the Maybe[S], I return a Maybe[T] with an uninitialized var of type T (which is kinda the Nothing type in Go, if you don't look too closely) and the error, if not I'm executing the function and returning the result with any error that might have happened in that function

So now running:

```go
maybe := New("2022-05-27T12:24:00Z", nil)
f := func(date string) (time.Time, error) {
    return time.Parse(time.RFC3339, date)
} // returns a func(string) (time.Time, error)
result := Map(maybe,f) // returns a Maybe[time.Time]
```

Will parse the string as a time.Time and handle the error when the string cannot be parsed, OK this is becoming a little more useful.

But what if I want to chain a bunch of functions together that all could potentially give an error. Let's implement a let's call it \`AndThen\` function

```go
func (m Maybe[T]) AndThen(f func (T) (T, error)) Maybe[T] {
    return Map(m, f)
}
```

As you can see AndThen is just a Map function. but now it applies to the struct

```go
addOne := func (x int) (int, error) { return x+1, nil }
multThree := func (x int) (int, error) { return x*3, nil }

maybe := New(42, nil)

res := maybe.
        andThen(addOne).
        andThen(multThree).
        Value
// 129

```

Unfortunately I couldn't get the AndThen to work with different input and result types. maybe someone more familiar with Go Generics can explain to me how that is done, if at all possible.

Here I come to the conclusion that the chainable AndThen functionality is actually useful. especially when it's possible to use different types for in- and output.

But I'm also coming to the conclusion, that implementing a pure functional library in Go is maybe not a good idea, as the language is not really suited for that. and that's fine. go learn yourself a bit of Haskell if you want that.
The number of abandoned 'experiments' I found on public repo's also attested to this

All the code used is in this repo: [https://gitlab.com/gertjana/optional-go/-/blob/main/cmd/optional/optional.go](https://gitlab.com/gertjana/optional-go/-/blob/main/cmd/optional/optional.go)

Hope you learnt something, I did

UPDATE:

It is indeed not possible to have the AndThen method on the Maybe with a function with different types, as all the types you use need to be defined on the Maybe.

Maybe there is a way to mimic union types with interfaces and get it working like that, but that's something for a new experiment.
