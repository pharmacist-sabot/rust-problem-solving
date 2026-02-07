## PS 01: การเป็นเจ้าของของ self ในฟังก์ชันเมมเบอร์

หัวข้อนี้เราจะพิจารณาคำถามพื้นฐานเกี่ยวกับความหมายของการประกาศเมธอด (method) ที่รับ `self` ว่า

> “การรับ `self` ในเมธอด หมายความว่าฟังก์ชันจะ **take ownership (เป็นเจ้าของ)** ของค่าที่เราส่งเข้าไปจริงหรือไม่?”

คำถามนี้สำคัญต่อการออกแบบ API และการจัดการทรัพยากรใน **Rust** เพราะการเป็นเจ้าของค่าหมายถึงค่าถูกย้าย (moved) เข้าไปในฟังก์ชันและไม่สามารถนำกลับมาใช้จากจุดเรียกเดิมได้

## ตัวอย่างปัญหา

สมมติว่ามีโค้ดฟังก์ชันจากมาตรฐานไลบรารี (Crate std)

```rust
pub fn cos(self) -> f32
```

เมื่อเรียกเมธอดนี้

```rust
fn main() {
    let angle  = 0.0f32;
    let cosine = angle.cos();
    println!( "angle = {angle}, cosine = {cosine}");
}
```

โค้ดนี้คอมไพล์และทำงานได้โดยไม่มี error แม้ว่าฟังก์ชันจะรับ `self`

จุดที่ทำให้เกิดข้อสงสัยคือ

> “ถ้า `cos(self)` รับ `self` โดยค่าที่จะย้าย ownership แล้วทำไม `angle` ยังถูกใช้งานใน println! โดยไม่มี error?”

คำตอบของคำถามนี้และแนวคิดที่เกี่ยวข้องคือหัวใจของบทนี้

## การวิเคราะห์ความหมายของ `self`

ใน Rust ทุกฟังก์ชันหรือเมธอดมีชนิด (signature) ที่ชัดเจนว่าพารามิเตอร์ใด _take ownership_, _borrow immutable_, หรือ _borrow mutable_ จากค่าที่ส่งเข้าไป

### ความแตกต่างระหว่าง `self`, `&self`, และ `&mut self`

Rust มีสามรูปแบบการรับค่า `self` ในเมธอด:

| Syntax | การรับค่า | ใช้งานตัวแปรเดิมต่อได้ไหม | เหมาะสำหรับ |
|--------|-----------|------------------------|-------------|
| `self` | Transfer ownership (Move/Copy) | ถ้าเป็น Copy: ได้ / ถ้าเป็น Move: ไม่ได้ | Consuming methods |
| `&self` | Immutable borrow | ได้ | Read-only methods |
| `&mut self` | Mutable borrow | ได้ (ด้วยเงื่อนไข) | Modifying methods |

### 1. เมื่อเมธอดประกาศรับ `self`

```rust
impl SomeType {
    fn consume(self) -> ReturnType { ... }
}
```

นี่หมายความว่าเมธอดจะ **transfer ownership ของค่าที่เรียกใช้เมธอดเข้าไปในฟังก์ชัน** — นั่นคือค่าเดิมจากจุดเรียกจะถูก _moved_ เข้าไปในเมธอด

**กรณีที่ชนิดข้อมูลไม่ implement `Copy`:**

```rust
struct NonCopy(String);

impl NonCopy {
    fn consume(self) {}
}

fn main() {
    let n = NonCopy(String::from("test"));
    n.consume();
    // n.consume(); // ERROR: use of moved value: `n`
    // println!("{:?}", n); // ERROR: ใช้ไม่ได้เพราะ n ถูก move ไปแล้ว
}
```

**กรณีที่ชนิดข้อมูล implement `Copy`:**

```rust
let x = 42i32;
let y = x; // x ถูก copy (ไม่ใช่ move)
println!("x = {}, y = {}", x, y); // ทำงานได้: x = 42, y = 42
```

## ทำไม `angle.cos()` ถึงไม่ทำให้ `angle` หายไป

ชนิดข้อมูลพื้นฐานของ Rust เช่น `f32`, `i32`, `bool`, และ tuple/array ของชนิดที่เป็น Copy ล้วน implement trait `Copy` โดยอัตโนมัติ

### หลักการทำงานของ `Copy` trait

เมื่อชนิดข้อมูล implement `Copy`:

- การส่งค่าทางพารามิเตอร์โดยตรง (`self`, หรือผ่านฟังก์ชัน) จะ **ไม่ย้าย ownership จริง** แต่จะทำการคัดลอกค่า (duplicate by copying bits)
- ดังนั้น `angle.cos()` จะทำให้ค่า `angle` ถูก _copy_ เข้าไปในเมทอดแทนที่จะถูกย้ายจริง
- ตัวแปรเดิมยังคงใช้งานได้ต่อเนื่องหลังจากเรียกเมธอด

