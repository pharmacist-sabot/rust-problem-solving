## OB 001: การเป็นเจ้าของของ self ในฟังก์ชันเมมเบอร์

ในหัวข้อนี้ เราจะมาหาคำตอบของคำถามพื้นฐานเกี่ยวกับการประกาศเมธอด (method) ที่รับ `self` ว่า

> "ถ้าเมธอดรับ `self` แปลว่าฟังก์ชันจะ **ยึดครอง (take ownership)** ค่าที่เราส่งเข้าไปจริงๆ หรือเปล่า?"

คำถามนี้สำคัญมากสำหรับการออกแบบ API และการจัดการทรัพยากรใน **Rust** เพราะถ้าฟังก์ชันยึดครองค่า แสดงว่าค่านั้นจะถูกย้าย (moved) เข้าไปในฟังก์ชัน และเราก็จะไม่สามารถเอามาใช้ต่อได้อีก

## ตัวอย่างที่ทำให้เกิดข้อสงสัย

ลองดูโค้ดฟังก์ชันจากมาตรฐานไลบรารี (Crate std) กันก่อน

```rust
pub fn cos(self) -> f32
```

เมื่อเราเรียกใช้เมธอดนี้

```rust
fn main() {
    let angle  = 0.0f32;
    let cosine = angle.cos();
    println!( "angle = {angle}, cosine = {cosine}");
}
```

โค้ดนี้คอมไพล์ผ่านและทำงานได้ปกติ แม้ว่าฟังก์ชันจะรับ `self` ก็ตาม

ประเด็นที่น่าสงสัยคือ

> "ถ้า `cos(self)` รับ `self` โดยตรง ซึ่งควรจะย้าย ownership แล้วทำไม `angle` ถึงยังใช้งานใน println! ได้อยู่ โดยไม่เกิด error เลย?"

คำตอบของคำถามนี้และแนวคิดที่เกี่ยวข้องคือหัวใจสำคัญของบทความนี้

## ทำความเข้าใจความหมายของ `self`

ใน Rust ทุกฟังก์ชันหรือเมธอดจะมี signature ที่บอกชัดเจนว่าพารามิเตอร์แต่ละตัว _ยึดครอง_, _ยืมแบบอ่านอย่างเดียว_, หรือ _ยืมแบบแก้ไขได้_ จากค่าที่ส่งเข้าไป

### ความแตกต่างระหว่าง `self`, `&self`, และ `&mut self`

Rust มีสามรูปแบบในการรับค่า `self` ในเมธอด:

| Syntax | วิธีรับค่า | ใช้ตัวแปรเดิมต่อได้ไหม | เหมาะสำหรับ |
|--------|-----------|------------------------|-------------|
| `self` | ย้าย ownership (Move/Copy) | ถ้าเป็น Copy: ได้ / ถ้าเป็น Move: ไม่ได้ | เมธอดที่กินค่าทิ้ง (consuming) |
| `&self` | ยืมแบบอ่านอย่างเดียว | ได้ | เมธอดที่แค่อ่านค่า |
| `&mut self` | ยืมแบบแก้ไขได้ | ได้ (ตามเงื่อนไขของ Rust) | เมธอดที่แก้ไขค่า |

### 1. เมื่อเมธอดรับ `self`

```rust
impl SomeType {
    fn consume(self) -> ReturnType { ... }
}
```

นี่แปลว่าเมธอดจะ **ยึดครองค่าที่เราเรียกเมธอดนั้น** — หรือก็คือค่าเดิมจะถูก _moved_ เข้าไปในเมธอด

**กรณีที่ชนิดข้อมูลไม่ได้ implement `Copy`:**

```rust
struct NonCopy(String);

impl NonCopy {
    fn consume(self) {}
}

fn main() {
    let n = NonCopy(String::from("test"));
    n.consume();
    // n.consume(); // ERROR: use of moved value: `n`
    // println!("{:?}", n); // ERROR: ใช้ไม่ได้แล้วเพราะ n ถูก move ไปแล้ว
}
```

**กรณีที่ชนิดข้อมูล implement `Copy`:**

```rust
let x = 42i32;
let y = x; // x ถูก copy (ไม่ใช่ move)
println!("x = {}, y = {}", x, y); // ใช้ได้ทั้งคู่: x = 42, y = 42
```

## ทำไม `angle.cos()` ถึงไม่ทำให้ `angle` หายไป

ชนิดข้อมูลพื้นฐานของ Rust เช่น `f32`, `i32`, `bool`, และ tuple/array ของชนิดที่เป็น Copy ล้วน implement trait `Copy` โดยอัตโนมัติ

### หลักการทำงานของ `Copy` trait

เมื่อชนิดข้อมูล implement `Copy`:

