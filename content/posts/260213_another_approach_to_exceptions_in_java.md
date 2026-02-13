---
title: Another approach to checked exceptions with the Result type
date: "2026-02-13"
author: Gertjan Assies
category: code
tags: java, functional-programming, exceptions, featured
image: "/content/images/result_type_top.jpg"
summary: Replacing checked exceptions with a Result type for cleaner, more composable error handling in Java
published: true
---

As changes are greater that I will be doing more Java engineering in the near future, I wanted to up my skills as the last time I did some serious Java work was 12 years ago

One of the things I like to do is take concepts that I learned from other languages that are not present in the languages Im focusing on.

As I learned from (mostly functional) programming languages is that failures should be modelled in your application just as much as successes (the happy path) and that something called an Exception is not really a semantically good way of handling those.

Java's checked exceptions often lead to verbose code, awkward abstractions, and layers of try-catch blocks that obscure business logic. What if there was a better way?

Enter the **Result type** a functional programming pattern that represents operations that can succeed or fail, without resorting to exceptions for control flow.

The application I chose to experiment with this, is a commandline application that manages filament (the rolls of plastic wire used in 3D-printing). so all the examples are from that sourcecode.

## The Problem with Checked Exceptions

Consider a typical repository method that reads from a file:

```java
public List<Filament> loadFilaments() throws IOException {
    return objectMapper.readValue(
        filePath.toFile(),
        new TypeReference<List<Filament>>() {}
    );
}
```

This signature forces every caller to either handle `IOException` or propagate it up the call stack. The problem compounds when you have multiple layers:

```java
// Service layer - forced to propagate
public List<Filament> getAllFilaments() throws IOException {
    return repository.loadFilaments();
}

// Command layer - more propagation
public String listAll() throws IOException {
    List<Filament> filaments = service.getAllFilaments();
    return formatFilaments(filaments);
}
```

Or worse, developers start swallowing exceptions inappropriately just to avoid dealing with them:

```java
try {
    return service.getAllFilaments();
} catch (IOException e) {
    return Collections.emptyList(); // Silent failure!
}
```

## The Result Type Solution

The `Result<T, E>` type explicitly represents success or failure as a value, not an exception. Here's the interface:

```java
public sealed interface Result<T, E>
    permits Result.Success, Result.Failure {

    record Success<T, E>(T value) implements Result<T, E> {}
    record Failure<T, E>(E error) implements Result<T, E> {}
}
```

Using Java's sealed interfaces and records (introduced in Java 17), we get exhaustive pattern matching and immutability for free.

## Wrapping with Result.of()

The key to adoption is making it **easy to wrap** potentially failing operations. The `Result.of()` method handles this beautifully:

```java
@FunctionalInterface
interface ThrowingSupplier<T> {
    T get() throws Exception;
}

static <T, E> Result<T, E> of(
    ThrowingSupplier<T> supplier,
    Function<Exception, E> errorMapper
) {
    try {
        return new Success<>(supplier.get());
    } catch (Exception e) {
        return new Failure<>(errorMapper.apply(e));
    }
}
```

Now our repository becomes exception-free:

```java
@Override
public Result<List<Filament>, String> loadAll() {
    if (!Files.exists(filePath)) {
        return new Failure<>("File not found: " + filePath);
    }

    return Result.of(
        () -> objectMapper.readValue(
            filePath.toFile(),
            new TypeReference<List<Filament>>() {}
        ),
        e -> "Failed to parse filament data: " + e.getMessage()
    );
}
```

Notice how we:
1. Handle the "file not found" case explicitly with a `Failure`
2. Wrap the Jackson parsing with `Result.of()` which catches any `Exception`
3. Map exceptions to user-friendly error messages using the error mapper function
4. In this case the Error type is a String, but we could just as easily pass on the Exception type itself.

## Unwrapping with result.fold()

The `fold()` method is where the magic happens. It forces you to handle both cases — success and failure — and produce a single unified result:

```java
default <R> R fold(
    Function<E, R> onFailure,
    Function<T, R> onSuccess
) {
    return switch (this) {
        case Success<T, E>(T value) -> onSuccess.apply(value);
        case Failure<T, E>(E error) -> onFailure.apply(error);
    };
}
```

In practice, this looks incredibly clean at the command layer:

```java
@ShellMethod(key = "list", value = "Lists all filaments")
public String listAll(
    @ShellOption(defaultValue = "TABLE") OutputFormat format
) {
    return filamentService.getAllFilaments().fold(
        error -> "Failed to retrieve filaments: " + error,
        filaments -> formatFilaments(filaments, format)
    );
}
```

No try-catch blocks. No exception propagation. Just two functions: one for the error path, one for the success path. The compiler ensures you handle both.

## Composition and Transformation

The real power emerges when composing multiple operations. The Result type provides `map()`, `flatMap()`, and `mapError()` for elegant transformations:

```java
// Transform successful values
result.map(filaments -> filaments.size());

// Chain operations that also return Results
result.flatMap(filament ->
    calculateCost(filament.id(), length)
);

// Transform error types
result.mapError(msg ->
    new ErrorResponse(500, msg)
);
```

Here's a real example from a cost calculation service:

```java
public Result<CostCalculation, String> calculateCost(
    int id,
    double length
) {
    return filamentRepository.findById(id)
        .flatMap(filament ->
            filamentTypeRepository.findById(filament.filamentTypeId())
                .map(type -> calculateCost(filament, type, length))
        );
}
```

If either the filament or filament type lookup fails, the error propagates automatically. If both succeed, the calculation runs. All without a single try-catch block.

## Benefits in Practice

After refactoring to the Result type, I've noticed:

**Cleaner signatures**: Methods declare exactly what can go wrong without forcing exception handling.

```java
// Before
List<Filament> getAllFilaments() throws IOException, ParseException

// After
Result<List<Filament>, String> getAllFilaments()
```

**Explicit error handling**: Every `.fold()` is a visible decision point where errors are handled.

**Better error messages**: Error mapper functions let you provide context-specific messages at the failure point, not generic catch-all handlers.

**Composability**: Chaining operations with `flatMap()` creates pipelines that fail fast and propagate errors naturally.

## When to Use It

The Result type shines for:
- **Repository/data access layers**: File I/O, database queries, external API calls
- **Business logic**: Validation, calculations, domain operations that can fail
- **Service boundaries**: Clear success/failure contracts between layers

You might still want traditional exceptions for:
- **Programmer errors**: `NullPointerException`, `IllegalStateException`
- **Fatal errors**: Out of memory, stack overflow
- **Third-party code**: When you don't control the interface

## Conclusion

The Result type transforms error handling from an afterthought into a first-class concern. By wrapping potentially failing operations with `Result.of()` in lower tiers and unwrapping with `result.fold()` at boundaries, you create code that's more explicit, composable, and maintainable.

Checked exceptions tried to force explicit error handling but ended up creating layers of ceremony. The Result type achieves the same goal through values instead of control flow — a more functional, more elegant approach.

The full implementation is available in my [Filament project](https://github.com/gertjana/filament-calculator-springboot).

the Result class specifically is here: [Result.java](https://github.com/gertjana/filament-calculator-springboot/blob/main/src/main/java/dev/gertjanassies/filament/util/Result.java)
