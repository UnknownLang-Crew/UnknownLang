# UnknownLang Spec

Inspired by Haxe, Rust, Swift, Python, Lua, and C#.
Build on Rust

UnknownLang is a statically typed compiled language focused on:

- simplicity
- safety
- readability
- explicit behavior

---

## Variables

```unk id="vars001"
let name = "Nova"         // immutable
mut score = 0             // mutable
const PI = 3.14159        // compile-time constant
```

Types are inferred when possible.

```unk id="vars002"
let age: Int = 20
mut items: Array<String> = []
```

---

## Primitive Types

```unk id="types001"
Int       // platform-width signed integer (32-bit on 32-bit targets, 64-bit on 64-bit targets)
Float     // platform-width float (64-bit on most targets)
Bool
String
Void
Nullable<T>
Array<T>
FixedArray<A, T>
Any
```

Sized integer types:

```unk id="types002"
// Signed
Int8
Int16
Int32
Int64

// Unsigned
UInt8
UInt16
UInt32
UInt64
```

Sized float types:

```unk id="types003"
Float32
Float64
```

---

## Operators

### Arithmetic

```unk id="op001"
let x = a + b    // addition
let x = a - b    // subtraction
let x = -a       // negation
let x = a * b    // multiplication
let x = a / b    // division (integer division truncates toward zero)
let x = a % b    // remainder (sign matches the dividend)
```

Arithmetic on integers **panics on overflow** by default. For intentional overflow, use the explicit methods:

```unk id="op002"
let x = Int8.MAX.wrappingAdd(1)            // -128  (wraps)
let x = Int8.MAX.saturatingAdd(10)         // 127   (clamps)
let x: Nullable<Int8> = 100i8.checkedAdd(50)  // null on overflow
```

For exponentiation, use the method form: `n.pow(3)`, `Math.pow(a, b)`.

---

### Bitwise

Bitwise operators work on integer types only.

```unk id="op003"
let x = a & b    // AND
let x = a | b    // OR
let x = a ^ b    // XOR  (not exponentiation)
let x = ~a       // NOT  (unary, flips all bits)
let x = a << n   // left shift
let x = a >> n   // right shift
```

Shift behavior:

- The shift amount must satisfy `0 <= n < bitWidth`. A `comptime` violation is a compile error; a runtime violation is a panic.
- `>>` on a **signed** integer is an **arithmetic** shift (sign-extending).
- `>>` on an **unsigned** integer is a **logical** shift (zero-filling).

```unk id="op004"
let a: UInt8 = 0b10001000 >> 3    // 0b00010001 (17) — logical
let b: Int8  = -8 >> 1            // -4              — arithmetic
let c: UInt8 = 0b1100 ^ 0b1010   // 0b0110 (6)
```

---

### Comparison

```unk id="op005"
==   !=   <   >   <=   >=
```

---

### Logical

```unk id="op006"
&&   ||   !
```

Both `&&` and `||` short-circuit.

---

### Range

```unk id="op007"
0..10     // exclusive: 0–9
0..=10    // inclusive: 0–10
```

---

### Operator Precedence (high → low)

| Level       | Operators                   |
| ----------- | --------------------------- |
| 1 (highest) | `!` `~` unary `-`           |
| 2           | `*` `/` `%`                 |
| 3           | `+` `-`                     |
| 4           | `<<` `>>`                   |
| 5           | `&`                         |
| 6           | `^`                         |
| 7           | `\|`                        |
| 8           | `==` `!=` `<` `>` `<=` `>=` |
| 9           | `&&`                        |
| 10          | `\|\|`                      |
| 11 (lowest) | `??` `? :`                  |

---

## Nullable Types

Nullable values use `Nullable<T>`.

```unk id="null001"
let name: Nullable<String> = null
```

Safe unwrap:

```unk id="null002"
if (let n = name) {
    print(n)
}
```

Fallback values:

```unk id="null003"
let display = name ?? "Unknown"
```

---

## Functions

```unk id="func001"
func greet(name: String) -> String {
    return "Hello ${name}"
}
```

Short form (single expression, no braces):

```unk id="func002"
func double(x: Int) -> Int = x * 2
```

Default parameters:

```unk id="func003"
func connect(host: String, port: Int = 8080) { }
```

Named arguments:

```unk id="func004"
connect(host="localhost", port=3000)
```

Lambdas:

```unk id="func005"
let add = (a: Int, b: Int) => a + b
```

---

## Collections

```unk id="col001"
let nums = [1, 2, 3]
let map = { "a": 1, "b": 2 }
```

Generic collections:

```unk id="col002"
Array<Int>
Map<String, Int>
Set<String>
```

---

## Control Flow

### If

```unk id="flow001"
if (score > 50) {
    print("Pass")
} else {
    print("Fail")
}
```

If expression:

```unk id="flow002"
let label = if (score > 50) "Pass" else "Fail"
```

---

### Loops

Range `0..10` is exclusive of the upper bound (iterates 0–9).

```unk id="flow003"
for (i in 0..10) {
    print(i)
}
```

```unk id="flow004"
while (running) {
    update()
}
```