> **หมายเหตุ:** `Copy` หมายถึง "duplicate by copying bits" ที่ compiler รู้จักและสามารถ optimize ได้ ไม่ใช่ "bitwise copy" แบบ raw memory copy เสมอไป สำหรับ primitive types อาจดูเหมือนกัน แต่สำหรับ struct ที่ implement `Copy` เอง อาจมี semantics เฉพาะได้

### ความสัมพันธ์กับ traits อื่น

```rust
// ถ้า implement Drop จะไม่สามารถเป็น Copy ได้
struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {}
}
// impl Copy for HasDrop {} // ERROR: the trait `Copy` may not be implemented for this type

// ถ้าไม่เป็น Copy แต่ต้องการ duplicate ต้องใช้ Clone
let s = String::from("hello");
let s2 = s.clone(); // explicit clone (expensive)
```

## ทำไมจึงใช้ `self` แทน `&self` ในกรณีนี้

เมื่อชนิดข้อมูลเป็นชนิดเล็ก เช่น `f32` หรือ `i32`:

- สำหรับ types ขนาดเล็ก ความต่างทางประสิทธิภาพระหว่าง `self` และ `&self` นั้นเล็กน้อยจนไม่มีนัยสำคัญ
- Compiler สามารถ optimize การ pass-by-value ให้ใช้ register ได้แทนการเข้าถึง memory
- การรับ `self` ทำให้ API อ่านง่ายกว่าและสอดคล้องกับนิยาม "ฟังก์ชันที่รับค่าทาง value โดยตรง"

ด้วยเหตุนี้ฟังก์ชันเช่น `f32::cos` มักประกาศด้วย `fn cos(self) -> f32` แทน `fn cos(&self) -> f32`

## การใช้ `self` กับชนิดที่ไม่ implement `Copy`

เมื่อชนิดข้อมูลไม่ implement trait `Copy` เช่น `String`, `Vec<T>`, หรือ struct ที่ประกอบด้วยชนิดเหล่านี้

- การประกาศเมธอดรับ `self` จะ **transfer ownership ของค่าลงในเมธอด**
- หลังจากเรียกเมธอดแล้ว ตัวแปรเดิมจะ **ไม่สามารถใช้งานได้อีก**

ตัวอย่างที่เห็นได้ชัดคือการใช้ naming convention `into_*` สำหรับ consuming methods:

```rust
struct MyStruct(String);

impl MyStruct {
    // naming convention: into_* บ่งบอกว่าเป็น consuming method
    fn into_inner(self) -> String {
        self.0
    }
}

fn main() {
    let s = MyStruct(String::from("hello"));
    let inner = s.into_inner(); // ownership ของ s ย้ายเข้าเมทอด
    // println!("{:?}", s); // ERROR: borrow of moved value: `s`
    println!("{}", inner); // ทำงานได้
}
```

## ⚠️ Copy Types กับ `&mut self` - ข้อควรระวัง

ประเด็นสำคัญที่มักเกิด silent bug เมื่อเรียก `&mut self` บน Copy type โดยไม่รู้ตัว Rust จะสร้าง temporary copy แล้ว mutate ค่านั้น แล้วทิ้งไปทันที

```rust
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

impl Point {
    fn translate(&mut self, dx: i32) {
        self.x += dx;
    }
}

fn main() {
    let p = Point { x: 0, y: 0 };
    p.translate(5); // ⚠️ สร้าง temporary copy แล้ว mutate แต่ทิ้งทันที
    println!("{:?}", p); // ค่าไม่เปลี่ยน! p.x ยังเป็น 0
}
```

**แนวทางที่ถูกต้อง** สำหรับ Copy types ควรใช้ consuming pattern แทน

```rust
impl Point {
    fn translated(self, dx: i32) -> Self { // คืนค่าใหม่แทน
        Point { x: self.x + dx, y: self.y }
    }
}

fn main() {
    let p = Point { x: 0, y: 0 };
    let p = p.translated(5); // ชัดเจนว่าสร้างค่าใหม่
    println!("{:?}", p); // Point { x: 5, y: 0 }
}
```

### Checklist เมื่อไหร่ที่ไม่ควรทำให้เป็น Copy

หลีกเลี่ยงการ implement `Copy` ถ้า type ของคุณ

1. **มี `&mut self` methods ที่เปลี่ยน internal state** (เช่น Iterator)
2. **มี invariant ที่ตรงต้องรักษา** และการ copy อาจทำให้ state ไขว้กัน
3. **มี Drop implementation** (ไม่สามารถเป็น Copy ได้อยู่แล้ว)

**ตัวอย่าง** `Range` ไม่เป็น Copy เพราะจะสร้าง confusion ถ้ามีหลาย iterators ที่ไขว้กัน