- การส่งค่าเข้าพารามิเตอร์โดยตรง (`self` หรือผ่านฟังก์ชัน) จะ **ไม่ได้ย้าย ownership จริงๆ** แต่จะทำการ copy ค่า (ทำสำเนา) แทน
- ดังนั้น `angle.cos()` จะทำให้ค่า `angle` ถูก _copy_ เข้าไปในเมธอด ไม่ใช่ย้ายแบบ move
- ตัวแปรเดิมก็ยังใช้งานได้ต่อหลังจากเรียกเมธอด

> **หมายเหตุ:** `Copy` หมายถึง "ทำสำเนาโดยการคัดลอก bits" ที่ compiler รู้จักและสามารถ optimize ได้ ไม่ใช่แค่ "bitwise copy" แบบ raw memory copy ธรรมดา สำหรับ primitive types อาจดูคล้ายกัน แต่สำหรับ struct ที่ implement `Copy` เอง อาจมี semantics เฉพาะของมันได้

### ความสัมพันธ์กับ traits อื่นๆ

```rust
// ถ้า implement Drop จะเป็น Copy ไม่ได้
struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {}
}
// impl Copy for HasDrop {} // ERROR: the trait `Copy` may not be implemented for this type

// ถ้าไม่เป็น Copy แต่อยากได้สำเนา ต้องใช้ Clone
let s = String::from("hello");
let s2 = s.clone(); // clone แบบชัดเจน (อาจจะช้า)
```

## ทำไมถึงใช้ `self` แทนที่จะเป็น `&self`

เมื่อชนิดข้อมูลมีขนาดเล็ก เช่น `f32` หรือ `i32`:

- สำหรับ types ขนาดเล็ก ความแตกต่างด้านประสิทธิภาพระหว่าง `self` กับ `&self` นั้นน้อยมากจนไม่สำคัญ
- Compiler สามารถ optimize การ pass-by-value ให้ใช้ register ได้แทนที่จะต้องไปเข้าถึง memory
- การรับ `self` ทำให้ API อ่านง่ายกว่า และสอดคล้องกับแนวคิด "ฟังก์ชันที่รับค่าทาง value โดยตรง"

ด้วยเหตุนี้ ฟังก์ชันอย่าง `f32::cos` จึงมักประกาศด้วย `fn cos(self) -> f32` แทนที่จะเป็น `fn cos(&self) -> f32`

## การใช้ `self` กับชนิดที่ไม่ได้ implement `Copy`

เมื่อชนิดข้อมูลไม่ได้ implement trait `Copy` เช่น `String`, `Vec<T>`, หรือ struct ที่มีชนิดเหล่านี้อยู่

- การประกาศเมธอดรับ `self` จะ **ยึดครองค่าลงในเมธอด**
- หลังจากเรียกเมธอดแล้ว ตัวแปรเดิมจะ **ใช้งานไม่ได้อีก**

ตัวอย่างที่เห็นได้ชัดคือการใช้ naming convention `into_*` สำหรับ consuming methods:

```rust
struct MyStruct(String);

impl MyStruct {
    // naming convention: into_* บอกว่าเป็น consuming method
    fn into_inner(self) -> String {
        self.0
    }
}

fn main() {
    let s = MyStruct(String::from("hello"));
    let inner = s.into_inner(); // ownership ของ s ถูกย้ายเข้าเมธอด
    // println!("{:?}", s); // ERROR: borrow of moved value: `s`
    println!("{}", inner); // ใช้ได้
}
```

## ⚠️ Copy Types กับ `&mut self` - ข้อควรระวัง

นี่เป็นประเด็นสำคัญที่มักทำให้เกิด silent bug เมื่อเรียก `&mut self` บน Copy type โดยไม่รู้ตัว Rust จะสร้าง temporary copy แล้ว mutate ค่านั้น แล้วทิ้งไปทันที

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

**วิธีที่ถูกต้อง** สำหรับ Copy types ควรใช้ consuming pattern แทน

```rust
impl Point {
    fn translated(self, dx: i32) -> Self { // คืนค่าใหม่
        Point { x: self.x + dx, y: self.y }
    }
}

fn main() {
    let p = Point { x: 0, y: 0 };
    let p = p.translated(5); // ชัดเจนว่ากำลังสร้างค่าใหม่
    println!("{:?}", p); // Point { x: 5, y: 0 }
}
```

### Checklist เมื่อไหร่ที่ไม่ควรทำให้เป็น Copy

หลีกเลี่ยงการ implement `Copy` ถ้า type ของคุณ

1. **มี `&mut self` methods ที่เปลี่ยน internal state** (เช่น Iterator)
2. **มี invariant ที่ต้องรักษา** และการ copy อาจทำให้ state ไขว้เขวได้
3. **มี Drop implementation** (ยังไงก็เป็น Copy ไม่ได้อยู่แล้ว)

