# Iris Language – Core Design (v1 Draft)

Iris is an interpreted, VM-based language focused on:

- Modern expressiveness (ADTs, pattern matching, error sets)
- Purity by default with explicit effects
- Zig-like error model (`T!`, `T?`)
- Extensible effect system
- Strong FFI integration with Rust
- Backend-friendly architecture without framework magic

---

1. Execution Model

---

- Interpreted by a virtual machine (VM).
- Modules are loaded and verified before execution.
- A static effect checker validates purity and effect propagation.
- Native plugins (Rust) integrate via a controlled ABI.

---

2. Values and Types

---

Iris has two categories of types:

2.1 Value Types (passed by copy)

- Int
- Bool
- Float
- Char
- Small builtin scalar types

Characteristics:

- No identity.
- Compared by value using `==`.
- Cheap to copy.

  2.2 Object Types (passed by reference)

- String
- Bytes
- List[T]
- Map[K,V]
- User-defined `struct`
- User-defined `enum`
- Opaque/native handles

Characteristics:

- Have identity (reference semantics).
- Immutable by default.
- Passed by reference.
- `==` compares identity unless the type implements `Eq`.

---

3. Equality Semantics

---

Default behavior of `==`:

- For value types → value comparison.
- For object types → reference (identity) comparison.
- If object type implements trait `Eq`, then `==` dispatches to `Eq.eq`.

Example:

```
trait Eq[T] {
    fn eq(a: T, b: T) -> Bool
}

impl Eq[User] {
    fn eq(a: User, b: User) -> Bool {
        a.id == b.id
    }
}
```

Identity comparison operator may be provided (`===` or `is`).

---

4. User-Defined Types

---

4.1 Struct

```
struct User {
    id: Int
    email: String
}
```

- Immutable by default.
- Update via copy-with syntax:

```
let u2 = u { email = "new@mail.com" }
```

4.2 Enum (Algebraic Data Type)

```
enum Role {
    Admin
    Member
}

enum Expr {
    IntLit(value: Int)
    Add(lhs: Expr, rhs: Expr)
}
```

- Pattern matching must be exhaustive.

---

5. Pattern Matching

---

```
match expr {
    .Variant(x) => ...
    .Other => ...
}
```

- Exhaustiveness required for enums.
- Wildcard `_` allowed.

---

6. Error System (Zig-like)

---

6.1 Error Declaration

```
error {
    InvalidInput
    NotFound
}

error Io {
    NotFound
    PermissionDenied
    BrokenPipe
}
```

Errors are:

- Tags (no payload).
- Interned symbols: `error.Name` or `error.Group.Name`.

  6.2 Fallible Functions

```
fn read_file(path: PathBuf) -> String! { ... }
```

- `!` means function may return an error.
- Error set is inferred:
  - Includes explicit `return error.X`
  - Includes errors from `try` calls
  - Excludes errors handled by `catch`

  6.3 try / catch

```
let x = try parse_int(s)

let port = env.get("PORT") catch "8080"

let value = db.get(id) catch |e| match e {
    error.Db.NotFound => default_value,
    _ => return e,
}
```

Errors are orthogonal to effects.

---

7. Optionals

---

`T?` is equivalent to `Option[T]`.

Example:

```
fn find_user(id: Int) -> User?
```

Pattern match:

```
match find_user(id) {
    Some(u) => ...
    None => ...
}
```

---

8. Purity and Effects

---

8.1 Pure by Default

Functions are pure unless they declare effects.

```
fn parse(s: String) -> Int! { ... }   // pure
```

8.2 Declaring Effects

```
fn read_file(path: PathBuf) -> String! with { fs }

fn handler(req: Req) -> Resp! with { io, time }
```

8.3 Effect Groups (Aliases)

```
effect HandlerEffects { io, time, net }

fn handle(req: Req) -> Resp! with HandlerEffects
```

Effect groups expand statically.

8.4 Primitive (Sealed) Effects

Declared only by stdlib/host:

```
effect io   { native, sealed }
effect fs   { native, sealed, requires { io } }
effect net  { native, sealed, requires { io } }
effect time { native, sealed }
effect rand { native, sealed }
effect ffi  { native, sealed }
```

Frameworks cannot declare sealed/native effects.

8.5 Effect Checking Rule

For a call to be valid:

callee.effects ⊆ caller.effects (after alias expansion)

8.6 Effects as Part of Function Type

```
fn() -> Int
fn() -> Int with { io }
```

These are distinct types.

---

9. Framework-Defined Effects

---

Frameworks may define abstract effects:

```
effect cache {
    fn get(key: String) -> Bytes?
    fn put(key: String, val: Bytes, ttl: Int) -> Void
}
```

Functions requiring them:

```
fn handler(req: Req) -> Resp! with { cache }
```

---

10. Effect Handlers

---

Handlers provide implementations for abstract effects:

```
fn main() -> Void! with { net, time } {
    let redis = make_cache_handler("redis://...") catch return
    with cache = redis {
        server.run(handler)
    }
}
```

- `with effect = handler { ... }` installs handler in scope.
- Handler may internally use primitive effects.

---

11. Native / Rust Integration

---

11.1 Native Function Declaration

```
@native(module="redis", symbol="connect") with { net }
fn redis_connect(url: String) -> RedisConn!
```

- Must declare required primitive effects.
- Error set inferred from runtime mapping.

  11.2 Opaque Types

```
opaque RedisConn
```

Used for native handles.

11.3 Plugin Loading

Host decides allowed primitive effects per plugin:

```
load_plugin("redis", allow={ net, time })
```

Prevents privilege escalation.

---

12. Traits

---

Nominal traits.

```
trait Eq[T] {
    fn eq(a: T, b: T) -> Bool
}

trait Hash[T] {
    fn hash(x: T) -> Int
}
```

Traits may be implemented for user types.

---

13. Generics

---

Parametric generics:

```
struct Vec[T] { ... }

fn map[T,U](xs: Vec[T], f: fn(T)->U) -> Vec[U]
```

Function types include effect sets:

```
fn(T) -> U with { io }
```

---

14. Identity vs Value

---

- Objects have identity.
- `==`:
  - Value types → value compare
  - Object types → identity compare
  - If implements `Eq` → dispatch to trait

Optional identity operator:

```
a === b
```

---

15. Mutability Model

---

- All user-defined types immutable by default.
- Updates create new instances.
- Controlled mutation may be introduced via stdlib types (e.g., `Ref[T]`) and may require effect `{ state }` (optional future design).

---

16. Design Principles

---

- Purity by default.
- Explicit side effects.
- Explicit failure.
- No hidden framework magic.
- Effects are static and verifiable.
- Native powers are sealed.
- Extensibility via abstract effects and handlers.
- Backend-oriented but general-purpose.

---

End of Iris v1 Core Design Draft.
