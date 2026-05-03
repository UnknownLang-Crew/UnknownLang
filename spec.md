**UnknownLang**

Language Specification

_Cool Lang Maype_

Inspired by **C# • Rust • Swift • Python • GDScript • Lua**

# **1\. Philosophy**

UnknownLang is a statically typed, compiled language designed to be easy to learn and powerful when needed.

### **Core principles**

- Private by default - visibility is opt-in with pub
- Immutable by default - mutation is opt-in with mut
- Explicit over implicit - errors, types, and side effects are visible
- One CLI - nova run, nova test, nova fmt, nova add - zero config to start
- Grows with you - beginners use simple forms, experts unlock generics, unsafe, and comptime

# **2\. Variables**

| _// Immutable - cannot be reassigned_     |
| ----------------------------------------- |
| **let** name = "Nova"                     |
| **let** count: int = 42                   |
| &nbsp;                                    |
| _// Mutable - can be reassigned_          |
| **mut** score = 0.0                       |
| **mut** items: Array&lt;String&gt; = \[\] |
| &nbsp;                                    |
| _// Compile-time constant_                |
| **const** PI = 3.14159                    |
| **const** MAX_RETRIES: int = 5            |

- let bindings are immutable after assignment
- mut bindings can be reassigned
- const must be a compile-time literal or comptime expression
- Type annotations are optional when the type can be inferred

# **3\. Types**

### **Primitive types**

| **Type** | **Description** | **Example**      |
| -------- | --------------- | ---------------- |
| int      | 64-bit integer  | 42               |
| float    | 64-bit float    | 3.14             |
| bool     | Boolean         | true, false      |
| str      | UTF-8 string    | "hello"          |
| char     | Unicode scalar  | 'a'              |
| null     | Null value      | null             |
| Any      | Dynamic type    | any value        |
| Void     | No value        | return type only |

### **Collection types**

| **let** nums: Array&lt;int&gt; = \[1, 2, 3\]                        |
| ------------------------------------------------------------------- |
| **let** map: Map&lt;str, int&gt; = { "a": 1, "b": 2 }               |
| **let** **set**: Set&lt;str&gt; = Set(\["a", "b"\])                 |
| **let** pair: (int, str) = (42, "hello") _// tuple_                 |
| **let** fn: **func**(int) -> int = (x) => x \* 2 _// function type_ |

### **Optional types**

| **let** name: str? = **null** _// may be null_  |
| ----------------------------------------------- |
| **let** name: str? = "Nova" _// or a value_     |
| &nbsp;                                          |
| _// Unwrap with if let_                         |
| **if** (**let** n = name) {                     |
| print(n.toUpperCase())                          |
| }                                               |
| &nbsp;                                          |
| _// Null coalescing_                            |
| **let** display = name ?? "Unknown"             |
| &nbsp;                                          |
| _// Optional chaining_                          |
| **let** city = user?.address?.city ?? "Unknown" |

### **Type aliases**

| **type** UserId = int                                  |
| ------------------------------------------------------ |
| **type** Callback = **func**(int) -> bool              |
| **type** Handler = **func**(Request, Response) -> Void |
| &nbsp;                                                 |
| _// Generic alias_                                     |
| **type** Pair&lt;A, B&gt; = (A, B)                     |
| **type** Results&lt;T&gt; = Array&lt;Result<T&gt;>     |

# **4\. Functions**

| _// Basic function_                                                      |
| ------------------------------------------------------------------------ |
| **func** greet(name: str) -> str {                                       |
| **return** "Hello, {name}!"                                              |
| }                                                                        |
| &nbsp;                                                                   |
| _// Expression body_                                                     |
| **func** double(x: int) -> int => x \* 2                                 |
| &nbsp;                                                                   |
| _// Default parameters_                                                  |
| **func** connect(host: str, port: int = 8080, tls: bool = **false**) { } |
| connect("localhost")                                                     |
| connect("localhost", port: 443, tls: **true**)                           |
| &nbsp;                                                                   |
| _// Named arguments_                                                     |
| greet(name: "World")                                                     |
| &nbsp;                                                                   |
| _// Variadic_                                                            |
| **func** sum(...nums: int) -> int {                                      |
| **return** nums.reduce(0, (acc, n) => acc + n)                           |
| }                                                                        |
| &nbsp;                                                                   |
| _// Lambdas_                                                             |
| **let** add = (a: int, b: int) => a + b                                  |
| **let** double = (x: int) => x \* 2                                      |
| &nbsp;                                                                   |
| _// Higher-order_                                                        |
| **func** apply(x: int, f: **func**(int) -> int) -> int => f(x)           |
| apply(5, (n) => n \* 2) _// 10_                                          |

# **5\. Strings**

| _// Interpolation_                              |
| ----------------------------------------------- |
| **let** s = "Hello, {name}! Count: {n}"         |
| **let** expr = "Area: {3.14 \* r \* r}"         |
| &nbsp;                                          |
| _// Multiline_                                  |
| **let** html = """                              |
| &lt;div&gt;                                     |
| &lt;p&gt;Hello&lt;/p&gt;                        |
| &lt;/div&gt;                                    |
| """                                             |
| &nbsp;                                          |
| _// Raw strings - no escapes, no interpolation_ |
| **let** path = r"C:\\Users\\Nova\\Documents"    |
| **let** regex = r"\\d+\\.\\d+"                  |

### **Escape sequences**