**ตัวอย่าง** `Range` ไม่เป็น Copy เพราะจะสร้างความสับสนถ้ามีหลาย iterators ที่ไขว้กัน

```rust
let mut range = 0..10;
let copy_of_range = range; // ถ้า Range เป็น Copy...

range.next(); // advance original
copy_of_range.next(); // advance copy → คนใช้งงว่าทำไม range ไม่ advance
```

## 🛠️ เทคนิคการย้าย Ownership บางส่วน

เมื่อต้องการยึดครองแค่ field เดียว โดยไม่กินทั้ง struct ใช้ `Option::take()`

```rust
struct ConnectionManager {
    conn: Option<Connection>,
}

impl ConnectionManager {
    // ย้าย ownership ออกจาก field โดยไม่กินทั้ง struct
    fn disconnect(&mut self) -> Option<Connection> {
        self.conn.take() // ย้าย ownership ออกจาก Option
    }
    
    // หรือใช้ pattern นี้เพื่อกิน field แล้วคืนค่าที่เหลือ
    fn into_connection(self) -> Option<Connection> {
        self.conn // ย้าย ownership ของ field ออกมา
    } // self ถูก drop แต่ไม่มีปัญหาเพราะเราเอา field ออกมาแล้ว
}
```

## ข้อผิดพลาดที่มักเจอ

### ข้อผิดพลาดที่ 1: คิดว่าทุกอย่างเป็น Copy

```rust
let s = String::from("hello");
let s2 = s; // String ไม่มี Copy, s ถูก move
// println!("{}", s); // ERROR: borrow of moved value: `s`
```

### ข้อผิดพลาดที่ 2: Generic ที่ลืมคิดเรื่อง Copy

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

### ข้อผิดพลาดที่ 3: สับสนระหว่าง Copy กับ Clone

```rust
#[derive(Clone)] // มี Clone แต่ไม่มี Copy
struct OnlyClone(i32);

let a = OnlyClone(42);
let b = a.clone(); // ต้องเรียกแบบชัดเจน
// let c = a; // นี่คือ move ไม่ใช่ copy
```

### ข้อผิดพลาดที่ 4: ลืมว่า Copy ก็ยังเป็นการทำสำเนา

```rust
#[derive(Copy, Clone)]
struct ID(u64);

fn process(id: ID) {
    // ถ้า ID มี side effect เมื่อถูกใช้ (เช่น logging)
    // การ copy จะทำให้เกิด side effect ซ้ำซ้อน
}
```

## สรุป

การประกาศเมธอดใน Rust ว่า `fn foo(self)` มีความหมายดังนี้

- **โดยปกติ** แปลว่าเมธอดจะ **ยึดครอง** ค่าที่เรียกใช้ (move)
- **ถ้าชนิดข้อมูล implement `Copy`** การย้าย ownership จะถูกแปลงเป็น _copy_ แทน ทำให้ตัวแปรเดิมยังใช้งานได้
- **การเลือกใช้** `self`, `&self`, หรือ `&mut self` ขึ้นอยู่กับว่าฟังก์ชันต้องการ
  - กินค่าทิ้ง (consuming) → `self`
  - อ่านค่าอย่างเดียว → `&self`
  - แก้ไขค่า → `&mut self`

### Checklist ในการอ่าน Method Signature

```rust
impl MyType {
    fn method(self)     // ถ้า MyType: Copy → ใช้ต่อได้ / ถ้าไม่ → move
    fn method(&self)    // ยืมแบบอ่านอย่างเดียว ใช้ต่อได้เสมอ
    fn method(&mut self) // ยืมแบบแก้ไข ใช้ต่อได้แต่ต้องระวัง aliasing rules
}
```

### Naming Conventions สำหรับ Ownership

| Prefix | ความหมาย | ตัวอย่าง |
|--------|---------|---------|
| `into_*` | กินค่า, ยึดครอง | `into_inner()`, `into_string()` |
| `as_*` | แปลงเป็น Borrow/Reference | `as_str()`, `as_slice()` |
| `to_*` | Clone/Copy แล้วแปลง | `to_string()`, `to_vec()` |
| `moved_*` | Copy type แบบกินค่า | `moved_x()`, `translated()` |

เมื่อเข้าใจความหมายนี้แล้ว เราสามารถอ่าน signature ของเมธอดใน Rust แล้วรู้ได้ทันทีว่าฟังก์ชันนั้นจะยึดครอง หรือแค่ยืมค่าเดิม

## แหล่งอ้างอิง

- [Rust Forum: Does a member function take ownership of a self argument?](https://users.rust-lang.org/t/does-a-member-function-take-ownership-of-a-self-argument/138034)
- [The Rust Programming Language - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust Reference: Copy Trait](https://doc.rust-lang.org/reference/items/traits.html#copy-and-clone)


