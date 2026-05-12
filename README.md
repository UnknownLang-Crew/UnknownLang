# UnknownLang

A statically typed compiled programming language focused on:

- simplicity
- safety
- readability
- explicit behavior

UnknownLang is inspired by:

- Rust
- Haxe
- Swift
- Python
- Lua
- C#

---

## Features

- Static typing with type inference
- Safe-by-default design
- Explicit `unsafe` blocks
- Nullable types
- Generics
- Match expressions
- Async/await
- Extensions
- Compile-time execution
- Fixed-size arrays
- Modern readable syntax

---

## Example

```unk
func greet(name: String) -> String {
    return "Hello ${name}"
}

func main() {
    let user: Nullable<String> = "Nova"

    if (let name = user) {
        print(greet(name))
    }
}
```

---

## Variables

```unk
let name = "Nova"     // immutable
mut score = 0         // mutable
const PI = 3.14159    // compile-time constant
```

---

## Nullable Types

```unk
let name: Nullable<String> = null

let display = name ?? "Unknown"

if (let n = name) {
    print(n)
}
```

---

## Functions

```unk
func add(a: Int, b: Int) -> Int {
    return a + b
}
```

Short form:

```unk
func double(x: Int) -> Int = x * 2
```

---

## Match

```unk
let text = match (status) {
    case 200: "OK"
    case 404: "Not Found"
    case _: "Unknown"
}
```

---

## Async

```unk
async func fetchUser(id: Int) -> User {
    return await api.get(id)
}
```

---

## Unsafe

```unk
unsafe {
    let ptr = alloc<Int>(1)

    ptr.write(0, 42)

    let value = ptr.read(0)

    free(ptr)
}
```

---

## License

yes, [MIT License](./LICENSE.md)
