# Troubleshooting Index — ดัชนีแก้ปัญหา

รวมปัญหาและ Error ที่พบบ่อยในการเขียน Rust จัดเรียงตามหมวดหมู่เพื่อให้ค้นหาได้ง่าย พร้อมลิงก์ไปยังบทเรียนที่อธิบายรายละเอียดเพิ่มเติม

---

## Ownership & Borrowing Errors

### E0382: Use of moved value

**อาการ:** ใช้ตัวแปรหลังจากที่มันถูก Move ไปแล้ว

```rust
let s = String::from("hello");
let t = s;          // s ถูก Move ไปที่ t
println!("{}", s);  // ❌ E0382: value used here after move
```

**สาเหตุ:** ตัวแปรที่ไม่ได้ implement `Copy` trait จะถูกย้ายความเป็นเจ้าของ (Move) เมื่อ assign ให้ตัวแปรอื่นหรือส่งเข้าฟังก์ชัน

**วิธีแก้:**

- ใช้ `.clone()` เพื่อสร้างสำเนา
- ใช้ Reference (`&s`) แทนการส่งค่าตรงๆ
- ปรับ API ให้รับ `&self` แทน `self`

📖 **อ่านเพิ่มเติม:** [การเป็นเจ้าของของ `self`](../part-01-ownership/self-ownership.md)

---

### E0502: Cannot borrow as mutable because it is also borrowed as immutable

**อาการ:** พยายาม Mutable Borrow ขณะที่ยังมี Immutable Borrow ค้างอยู่

```rust
let mut input = String::new();
let trimmed = input.trim();         // Immutable borrow
std::io::stdin().read_line(&mut input).unwrap();  // ❌ E0502
println!("{}", trimmed);
```

**สาเหตุ:** Rust ไม่อนุญาตให้มี `&mut` และ `&` ของตัวแปรเดียวกันอยู่พร้อมกัน เพื่อป้องกัน Data Race

**วิธีแก้:**

- ใช้ `.to_string()` หรือ `.to_owned()` เพื่อสร้าง Owned Value แทนการเก็บ Reference
- จัดลำดับ Scope ให้ Immutable Borrow จบก่อนที่จะเริ่ม Mutable Borrow

📖 **อ่านเพิ่มเติม:** [กับดักของ `read_line` ในลูป — Borrow Checker Error](../part-01-ownership/read-line-traps.md)

---

### E0499: Cannot borrow as mutable more than once at a time

**อาการ:** พยายาม Mutable Borrow ตัวแปรเดียวกัน 2 ครั้งพร้อมกัน

```rust
let mut v = vec![1, 2, 3];
let first = &mut v[0];
let second = &mut v[1];  // ❌ E0499
```

**สาเหตุ:** กฎ Aliasing XOR Mutation — Mutable Reference ต้องมีได้เพียงตัวเดียวในขณะใดขณะหนึ่ง

**วิธีแก้:**

- แยก Scope ของแต่ละ Mutable Borrow
- ใช้ `split_at_mut()` สำหรับ Slice
- พิจารณาใช้ `Cell` หรือ `RefCell` สำหรับ Interior Mutability

---

## Lifetime Errors

### Lifetime may not live long enough

**อาการ:** พยายาม Return Reference ที่มี Lifetime สั้นกว่าที่ Signature กำหนด

```rust
fn to_other_ctx(&mut self) -> OtherCtx<'s> {
    OtherCtx { window_state: self.window_state }
    // ❌ lifetime may not live long enough
}
```

**สาเหตุ:** การเข้าถึง Field ผ่าน `&mut self` เป็น Reborrow ที่มี Lifetime สั้นกว่า Lifetime ดั้งเดิมของ Field

**วิธีแก้:**

- ใช้ Anonymous Lifetime `'_` ใน Return Type: `-> OtherCtx<'_>`
- หรือระบุ Lifetime แบบ Explicit ที่ผูกกับ `&mut self`

📖 **อ่านเพิ่มเติม:** [Reborrowing และการแปลงโครงสร้างข้อมูล](../part-01-ownership/reborrowing-transform.md)

---

### E0106: Missing lifetime specifier

**อาการ:** ลืมระบุ Lifetime ใน Struct หรือ Function Signature ที่มี Reference

```rust
struct Foo {
    data: &str,  // ❌ E0106: missing lifetime specifier
}
```

**วิธีแก้:**

- เพิ่ม Lifetime Parameter: `struct Foo<'a> { data: &'a str }`
- หากเป็นกรณีง่ายใน Function ให้อาศัย Lifetime Elision Rules

---

## Common Runtime Issues

### Silent Bug: Buffer ไม่ถูก Clear ในลูป

**อาการ:** ข้อมูลเก่าสะสมใน String buffer ทำให้เงื่อนไขในลูปไม่ทำงานตามที่คาดหวัง โปรแกรมไม่พังแต่ทำงานผิด

```rust
let mut input = String::new();
loop {
    // ❌ ลืม input.clear()
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "exit" { break; } // ไม่มีวันเป็น true หลังรอบแรก
}
```

**สาเหตุ:** `read_line()` ออกแบบมาให้ **Append** ข้อมูลต่อท้ายเสมอ ไม่ใช่ Overwrite

**วิธีแก้:**

- เรียก `.clear()` ก่อน `read_line()` ทุกรอบ
- หรือประกาศ `String::new()` ภายในลูป
- หรือใช้ `stdin().lock().lines()` แทน

📖 **อ่านเพิ่มเติม:** [กับดักของ `read_line` ในลูป — บั๊กเงียบ](../part-01-ownership/read-line-traps.md)

---

### Silent Bug: `mut self` กับ Copy Types

**อาการ:** เรียกเมธอดที่รับ `mut self` บน Copy Type แล้วค่าไม่เปลี่ยน

```rust
#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

impl Point {
    fn move_wrong(mut self, dx: i32) {
        self.x += dx;  // แก้ไขแค่สำเนา!
    }
}
```

**สาเหตุ:** สำหรับ Copy Type การรับ `self` (by value) จะทำสำเนาเข้ามา การแก้ไขจึงเกิดขึ้นกับสำเนาเท่านั้น

**วิธีแก้:**

- ใช้ `&mut self` หากต้องการแก้ไขค่าเดิม
- หรือ Return `Self` กลับไป (Functional style): `fn moved(self, dx: i32) -> Self`

📖 **อ่านเพิ่มเติม:** [การเป็นเจ้าของของ `self` — หลุมพราง Silent Bug](../part-01-ownership/self-ownership.md)

---

## Quick Lookup by Error Code

| Error Code | ชื่อ Error | หมวดหมู่ | ลิงก์ |
|------------|-----------|----------|------|
| E0106 | Missing lifetime specifier | Lifetime | [ด้านบน](#e0106-missing-lifetime-specifier) |
| E0382 | Use of moved value | Ownership | [ด้านบน](#e0382-use-of-moved-value) |
| E0499 | Cannot borrow `&mut` more than once | Borrowing | [ด้านบน](#e0499-cannot-borrow-as-mutable-more-than-once-at-a-time) |
| E0502 | Cannot borrow `&mut` while `&` exists | Borrowing | [ด้านบน](#e0502-cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable) |

---

## แหล่งข้อมูลเพิ่มเติม

- [Rust Compiler Error Index](https://doc.rust-lang.org/error_codes/error-index.html) — รวม Error Code ทั้งหมดจาก Rust Compiler
- [Rust Reference: Borrow Checker](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions) — เอกสารอ้างอิงกฎการยืม
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md) — บทความยอดนิยมเรื่อง Lifetime