---

## Match

```unk id="match001"
match (value) {
    case 1:
        print("one")

    case 2, 3:
        print("two or three")

    case _:
        print("other")
}
```

Match expression:

```unk id="match002"
let text = match (status) {
    case 200: "OK"
    case 404: "Not Found"
    case _: "Unknown"
}
```

---

## Classes

```unk id="class001"
pub class Player {
    pub let id: Int
    mut _health: Int = 100

    pub func init(id: Int) {
        self.id = id
    }

    pub func heal(n: Int) {
        self._health += n
    }
}
```

Inheritance:

```unk id="class002"
pub class Mage extends Player {
    pub override func heal(n: Int) {
        print("Magic heal")
    }
}
```

---

## Interfaces

```unk id="iface001"
pub interface Drawable {
    func draw()
}
```

Implementation:

```unk id="iface002"
pub class Circle implements Drawable {
    pub func draw() {
        print("drawing")
    }
}
```

---

## Enums

```unk id="enum001"
pub enum Direction {
    North,
    South,
    East,
    West
}
```

Enums with values:

```unk id="enum002"
pub enum Shape {
    Circle(radius: Float),
    Rect(w: Float, h: Float)
}
```

---

## Generics

```unk id="gen001"
pub class Stack<T> {
    mut items: Array<T> = []

    pub func push(item: T) {
        self.items.append(item)
    }
}
```

Generic functions:

```unk id="gen002"
func max<T>(a: T, b: T) -> T {
    return a > b ? a : b
}
```

---

## Error Handling

Inline fallback:

```unk id="err001"
let content = readFile("test.txt") or ""
```

Early return:

```unk id="err002"
let user = findUser(id) or return null
```

Throwing:

```unk id="err003"
func connect() throws (NetworkError) {
    throw NetworkError("offline")
}
```

Try/catch:

```unk id="err004"
try {
    risky()
} catch (e: Error) {
    print(e)
} finally {
    print("automatically run")
}
```

---

## Async

```unk id="async001"
async func fetchUser(id: Int) -> User {
    return await api.get(id)
}
```

Parallel tasks:

```unk id="async002"
let a = spawn fetchUser(1)
let b = spawn fetchUser(2)

let [u1, u2] = await all(a, b)
```

---

## Functional Features

```unk id="fp001"
nums
    .map((x) => x * 2)
    .filter((x) => x > 3)
```

---

## Extensions

```unk id="ext001"
pub class StringExtensions on String {
    pub inline func shout(self: String) -> String {
        return self.toUpperCase()
    }
}
```

Enable extensions:

```unk id="ext002"
using StringExtensions

print("hello".shout())
```

---

## Defer

```unk id="defer001"
func load() {
    let file = open("data.txt")

    defer file.close()

    print(file.readAll())
}
```

---

## Imports

```unk id="imp001"
import Math
```

```unk id="imp002"
from Math import sqrt, PI
```

Aliases:

```unk id="imp003"
import Math as NeverGonnaGiveYouUp
```

---

## Compile-Time

```unk id="comp001"
comptime let PLATFORM = os.name() // fixed value after build
```

Compile-time conditionals:

```unk id="comp002"
comptime if (PLATFORM == "windows") {
    print("Windows")
}
```

---

## Compiler Bootstrap AST

Phase 0 of the compiler models only the syntax needed for the first parser and AST printer:

- integer expressions
- identifier expressions
- binary expressions

The bootstrap AST keeps source spans on every expression so parser errors and tree dumps can point back to source text.

Later phases extend this tree with declarations, types, functions, control flow, classes, enums, generics, async, unsafe, and FFI.

---

## Unsafe

```unk id="unsafe001"
unsafe {
    let ptr = alloc<Int>(1)
    ptr.write(0, 42)
    free(ptr)
}
```

### Overview

`unsafe` is an explicit escape hatch. The compiler cannot verify safety inside an `unsafe` block, so the programmer takes responsibility for correctness. The type system, scoping rules, and visibility modifiers remain fully enforced — only five specific guarantees are relaxed.

### What `unsafe` unlocks

Exactly five capabilities:

1. Raw pointer operations — dereferencing, arithmetic, casting
2. Manual memory — `alloc`, `realloc`, `free`
3. Calling `unsafe func` functions
4. Implementing `unsafe interface` interfaces
5. Mutating `static mut` globals

Everything else is enforced as normal inside `unsafe` blocks.

### Raw pointer types

Two pointer types, constructable only inside `unsafe`:

```unk id="unsafe002"
RawPtr<T>               // read/write pointer
Nullable<RawPtr<T>>     // nullable read/write pointer
ConstPtr<T>             // read-only pointer
```

Pointers are never automatically dereferenced. All access is through explicit method calls:

```unk id="unsafe003"
unsafe {
    let p: RawPtr<Int> = alloc<Int>(4)   // allocate 4 ints

    p.write(0, 10)                        // write at offset
    p.write(1, 20)
    let v = p.read(0)                     // read at offset

    let q = p.offset(2)                   // pointer arithmetic → RawPtr<Int>
    let r = p.cast<Float32>()             // type-punning cast

    realloc(p, 8)                         // grow allocation
    free(p)                               // manual deallocation
}
```