```rust
let mut range = 0..10;
let copy_of_range = range; // ถ้า Range เป็น Copy...

range.next(); // advance original
copy_of_range.next(); // advance copy → ผู้ใช้งงว่าทำไม range ไม่ advance
```

## 🛠️ เทคนิคการย้าย Ownership บางส่วน

เมื่อต้องการ take ownership แค่ field เดียว โดยไม่ consume ทั้ง struct ใช้ `Option::take()`

```rust
struct ConnectionManager {
    conn: Option<Connection>,
}

impl ConnectionManager {
    // ย้าย ownership ออกจาก field โดยไม่ consume ทั้ง struct
    fn disconnect(&mut self) -> Option<Connection> {
        self.conn.take() // ย้าย ownership ออกจาก Option
    }
    
    // หรือใช้ pattern นี้เพื่อ consume field แล้วคืนค่าที่เหลือ
    fn into_connection(self) -> Option<Connection> {
        self.conn // ย้าย ownership ของ field ออกมา
    } // self ถูก drop แต่ไม่มีผลเพราะเราเอา field ออกมาแล้ว
}
```

## Common Pitfalls

### Pitfall 1 สมมติว่าทุกอย่างเป็น Copy

```rust
let s = String::from("hello");
let s2 = s; // String ไม่มี Copy, s ถูก move
// println!("{}", s); // ERROR: borrow of moved value: `s`
```

### Pitfall 2 Generic ที่ลืมคิดเรื่อง Copy

```rust
fn generic_consume<T>(val: T) {
    // ถ้า T ไม่เป็น Copy, val จะถูก move
    // ใช้ val ตรงนี้...
} // ...และถูก drop ตรงนี้

fn main() {
    let s = String::from("test");
    generic_consume(s);
    // generic_consume(s); // ERROR ถ้า T ไม่เป็น Copy
}
```

### Pitfall 3 สับสนระหว่าง Copy และ Clone

```rust
#[derive(Clone)] // มี Clone แต่ไม่มี Copy
struct OnlyClone(i32);

let a = OnlyClone(42);
let b = a.clone(); // ต้องเรียก explicitly
// let c = a; // นี่คือ move ไม่ใช่ copy
```

### Pitfall 4 ลืมว่า Copy ก็ยังเป็น duplicate

```rust
#[derive(Copy, Clone)]
struct ID(u64);

fn process(id: ID) {
    // ถ้า ID มี side effect เมื่อถูกใช้ (เช่น logging)
    // การ copy จะทำให้เกิด duplicate side effect
}
```

## สรุป

การประกาศเมธอดใน Rust ว่า `fn foo(self)` มีความหมายดังนี้

- **โดยทั่วไป** หมายถึงเมธอดจะ **take ownership** ของค่าที่เรียกใช้ (move)
- **ถ้าชนิดข้อมูล implement `Copy`** การ transfer ownership จะถูกแปลงเป็น _copy_ แทน ทำให้ตัวแปรเดิมยังใช้งานได้
- **การเลือกใช้** `self`, `&self`, หรือ `&mut self` ขึ้นอยู่กับว่าฟังก์ชันต้องการ
  - บริโภคค่า (consuming) → `self`
  - อ่านค่าเท่านั้น → `&self`
  - แก้ไขค่า → `&mut self`

### Checklist ในการอ่าน Method Signature

```rust
impl MyType {
    fn method(self)     // ถ้า MyType: Copy → ใช้ต่อได้ / ถ้าไม่ → move
    fn method(&self)    // borrow แบบอ่านอย่างเดียว ใช้ต่อได้เสมอ
    fn method(&mut self) // borrow แบบแก้ไข ใช้ต่อได้แต่ต้องระวัง aliasing rules
}
```

### Naming Conventions สำหรับ Ownership

| Prefix | ความหมาย | ตัวอย่าง |
|--------|---------|---------|
| `into_*` | Consuming, take ownership | `into_inner()`, `into_string()` |
| `as_*` | Borrow/Reference conversion | `as_str()`, `as_slice()` |
| `to_*` | Clone/Copy conversion | `to_string()`, `to_vec()` |
| `moved_*` | Copy type consuming | `moved_x()`, `translated()` |

เมื่อเข้าใจความหมายนี้แล้ว เราสามารถอ่าน signature ของเมทอดใน Rust แล้วรู้ได้ทันทีว่าฟังก์ชันนั้นจะ transfer ownership หรือเพียงแค่ borrow ค่าเดิม

## แหล่งอ้างอิง

- [Rust Forum: Does a member function take ownership of a self argument?](https://users.rust-lang.org/t/does-a-member-function-take-ownership-of-a-self-argument/138034)
- [The Rust Programming Language - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust Reference: Copy Trait](https://doc.rust-lang.org/reference/items/traits.html#copy-and-clone)