| **Escape** | **Meaning**        |
| ---------- | ------------------ |
| \\n        | Newline            |
| \\t        | Tab                |
| \\\\       | Backslash          |
| \\"        | Quote              |
| \\{        | Literal {          |
| \\uXXXX    | Unicode code point |

# **6\. Control Flow**

| _// if / else_                                             |
| ---------------------------------------------------------- |
| **if** (score > 100) {                                     |
| print("Winner!")                                           |
| } **else** **if** (score > 50) {                           |
| print("Good!")                                             |
| } **else** {                                               |
| print("Keep going!")                                       |
| }                                                          |
| &nbsp;                                                     |
| _// if as expression_                                      |
| **let** label = **if** (score > 50) "Pass" **else** "Fail" |
| &nbsp;                                                     |
| _// for - C-style_                                         |
| **for** (**let** i = 0; i < 10; i++) { print(i) }          |
| &nbsp;                                                     |
| _// for - iterate_                                         |
| **for** (item **in** myList) { process(item) }             |
| &nbsp;                                                     |
| _// for - range_                                           |
| **for** (i **in** 0..10) { print(i) } _// 0-9_             |
| **for** (i **in** 0..=10) { print(i) } _// 0-10 inclusive_ |
| &nbsp;                                                     |
| _// while / loop_                                          |
| **while** (running) { update() }                           |
| **loop** { **if** (done) { **break** } }                   |
| &nbsp;                                                     |
| _// Labelled break_                                        |
| outer: **for** (i **in** 0..5) {                           |
| **for** (j **in** 0..5) {                                  |
| **if** (i == j) { **break** outer }                        |
| }                                                          |
| }                                                          |

# **7\. Pattern Matching**

| **match** (x) {                                    |
| -------------------------------------------------- |
| **case** 1: print("one")                           |
| **case** 2, 3: print("two or three")               |
| **case** 4..10: print("four to ten")               |
| **case** n: print("other: {n}")                    |
| }                                                  |
| &nbsp;                                             |
| _// Tuple destructuring_                           |
| **match** (point) {                                |
| **case** (0, 0): print("Origin")                   |
| **case** (x, 0): print("On X-axis")                |
| **case** (y, z) **as** v: print("Point {v}")       |
| **case** \_: print("Somewhere")                    |
| }                                                  |
| &nbsp;                                             |
| _// Enum variants_                                 |
| **match** (shape) {                                |
| **case** Circle(r): print("area={3.14 \* r \* r}") |
| **case** Rect(w, h): print("area={w \* h}")        |
| **case** \_: print("other")                        |
| }                                                  |
| &nbsp;                                             |
| _// Type dispatch_                                 |
| **match** (**type**(value)) {                      |
| **case** Number **as** n: print("Number: {n}")     |
| **case** String **as** s: print("String: {s}")     |
| **case** Dog **as** d: d.bark()                    |
| **case** \_: print("Something else")               |
| }                                                  |
| &nbsp;                                             |
| _// Guards_                                        |
| **match** (score) {                                |
| **case** x **if** (x >= 90): print("A")            |
| **case** x **if** (x >= 75): print("B")            |
| **case** x: print("Fail")                          |
| }                                                  |
| &nbsp;                                             |
| _// match as expression_                           |
| **let** label = **match** (status) {               |
| **case** 200: "OK"                                 |
| **case** 404: "Not Found"                          |
| **case** x: "Status {x}"                           |
| }                                                  |

# **8\. Classes**

| **pub** **class** Animal {                                         |
| ------------------------------------------------------------------ |
| _// Private fields_                                                |
| **mut** \_name: str                                                |
| **mut** \_health: int = 100                                        |
| &nbsp;                                                             |
| _// Public fields_                                                 |
| **pub** **let** id: int                                            |
| **pub** **mut** nickname: str = ""                                 |
| &nbsp;                                                             |
| **pub** **func** init(id: int, name: str) {                        |
| **self**.id = id                                                   |
| **self**.\_name = name                                             |
| }                                                                  |
| &nbsp;                                                             |
| **pub** **func** speak() -> str { **return** "..." }               |
| **func** validate() -> bool => **self**.\_health >= 0 _// private_ |
| }                                                                  |
| &nbsp;                                                             |
| _// Inheritance_                                                   |
| **pub** **class** Dog **extends** Animal {                         |
| **pub** **let** breed: str                                         |
| &nbsp;                                                             |
| **pub** **func** init(id: int, name: str, breed: str) {            |
| **super**(id, name)                                                |
| **self**.breed = breed                                             |
| }                                                                  |
| &nbsp;                                                             |
| **override** **pub** **func** speak() -> str => "Woof!"            |
| }                                                                  |

_override is required to override a parent method. Omitting it is a compile error._

_super(…) must be the first statement in init._

# **9\. Interfaces**

| **pub** **interface** Drawable {                                                      |
| ------------------------------------------------------------------------------------- |
| **func** draw(canvas: Canvas)                                                         |
| **func** bounds() -> Rect                                                             |
| }                                                                                     |
| &nbsp;                                                                                |
| _// Default implementations_                                                          |
| **pub** **interface** Printable {                                                     |
| **func** toString() -> str                                                            |
| &nbsp;                                                                                |
| **func** print() { _// default method_                                                |
| print(**self**.toString())                                                            |
| }                                                                                     |
| }                                                                                     |
| &nbsp;                                                                                |
| _// Implement multiple interfaces_                                                    |
| **pub** **class** Circle **implements** Drawable, Printable {                         |
| **pub** **func** draw(canvas: Canvas) { ... }                                         |
| **pub** **func** bounds() -> Rect { ... }                                             |
| **pub** **func** toString() -> str => "Circle(r={self.radius})"                       |
| }                                                                                     |
| &nbsp;                                                                                |
| _// Extend + implement_                                                               |
| **pub** **class** Player **extends** Entity **implements** Drawable, Serializable { } |
| &nbsp;                                                                                |
| _// Interface as type_                                                                |
| **func** render(item: Drawable) {                                                     |
| item.draw(canvas)                                                                     |
| }                                                                                     |

# **10\. Enums**

| _// Simple enum_                                        |
| ------------------------------------------------------- |
| **pub** **enum** Direction { North, South, East, West } |
| &nbsp;                                                  |
| _// Enum with payloads (tagged union)_                  |
| **pub** **enum** Shape {                                |
| Circle(radius: float),                                  |
| Rect(w: float, h: float),                               |
| Triangle(base: float, height: float)                    |
| }                                                       |
| &nbsp;                                                  |
| _// Enum with methods_                                  |
| **pub** **enum** Color {                                |
| Red, Green, Blue,                                       |
| Custom(r: int, g: int, b: int)                          |
| &nbsp;                                                  |
| **pub** **func** toHex() -> str {                       |
| **match** (**self**) {                                  |
| **case** Red: **return** "#FF0000"                      |
| **case** Custom(r, g, b): **return** "#{r}{g}{b}"       |
| **case** \_: **return** "#000000"                       |
| }                                                       |
| }                                                       |
| }                                                       |

# **11\. Generics**

| _// Generic function_                                                   |
| ----------------------------------------------------------------------- |
| **func** max&lt;T: Comparable&gt;(a: T, b: T) -> T {                    |
| **return** (a > b) ? a : b                                              |
| }                                                                       |
| &nbsp;                                                                  |
| _// Multiple constraints_                                               |
| **func** merge&lt;T: Cloneable & Serializable&gt;(a: T, b: T) -> T { }  |
| &nbsp;                                                                  |
| _// Generic class_                                                      |
| **pub** **class** Stack&lt;T&gt; {                                      |
| **mut** items: Array&lt;T&gt; = \[\]                                    |
| &nbsp;                                                                  |
| **pub** **func** push(item: T) { **self**.items.append(item) }          |
| **pub** **func** pop() -> T? { **return** **self**.items.removeLast() } |
| **pub** **get** top: T? => **self**.items.last()                        |
| **pub** **get** isEmpty: bool => **self**.items.length() == 0           |
| }                                                                       |
| &nbsp;                                                                  |
| _// Where clause_                                                       |
| **func** serialize&lt;T&gt;(items: Array&lt;T&gt;) -> str               |
| **where** T: Serializable {                                             |
| **return** items.map((x) => x.toJson()).join(",")                       |
| }                                                                       |

# **12\. Properties & Access**

### **Access modifiers**

| **Syntax**            | **Readable from** | **Writable from** |
| --------------------- | ----------------- | ----------------- |
| let x                 | same class        | init only         |
| mut x                 | same class        | same class        |
| pub let x             | anywhere          | init only         |
| pub mut x             | anywhere          | anywhere          |
| pub get x             | anywhere          | no setter         |
| pub get x + set x     | anywhere          | same class        |
| pub get x + pub set x | anywhere          | anywhere          |
| pub static let x      | anywhere          | never             |
| pub static mut x      | anywhere          | anywhere          |

### **Computed properties**

| **pub** **class** Circle {                                             |
| ---------------------------------------------------------------------- |
| **pub** **let** radius: float                                          |
| &nbsp;                                                                 |
| **pub** **get** area: float {                                          |
| **return** 3.14159 \* **self**.radius \* **self**.radius               |
| }                                                                      |
| **pub** **get** diameter: float => **self**.radius \* 2 _// shorthand_ |
| }                                                                      |
| &nbsp;                                                                 |
| _// pub get + private set_                                             |
| **pub** **class** Player {                                             |
| **pub** **get** score: int => **self**.\_score                         |
| **set** score(v: int) { **self**.\_score = max(0, v) }                 |
| &nbsp;                                                                 |
| **pub** **func** addPoints(n: int) {                                   |
| **self**.score += n _// private set - allowed internally_              |
| }                                                                      |
| }                                                                      |
| &nbsp;                                                                 |
| _// Lazy - computed once, then cached_                                 |
| **pub** **get** **lazy** summary: str {                                |
| **return** **self**.computeExpensiveSummary()                          |
| }                                                                      |

# **13\. Static Extensions**

Static extension classes add methods to existing types without subclassing. using activates them in scope.

| **pub** **static** **class** StringExtensions **on** String {                                     |
| ------------------------------------------------------------------------------------------------- |
| **pub** **static** **func** wordCount(**self**: str) -> int {                                     |
| **return** **self**.split(" ").length()                                                           |
| }                                                                                                 |
| **pub** **static** **func** truncate(**self**: str, max: int) -> str {                            |
| **if** (**self**.length() <= max) { **return** **self** }                                         |
| **return** **self**.slice(0, max) + "..."                                                         |
| }                                                                                                 |
| }                                                                                                 |
| &nbsp;                                                                                            |
| _// Generic extension_                                                                            |
| **pub** **static** **class** ArrayExtensions&lt;T&gt; **on** Array&lt;T&gt; {                     |
| **pub** **static** **func** second(**self**: Array&lt;T&gt;) -> T? {                              |
| **return** **self**.length() > 1 ? **self**\[1\] : **null**                                       |
| }                                                                                                 |
| **pub** **static** **func** chunk(**self**: Array&lt;T&gt;, size: int) -> Array&lt;Array<T&gt;> { |
| _// ..._                                                                                          |
| }                                                                                                 |
| }                                                                                                 |
| &nbsp;                                                                                            |
| _// Activate - file-level_                                                                        |
| **using** StringExtensions, ArrayExtensions                                                       |
| &nbsp;                                                                                            |
| "hello world".wordCount() _// 2_                                                                  |
| \[1,2,3,4,5\].chunk(2) _// \[\[1,2\],\[3,4\],\[5\]\]_                                             |
| &nbsp;                                                                                            |
| _// Block-scoped - only active inside the block_                                                  |
| {                                                                                                 |
| **using** DebugExtensions                                                                         |
| myObj.prettyPrint()                                                                               |
| }                                                                                                 |
| myObj.prettyPrint() _// compile error - not in scope_                                             |

_str.wordCount() compiles to StringExtensions.wordCount(str) - no runtime cost, no monkey-patching._

# **14\. Error Handling**

| _// try / catch (T, ...) as e_                                                          |
| --------------------------------------------------------------------------------------- |
| **try** {                                                                               |
| **let** file = fs.open("data.txt")                                                      |
| process(file.readAll())                                                                 |
| } **catch** (TimeoutError) **as** e {                                                   |
| print("Timed out: {e.after}ms")                                                         |
| } **catch** (AuthError, ForbiddenError) **as** e {                                      |
| redirect("/login")                                                                      |
| } **catch** (Error) **as** e {                                                          |
| print("Unexpected: {e}")                                                                |
| } **finally** {                                                                         |
| cleanup()                                                                               |
| }                                                                                       |
| &nbsp;                                                                                  |
| _// try as expression_                                                                  |
| **let** port = **try** {                                                                |
| json.parse(fs.readFile("config.json"))\["port"\].asInt()                                |
| } **catch** (IOError, ParseError) **as** e {                                            |
| 8080                                                                                    |
| }                                                                                       |
| &nbsp;                                                                                  |
| _// or - inline fallback_                                                               |
| **let** content = fs.readFile("readme.txt") **or** ""                                   |
| **let** port = config.getInt("port") **or** 8080                                        |
| **let** cfg = fs.readFile("app.config") **or** **throw** StartupError("Missing")        |
| &nbsp;                                                                                  |
| _// or return_                                                                          |
| **func** loadUser(id: int) -> User? {                                                   |
| **let** row = db.query("SELECT \* FROM users WHERE id={id}") **or** **return** **null** |
| **return** User.fromRow(row) **or** **return** **null**                                 |
| }                                                                                       |
| &nbsp;                                                                                  |
| _// throws - compiler-enforced_                                                         |
| **func** connect(url: str) **throws** (TimeoutError, NetworkError) {                    |
| **throw** NetworkError("No connection")                                                 |
| }                                                                                       |
| &nbsp;                                                                                  |
| _// Custom error types_                                                                 |
| **error** NetworkError {                                                                |
| Timeout(after: int),                                                                    |
| NotFound(url: str),                                                                     |
| Unauthorized                                                                            |
| }                                                                                       |

# **15\. Async & Concurrency**

| _// Async function_                                                       |
| ------------------------------------------------------------------------- |
| **async** **func** fetchUser(id: int) **throws** (NetworkError) -> User { |
| **let** resp = **await** http.**get**("/users/{id}")                      |
| **return** json.parse&lt;User&gt;(resp.body)                              |
| }                                                                         |
| &nbsp;                                                                    |
| _// Parallel tasks_                                                       |
| **let** taskA = **spawn** fetchUser(1)                                    |
| **let** taskB = **spawn** fetchUser(2)                                    |
| **let** \[a, b\] = **await** all(taskA, taskB)                            |
| &nbsp;                                                                    |
| _// Race - first wins_                                                    |
| **let** fastest = **await** race(taskA, taskB)                            |
| &nbsp;                                                                    |
| _// Structured concurrency_                                               |
| **scope** {                                                               |
| **spawn** watchdog()                                                      |
| **spawn** mainLoop()                                                      |
| **await** done                                                            |
| } _// all tasks cancelled on exit_                                        |
| &nbsp;                                                                    |
| _// Channels_                                                             |
| **let** ch = **new** Channel&lt;int&gt;()                                 |
| **spawn** {                                                               |
| **for** (i **in** 0..10) { ch.send(i) }                                   |
| ch.close()                                                                |
| }                                                                         |
| **for** (val **in** ch) { print(val) }                                    |

# **16\. Type System**

| _// is / is not_                                                       |
| ---------------------------------------------------------------------- |
| 42 **is** Number _// true_                                             |
| "hi" **is** String _// true_                                           |
| **null** **is** Null _// true_                                         |
| \[1,2\] **is** Array _// true_                                         |
| x **is** **not** String                                                |
| &nbsp;                                                                 |
| _// Narrows type inside if_                                            |
| **if** (value **is** String) {                                         |
| print(value.toUpperCase()) _// value is String here_                   |
| }                                                                      |
| &nbsp;                                                                 |
| _// type() - runtime type_                                             |
| **type**(42) _// Number_                                               |
| **type**("hi") _// String_                                             |
| **type**(**null**) _// Null_                                           |
| **type**(x) == Number                                                  |
| **type**(x).name _// "Number"_                                         |
| &nbsp;                                                                 |
| _// cast_                                                              |
| **let** s = cast&lt;String&gt;(value) _// String? - safe_              |
| **let** s = cast!&lt;String&gt;(value) _// String - throws on failure_ |
| **let** n = cast&lt;Number&gt;(value) **or** 0 _// with fallback_      |
| &nbsp;                                                                 |
| _// Type dispatch in match_                                            |
| **match** (**type**(value)) {                                          |
| **case** Number **as** n: print("Number: {n}")                         |
| **case** String **as** s: print("String: {s}")                         |
| **case** \_: print("Unknown")                                          |
| }                                                                      |

# **17\. Destructuring**

| _// Tuple_                                                            |
| --------------------------------------------------------------------- |
| **let** (x, y) = point                                                |
| **let** (min, max) = minMax(\[3,1,4,1,5\])                            |
| **let** (\_, y) = point \_// ignore with \_\_                         |
| &nbsp;                                                                |
| _// Struct / class_                                                   |
| **let** { name, age } = user                                          |
| **let** { name **as** userName, age } = user _// rename_              |
| **let** { address: { city, country } } = user _// nested_             |
| **let** { name, role = "guest" } = user _// with default_             |
| &nbsp;                                                                |
| _// Array_                                                            |
| **let** \[first, second, ...rest\] = \[1, 2, 3, 4, 5\]                |
| &nbsp;                                                                |
| _// Swap_                                                             |
| **mut** a = 1                                                         |
| **mut** b = 2                                                         |
| (a, b) = (b, a)                                                       |
| &nbsp;                                                                |
| _// In function parameters_                                           |
| **func** printPoint((x, y): (int, int)) { ... }                       |
| **func** greetUser({ name, age }: User) { ... }                       |
| &nbsp;                                                                |
| _// In for loops_                                                     |
| **for** ((key, value) **in** myMap) { print("{key} = {value}") }      |
| **for** ((i, item) **in** items.enumerate()) { print("{i}: {item}") } |
| &nbsp;                                                                |
| _// In match_                                                         |
| **match** (user) {                                                    |
| **case** { name, age } **if** (age >= 18): print("{name} is adult")   |
| **case** { name }: print("{name} is minor")                           |
| }                                                                     |

# **18\. Spread Operator**

| _// Arrays_                                                  |
| ------------------------------------------------------------ |
| **let** a = \[1, 2, 3\]                                      |
| **let** b = \[4, 5, 6\]                                      |
| **let** c = \[...a, ...b\] _// \[1,2,3,4,5,6\]_              |
| **let** d = \[0, ...a, 4\] _// \[0,1,2,3,4\]_                |
| &nbsp;                                                       |
| _// Maps_                                                    |
| **let** merged = { ...defaults, ...custom } _// custom wins_ |
| &nbsp;                                                       |
| _// Function calls_                                          |
| **let** args = \[1, 2, 3\]                                   |
| sum(...args) _// same as sum(1, 2, 3)_                       |
| &nbsp;                                                       |
| _// Variadic functions_                                      |
| **func** sum(...nums: int) -> int {                          |
| **return** nums.reduce(0, (acc, n) => acc + n)               |
| }                                                            |
| **func** log(level: str, ...messages: str) { }               |
| &nbsp;                                                       |
| _// Rest in destructuring_                                   |
| **let** \[first, second, ...rest\] = \[1, 2, 3, 4, 5\]       |
| **let** { name, ...others } = user                           |

# **19\. Functional Features**

| _// Lambdas_                                               |
| ---------------------------------------------------------- |
| **let** double = (x: int) => x \* 2                        |
| **let** add = (a: int, b: int) => a + b                    |
| &nbsp;                                                     |
| _// Collection methods_                                    |
| **let** nums = \[1, 2, 3, 4, 5\]                           |
| nums.map((x) => x \* 2) _// \[2,4,6,8,10\]_                |
| nums.filter((x) => x % 2 == 0) _// \[2,4\]_                |
| nums.reduce(0, (acc, x) => acc + x) _// 15_                |
| nums.find((x) => x > 3) _// 4_                             |
| nums.every((x) => x > 0) _// true_                         |
| nums.some((x) => x > 4) _// true_                          |
| nums.flatMap((x) => \[x, x \* 2\])                         |
| nums.filterMap((x) => x > 3 ? x : **null**) _// \[4,5\]_   |
| nums.zip(\["a","b","c"\]) _// \[(1,"a"),(2,"b"),(3,"c")\]_ |
| nums.take(3) _// \[1,2,3\]_                                |
| nums.drop(3) _// \[4,5\]_                                  |
| nums.chunk(2) _// \[\[1,2\],\[3,4\],\[5\]\]_               |
| &nbsp;                                                     |
| _// Pipeline \|>_                                          |
| **let** result = data                                      |
| \|> normalize                                              |
| \|> dedupe                                                 |
| \|> sortBy(score)                                          |
| \|> take(10)                                               |

# **20\. Generators**

| _// Define with func\*_                                                            |
| ---------------------------------------------------------------------------------- |
| func\* range(start: int, end: int) -> Generator&lt;int&gt; {                       |
| **mut** i = start                                                                  |
| **while** (i < end) {                                                              |
| **yield** i                                                                        |
| i++                                                                                |
| }                                                                                  |
| }                                                                                  |
| &nbsp;                                                                             |
| **for** (n **in** range(0, 10)) { print(n) }                                       |
| &nbsp;                                                                             |
| _// Infinite generator_                                                            |
| func\* naturals() -> Generator&lt;int&gt; {                                        |
| **mut** n = 0                                                                      |
| **loop** { **yield** n; n++ }                                                      |
| }                                                                                  |
| naturals().take(5) _// \[0,1,2,3,4\]_                                              |
| &nbsp;                                                                             |
| _// yield from - delegate to another generator_                                    |
| func\* flatten&lt;T&gt;(nested: Array&lt;Generator<T&gt;>) -> Generator&lt;T&gt; { |
| **for** (gen **in** nested) { **yield** **from** gen }                             |
| }                                                                                  |

_Generators implement the Iterable&lt;T&gt; interface - they work with for, map, filter, take, and all collection methods._

# **21\. Defer**

| _// Runs when the current scope exits_                       |
| ------------------------------------------------------------ |
| **func** readFile(path: str) -> str {                        |
| **let** file = fs.open(path)                                 |
| **defer** file.close() _// always runs_                      |
| **return** file.readAll()                                    |
| }                                                            |
| &nbsp;                                                       |
| _// Multiple defers - LIFO order_                            |
| **func** setup() {                                           |
| **let** a = acquireA()                                       |
| **defer** releaseA(a)                                        |
| **let** b = acquireB()                                       |
| **defer** releaseB(b) _// runs first_                        |
| }                                                            |
| &nbsp;                                                       |
| _// defer with a block_                                      |
| **defer** {                                                  |
| log.info("Cleaning up...")                                   |
| db.close()                                                   |
| }                                                            |
| &nbsp;                                                       |
| _// Works even when exceptions are thrown_                   |
| **func** process() {                                         |
| **let** conn = db.connect()                                  |
| **defer** conn.close() _// conn closed even if query throws_ |
| **let** data = conn.query("SELECT ...")                      |
| render(data)                                                 |
| }                                                            |

# **22\. Modules & Imports**

| _// Whole module_                                     |
| ----------------------------------------------------- |
| **import** math                                       |
| **import** "./utils"                                  |
| **import** express _// third-party_                   |
| &nbsp;                                                |
| math.sqrt(16)                                         |
| &nbsp;                                                |
| _// Named imports_                                    |
| **from** math **import** sqrt, PI                     |
| **from** fs **import** readFile, writeFile            |
| **from** "./models" **import** User, Post             |
| &nbsp;                                                |
| sqrt(16) _// no prefix needed_                        |
| &nbsp;                                                |
| _// Multiline_                                        |
| **from** http **import** (                            |
| Request, Response, Router, StatusCode                 |
| )                                                     |
| &nbsp;                                                |
| _// Aliases_                                          |
| **import** numpy **as** np                            |
| **from** postgres **import** connect **as** pgConnect |
| **from** "./models" **import** User **as** UserModel  |
| &nbsp;                                                |
| _// Wildcard_                                         |
| **from** math **import** \*                           |
| &nbsp;                                                |
| _// Exporting_                                        |
| **pub** **class** User { }                            |
| **pub** **func** greet(name: str) -> str { }          |
| **pub** **let** VERSION = "1.0.0"                     |

# **23\. Annotations**

| _// Built-in annotations_                               |
| ------------------------------------------------------- |
| @deprecated("Use newFunc() instead")                    |
| **pub** **func** oldFunc() { }                          |
| &nbsp;                                                  |
| @inline                                                 |
| **func** hotPath(x: int) -> int => x \* 2               |
| &nbsp;                                                  |
| @test                                                   |
| **func** testAddition() {                               |
| assert(add(1, 2) == 3)                                  |
| }                                                       |
| &nbsp;                                                  |
| @serializable                                           |
| **pub** **class** User {                                |
| **pub** **let** name: str                               |
| @serialize(key: "max_retries")                          |
| **pub** **let** maxRetries: int = 3                     |
| @serialize(omitIfNull: **true**)                        |
| **pub** **let** bio: str? = **null**                    |
| }                                                       |
| &nbsp;                                                  |
| _// Custom annotations_                                 |
| **annotation** Route(method: str, path: str)            |
| **annotation** Cache(ttl: int = 60)                     |
| &nbsp;                                                  |
| @Route("GET", "/users")                                 |
| @Cache(ttl: 300)                                        |
| **pub** **func** getUsers(req: Request) -> Response { } |

# **24\. Compile-time (comptime)**

| _// Compile-time constants_                                       |
| ----------------------------------------------------------------- |
| **comptime** **let** PLATFORM = os.name()                         |
| **comptime** **let** BUILD_DATE = datetime.now()                  |
| **comptime** **let** VERSION = git.tag() ?? "dev"                 |
| &nbsp;                                                            |
| _// Dead-code elimination_                                        |
| **comptime** **if** (PLATFORM == "windows") {                     |
| **func** getCachePath() -> str => r"C:\\Users\\AppData"           |
| } **else** {                                                      |
| **func** getCachePath() -> str => "~/.cache/nova"                 |
| }                                                                 |
| &nbsp;                                                            |
| _// Compile-time function_                                        |
| **comptime** **func** factorial(n: int) -> int {                  |
| **if** (n <= 1) { **return** 1 }                                  |
| **return** n \* factorial(n - 1)                                  |
| }                                                                 |
| **const** FACT*10 = factorial(10) *// 3628800\_                   |
| &nbsp;                                                            |
| _// Type inspection at compile time_                              |
| **comptime** **func** fieldNames&lt;T&gt;() -> Array&lt;str&gt; { |
| **return** **type**(T).fields.map((f) => f.name)                  |
| }                                                                 |
| **comptime** **let** userFields = fieldNames&lt;User&gt;()        |

# **25\. Unsafe & Manual Memory**

| _// opt-in unsafe block_                                   |
| ---------------------------------------------------------- |
| **unsafe** {                                               |
| **let** buf = alloc&lt;u8&gt;(1024)                        |
| **defer** free(buf)                                        |
| buf\[0\] = 0xFF                                            |
| }                                                          |
| &nbsp;                                                     |
| _// Pointer types_                                         |
| **let** ptr: Pointer&lt;int&gt; = alloc&lt;int&gt;(1)      |
| ptr.write(0, 99)                                           |
| **let** val = ptr.read(0) _// 99_                          |
| free(ptr)                                                  |
| &nbsp;                                                     |
| _// Pointer arithmetic (unsafe only)_                      |
| **unsafe** {                                               |
| **let** arr = alloc&lt;int&gt;(10)                         |
| **let** p = arr.offset(3) _// pointer to arr\[3\]_         |
| p.write(0, 42)                                             |
| }                                                          |
| &nbsp;                                                     |
| _// Extern - call C functions_                             |
| **extern** **func** malloc(size: int) -> Pointer&lt;u8&gt; |
| **extern** **func** free(ptr: Pointer&lt;u8&gt;)           |
| &nbsp;                                                     |
| _// Fixed-size stack arrays_                               |
| **let** buf: \[u8; 256\] = zeroed()                        |

# **26\. Operator Overloading**

| **pub** **class** Vec2 {                                                                |
| --------------------------------------------------------------------------------------- |
| **pub** **let** x: float                                                                |
| **pub** **let** y: float                                                                |
| &nbsp;                                                                                  |
| **operator** +(other: Vec2) -> Vec2 => Vec2(**self**.x + other.x, **self**.y + other.y) |
| **operator** -(other: Vec2) -> Vec2 => Vec2(**self**.x - other.x, **self**.y - other.y) |
| **operator** \*(s: float) -> Vec2 => Vec2(**self**.x \* s, **self**.y \* s)             |
| **operator** -() -> Vec2 => Vec2(-self.x, -self.y) _// unary_                           |
| **operator** ==(other: Vec2) -> bool => **self**.x == other.x && **self**.y == other.y  |
| **operator** \[\](i: int) -> float {                                                    |
| **match** (i) {                                                                         |
| **case** 0: **return** **self**.x                                                       |
| **case** 1: **return** **self**.y                                                       |
| **case** \_: **throw** IndexError("out of range")                                       |
| }                                                                                       |
| }                                                                                       |
| **operator** str() -> str => "({self.x}, {self.y})"                                     |
| }                                                                                       |
| &nbsp;                                                                                  |
| **let** a = Vec2(1.0, 2.0)                                                              |
| **let** b = Vec2(3.0, 4.0)                                                              |
| **let** c = a + b _// Vec2(4.0, 6.0)_                                                   |
| print(a) _// "(1.0, 2.0)"_                                                              |
| print(a\[0\]) _// 1.0_                                                                  |

Overloadable: + − \* / % == != &lt; &gt; &lt;= &gt;= \[\] () unary- str()

# **27\. Concurrency Primitives**

| _// Mutex&lt;T&gt; - wraps a value, exclusive access_     |
| --------------------------------------------------------- |
| **let** mu = **new** Mutex&lt;int&gt;(0)                  |
| &nbsp;                                                    |
| **lock** (mu) **as** value {                              |
| value += 1                                                |
| }                                                         |
| &nbsp;                                                    |
| _// Atomic&lt;T&gt;_                                      |
| **let** counter = **new** Atomic&lt;int&gt;(0)            |
| counter.increment()                                       |
| counter.add(5)                                            |
| counter.compareAndSwap(expected: 5, **new**: 0)           |
| &nbsp;                                                    |
| _// RwLock&lt;T&gt; - multiple readers, exclusive writer_ |
| **let** **lock** = **new** RwLock&lt;Array<int&gt;>(\[\]) |
| &nbsp;                                                    |
| **lock**.read { (data) => print(data.length()) }          |
| **lock**.write { (data) => data.append(42) }              |

# **28\. Standard Library Overview**

| **Module**  | **Contents**                                         |
| ----------- | ---------------------------------------------------- |
| fs          | readFile, writeFile, open, mkdir, exists, stat, walk |
| http        | get, post, put, delete, Router, Request, Response    |
| json        | parse, stringify, schema validation                  |
| math        | sqrt, pow, sin, cos, log, floor, ceil, PI, E         |
| crypto      | hash.sha256, hmac, random, uuid                      |
| regex       | Regex, match, matchAll, replace                      |
| datetime    | now, parse, format, Duration, Timezone               |
| path        | join, dirname, basename, ext, resolve                |
| process     | argv, env, exit, exec, pid, cwd                      |
| net         | tcp, udp, dns, Socket                                |
| db          | connect, query, transaction (postgres/sqlite/mysql)  |
| log         | debug, info, warn, error, fatal, setLevel            |
| test        | assert, assertEquals, assertThrows, mock, spy        |
| collections | Queue, Deque, LinkedList, PriorityQueue, OrderedMap  |
| encoding    | base64, hex, utf8, gzip, deflate                     |
| io          | stdin, stdout, stderr, Reader, Writer, Buffer        |

# **29\. Package System**

### **nova.toml**

| \[package\]                                           |
| ----------------------------------------------------- |
| name = "my-app"                                       |
| version = "1.0.0"                                     |
| authors = \["Alice &lt;<alice@example.com>&gt;"\]     |
| license = "MIT"                                       |
| &nbsp;                                                |
| \[dependencies\]                                      |
| express = "^2.1.0"                                    |
| postgres = "^1.4.0"                                   |
| uuid = "^0.9.0"                                       |
| &nbsp;                                                |
| \[dev-dependencies\]                                  |
| testtools = "^0.3.0"                                  |
| &nbsp;                                                |
| \[build\]                                             |
| optimize = "release" # "debug" \| "release" \| "size" |
| target = "native" # "native" \| "wasm" \| "embedded"  |

### **CLI commands**

| nova run # run src/main.nova      |
| --------------------------------- |
| nova build # compile to binary    |
| nova build --target wasm          |
| nova test # run @test functions   |
| nova test --filter math           |
| nova fmt # format all .nova files |
| nova fmt --check # CI mode        |
| nova add express # add dependency |
| nova add express@2.1.0            |
| nova remove express               |
| nova update # update all deps     |
| nova docs # generate HTML docs    |
| nova repl # interactive REPL      |

### **Project layout**

| my-app/                |
| ---------------------- |
| ├── nova.toml          |
| ├── src/               |
| │ ├── main.nova        |
| │ ├── models/          |
| │ │ ├── user.nova      |
| │ │ └── post.nova      |
| │ └── utils/           |
| │ └── string.nova      |
| ├── tests/             |
| │ ├── user_test.nova   |
| │ └── string_test.nova |
| └── docs/              |

# **30\. Quick Reference Card**

| _// Variables_                                                                     |
| ---------------------------------------------------------------------------------- |
| **let** x = 42 **mut** y = 0.0 **const** Z = 100                                   |
| **type** UserId = int **type** CB = **func**(int) -> bool                          |
| &nbsp;                                                                             |
| _// Functions_                                                                     |
| **func** f(x: int) -> str { } **func** g(x: int) => x \* 2                         |
| **async** **func** h() -> User { } **func** v(...args: int) { }                    |
| &nbsp;                                                                             |
| _// Classes_                                                                       |
| **pub** **class** Foo **extends** Bar **implements** Baz {                         |
| **pub** **let** id: int **mut** \_val: str                                         |
| **pub** **get** val: str => **self**.\_val                                         |
| **pub** **set** val(v: str) { **self**.\_val = v }                                 |
| **pub** **static** **let** count = 0                                               |
| **override** **pub** **func** toString() -> str => "Foo({self.id})"                |
| }                                                                                  |
| &nbsp;                                                                             |
| _// Match_                                                                         |
| **match** (x) { **case** 0: ... **case** (a,b): ... **case** \_ : ... }            |
| **match** (**type**(x)) { **case** Dog **as** d: d.bark() **case** \_: ... }       |
| &nbsp;                                                                             |
| _// Errors_                                                                        |
| **try** { } **catch** (TypeError, ValueError) **as** e { } **finally** { }         |
| value **or** default value **or** **return** **null** value **or** **throw** Err() |
| **func** f() **throws** (IOError) { **throw** IOError("msg") }                     |
| &nbsp;                                                                             |
| _// Types_                                                                         |
| x **is** String x **is** **not** Null                                              |
| cast&lt;String&gt;(x) cast!&lt;String&gt;(x) **type**(x).name                      |
| &nbsp;                                                                             |
| _// Async_                                                                         |
| **let** t = **spawn** fetchUser(1) **let** \[a,b\] = **await** all(t1, t2)         |
| **scope** { **spawn** f() **spawn** g() **await** done }                           |
| &nbsp;                                                                             |
| _// Functional_                                                                    |
| nums.map((x)=>x\*2).filter((x)=>x>2).reduce(0,(a,x)=>a+x)                          |
| data \|> normalize \|> sort \|> take(10)                                           |
| func\* gen() { **yield** 1; **yield** 2 }                                          |
| &nbsp;                                                                             |
| _// Imports_                                                                       |
| **import** math math.sqrt(16)                                                      |
| **from** math **import** sqrt, PI sqrt(16)                                         |
| **import** numpy **as** np np.array(\[\])                                          |
| **using** StringExtensions "hi".wordCount()                                        |
| &nbsp;                                                                             |
| _// Spread & destructure_                                                          |
| **let** (x, y) = point **let** { name, age } = user                                |
| **let** \[first, ...rest\] = list **let** merged = \[...a, ...b\]                  |
| &nbsp;                                                                             |
| _// Annotations_                                                                   |
| @deprecated("use g()") @test @serializable @inline                                 |
| &nbsp;                                                                             |
| _// Comptime_                                                                      |
| **comptime** **let** PLATFORM = os.name()                                          |
| **comptime** **if** (PLATFORM == "wasm") { }                                       |