`p.read(i)` and `p.write(i, v)` are the only access points. There is no dereference syntax.

### Unsafe functions

Functions that require an `unsafe` block at the call site are declared with `unsafe func`:

```unk id="unsafe004"
unsafe func transmute<From, To>(value: From) -> To { ... }
unsafe func volatileRead<T>(ptr: ConstPtr<T>) -> T { ... }
unsafe func atomicAdd(ptr: RawPtr<Int32>, delta: Int32) -> Int32 { ... }
```

Calling an `unsafe func` outside an `unsafe` block is a compile error:

```unk id="unsafe005"
let x = transmute<Int32, Float32>(bits)      // compile error

unsafe {
    let x = transmute<Int32, Float32>(bits)  // no compile error
}
```

A function that contains `unsafe` internally but exposes a safe public signature is the standard pattern for building safe abstractions over unsafe primitives:

```unk id="unsafe006"
// safe to call — unsafe is contained inside
func reinterpretBits(n: Int32) -> Float32 {
    unsafe {
        return transmute<Int32, Float32>(n)
    }
}
```

### Unsafe interfaces

An interface marked `unsafe interface` signals that implementors must uphold invariants the compiler cannot verify — typically memory layout contracts or aliasing rules:

```unk id="unsafe007"
pub unsafe interface Allocator {
    unsafe func alloc(size: Int) -> RawPtr<UInt8>
    unsafe func free(ptr: RawPtr<UInt8>)
}
```

Implementing an unsafe interface requires `unsafe impl`:

```unk id="unsafe008"
pub unsafe impl SystemAllocator implements Allocator {
    pub unsafe func alloc(size: Int) -> RawPtr<UInt8> { ... }
    pub unsafe func free(ptr: RawPtr<UInt8>) { ... }
}
```

Calling methods on an `Allocator` value must be done inside `unsafe`.

### Static mutable globals

`static mut` globals require an `unsafe` block for both reads and writes, because concurrent access is inherently racy:

```unk id="unsafe009"
static mut INSTANCE_COUNT: Int32 = 0

func register() {
    unsafe {
        INSTANCE_COUNT += 1      // no compile error
    }
}

let n = INSTANCE_COUNT           // compile error
```

### FFI

External C functions are declared with `extern "C"` and implicitly `unsafe func`. The declaration block is safe; the call site is not:

```unk id="unsafe010"
extern "C" {
    unsafe func malloc(size: Int) -> RawPtr<UInt8>
    unsafe func memcpy(dst: RawPtr<UInt8>, src: ConstPtr<UInt8>, n: Int) -> RawPtr<UInt8>
    unsafe func strlen(s: ConstPtr<UInt8>) -> Int
}

func copyBytes(dst: RawPtr<UInt8>, src: ConstPtr<UInt8>, n: Int) {
    unsafe {
        memcpy(dst, src, n)
    }
}
```

### `unsafe` as an expression

`unsafe` blocks are expressions and can return a value:

```unk id="unsafe011"
let bits: UInt32 = unsafe { transmute<Float32, UInt32>(3.14) }
```

### What `unsafe` does not relax

- The type system — all types are still checked
- `Nullable<T>` — unwrapping still requires `??` or `if let`
- Integer overflow — use explicit wrapping ops (`a.wrappingAdd(b)`) if needed
- Error handling — `throws` functions still require `try`
- Visibility — `pub`/private access rules still apply

---

## FixedArray

`FixedArray<A, T>` is a fixed-size array whose length `A` must be known at compile time. Like `Array<T>`, it is a reference type — assigning or passing a `FixedArray` shares the same underlying buffer.

```unk id="farr001"
let buf: FixedArray<4, Float32> = [0.0, 0.0, 0.0, 1.0]
```

The size parameter accepts any `comptime` expression:

```unk id="farr002"
const N = 8
let data: FixedArray<N * 2, Int> = [0; 16]   // filled with 16 zeros
```

---

### Index Access

```unk id="farr003"
let first = buf[0]
buf[1] = 3.14
```

Out-of-bounds access is a compile error when the index is a `comptime` value, and a runtime panic otherwise.

---

### Iteration

```unk id="farr004"
for (val in buf) {
    print(val)
}
```

---

### Slicing into Array\<T\>

A `FixedArray` can be sliced into a dynamic `Array<T>`. The slice is a copy:

```unk id="farr005"
let arr: Array<Float32> = buf.slice()          // full copy
let partial: Array<Float32> = buf.slice(1..3)  // elements at index 1 and 2
```

---

### Unsafe Raw Pointer Access

A `RawPtr<T>` to the underlying buffer can be obtained inside an `unsafe` block:

```unk id="farr006"
unsafe {
    let ptr: RawPtr<Float32> = buf.rawPtr()
    ptr.write(0, 99.0)
}
```

The pointer is valid only for the lifetime of the `FixedArray`. Accessing it after the array is freed is undefined behavior.

---
