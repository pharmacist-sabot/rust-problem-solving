## OB 001: การเป็นเจ้าของของ self ในฟังก์ชันเมมเบอร์ (Self Ownership)

ในหัวข้อนี้ เราจะมาเจาะลึกคำถามพื้นฐานที่สำคัญมากสำหรับการออกแบบ API ใน Rust นั่นคือ

> **"ถ้าเมธอดรับ `self` แปลว่าฟังก์ชันจะ 'ยึดครอง' (Take Ownership) ค่าที่เราส่งเข้าไปจนเราใช้ต่อไม่ได้... จริงหรือ?"**

คำตอบของเรื่องนี้ไม่ได้มีแค่ "ใช่" หรือ "ไม่ใช่" แต่ขึ้นอยู่กับคุณสมบัติของ Type นั้นๆ และเหตุผลเบื้องหลังเรื่องประสิทธิภาพ (Performance) ที่ Rust ออกแบบมาอย่างตั้งใจ

## จุดเริ่มต้นของความสงสัย

ลองดูตัวอย่างคลาสสิกจาก Standard Library ของ Rust คือฟังก์ชัน `cos` สำหรับคำนวณค่า cosine

```rust
// Signature ใน std library
pub fn cos(self) -> f32
```

สังเกตว่ามันรับ `self` (ไม่ใช่ `&self`) ซึ่งตามกฎ Ownership แล้ว การรับแบบนี้ควรจะเป็นการ **Move** หรือย้ายสิทธิ์ความเป็นเจ้าของเข้าไปในฟังก์ชัน

แต่เมื่อเราเขียนโค้ดจริง

```rust
fn main() {
    let angle = 0.0f32;
    let cosine = angle.cos(); // เรียกเมธอดที่รับ self
    
    // เอ๊ะ! ทำไมยังเอา angle มาปริ้นต์ต่อได้? ไม่โดน Move ไปแล้วเหรอ?
    println!("angle = {angle}, cosine = {cosine}");
}
```

โค้ดนี้กลับคอมไพล์ผ่านและทำงานได้ปกติ นี่จึงเป็นที่มาของความสับสนว่าตกลง `self` ทำงานอย่างไรกันแน่

## กฎพื้นฐาน 3 ร่างของ `self`

ก่อนอื่นต้องแม่นยำเรื่อง Syntax ของ `self` ใน Rust เมธอดรับ `self` ได้ 3 แบบหลักๆ

### 1. `self` (Take Ownership)

- **ความหมาย**: ยึดครองความเป็นเจ้าของ (Take Ownership)
- **ผลลัพธ์ต่อตัวแปรเดิม**: ตัวแปรเดิมจะถูก **Move** (ย้ายสิทธิ์) หรือ **Copy** (ทำสำเนา) ไป
- **สถานการณ์ที่ใช้**: เมื่อต้องการ "กิน" ค่าเข้าไป (Consuming) หรือคำนวณแล้วคืนค่าใหม่ (Transformation)

### 2. `&self` (Shared Borrow)

- **ความหมาย**: ยืมอ่าน (Shared Borrow)
- **ผลลัพธ์ต่อตัวแปรเดิม**: ตัวแปรเดิมยังคงอยู่ และสามารถถูกอ่านจากที่อื่นพร้อมกันได้
- **สถานการณ์ที่ใช้**: เมื่อต้องการแค่อ่านค่าเพื่อคำนวณ แต่ไม่ต้องการแก้ไข

### 3. `&mut self` (Mutable Borrow)

- **ความหมาย**: ยืมไปแก้ (Mutable Borrow)
- **ผลลัพธ์ต่อตัวแปรเดิม**: ตัวแปรเดิมยังคงอยู่ แต่จะถูกล็อก (Lock) ห้ามใครใช้อ่านหรือเขียนแทรกชั่วคราว
- **สถานการณ์ที่ใช้**: เมื่อต้องการแก้ไขค่าภายใน (Update state)

## ทำไม `angle.cos()` ถึงไม่กิน `angle` หายไป?

คำตอบอยู่ที่ **Trait `Copy`** ครับ

ใน Rust ชนิดข้อมูลพื้นฐาน (Primitives) ที่มีขนาดเล็ก เช่น `f32`, `i32`, `bool`, `char` จะถูก implement trait ที่ชื่อว่า `Copy` โดยอัตโนมัติ

### ความลับของ `Copy` Trait (Semantics vs Mechanics)

นี่คือจุดที่น่าสนใจจากมุมมองเชิงลึก

1. **ในทางความหมาย (Semantics)** การประกาศ `fn(self)` คือการประกาศเจตนาว่า **"ฉันต้องการครอบครองค่านี้" (Move semantics)** เสมอ
2. **ในทางกลไก (Mechanics)** แต่ถ้า Type นั้นมี `Copy` trait แปะอยู่ คอมไพเลอร์จะเปลี่ยนพฤติกรรมจากการ "ย้ายความเป็นเจ้าของ" (ทำให้ตัวเดิมใช้ไม่ได้) เป็นการ **"ทำสำเนาบิต" (Bitwise Copy)** เข้าไปแทน

ดังนั้น `angle.cos()` จึงแค่ "สำเนา" ค่าของ `angle` เข้าไปคำนวณ ไม่ได้ยึดตัวแปรต้นทางไป ตัวแปรเดิมจึงยังใช้งานได้ครับ

### 💡 Deep Dive: ทำไมใช้ `self` ถึงดีกว่า `&self`?

ทำไมฟังก์ชันคณิตศาสตร์อย่าง `cos`, `sin`, `abs` ถึงเลือกใช้ `self`? ทำไมไม่ใช้ `&self` เพื่อความชัดเจน? คำตอบคือ **Performance** ครับ

- **`self` (Pass-by-value)** สำหรับข้อมูลขนาดเล็ก (เช่น `f32` = 4 bytes) CPU สามารถโหลดค่านี้เข้าไปใน **Register** ได้โดยตรงและคำนวณได้ทันที นี่คือวิธีที่เร็วที่สุด
- **`&self` (Pass-by-reference)** คอมพิวเตอร์ต้องส่ง **Pointer** (ซึ่งขนาด 8 bytes บนเครื่อง 64-bit) ไปที่ฟังก์ชัน จากนั้นฟังก์ชันต้องเสียเวลาวิ่งกลับไปอ่านค่าที่ Memory ปลายทางอีกที (Dereference)

การใช้ `self` กับ Primitive types จึงทั้งเร็วกว่าและประหยัดหน่วยความจำกว่าครับ

## แล้วถ้า Type **ไม่เป็น** `Copy` ล่ะ?

สำหรับ Type ที่ซับซ้อน เช่น `String`, `Vec<T>` หรือ Struct ทั่วไป Rust จะ **ไม่** ให้เป็น `Copy` โดยอัตโนมัติ

ถ้าเราประกาศเมธอดรับ `self` กับ Type เหล่านี้ จะเกิดการ **Move** ทันทีครับ

```rust
struct MyData(String);

impl MyData {
    fn consume_me(self) { // รับ self และ Type นี้ไม่ใช่ Copy
        println!("Inside: {}", self.0);
    } // self ถูก Drop (ทำลาย) ทิ้งเมื่อจบฟังก์ชันนี้
}

fn main() {
    let d = MyData(String::from("Hello"));
    d.consume_me(); // Ownership ถูกย้ายเข้าไป
    // println!("{:?}", d); // ❌ ERROR: value used here after move
}
```

## ⚠️ ระวังหลุมพราง "Silent Bug" ของ Copy Types

ข้อนี้สำคัญมาก และมักเป็นจุดตายของมือใหม่ คือการใช้ `mut` กับ `self` ใน Type ที่เป็น `Copy`

### สิ่งที่มักเข้าใจผิด

เราอาจเผลอเขียนเมธอดที่รับค่าเข้ามาแก้ไข (Mutate) แต่ดันรับมาแบบ `self` (Pass by value)

```rust
#[derive(Clone, Copy, Debug)]
struct Point { x: i32, y: i32 }

impl Point {
    // ⚠️ ระวัง! รับ mut self (Value) ไม่ใช่ &mut self (Reference)
    fn move_wrong(mut self, dx: i32) {
        self.x += dx; 
        // สิ่งที่ถูกแก้คือ "ตัวสำเนา" (Copy) ที่อยู่ในฟังก์ชันนี้เท่านั้น
    } // ตัวสำเนาถูกทำลายทิ้งตรงนี้
}

fn main() {
    let p = Point { x: 0, y: 0 };
    p.move_wrong(10); 
    println!("{:?}", p); // 😱 ผลลัพธ์: Point { x: 0, y: 0 } ค่าไม่เปลี่ยน
}
```

### วิธีแก้ไข

1. **ถ้าจะแก้ค่าเดิม** ใช้ `&mut self` เสมอ
2. **ถ้าจะคืนค่าใหม่ (Functional style)** รับ `self` แล้วคืน `Self` กลับไป

```rust
impl Point {
    // ✅ แบบคืนค่าใหม่
    fn moved(self, dx: i32) -> Self {
        Point { x: self.x + dx, y: self.y }
    }
}
```

## เมื่อไหร่ที่ไม่ควรทำเป็น Copy? (กรณีศึกษา Range)

บางครั้งข้อมูลมีขนาดเล็ก แต่เราก็ไม่ควรให้มันเป็น `Copy` ตัวอย่างที่ดีที่สุดคือ **`Range` (เช่น `0..10`)**

ถึงแม้ Range จะเก็บแค่ตัวเลข `start` กับ `end` แต่ Rust ไม่ให้มันเป็น Copy เพราะมันทำหน้าที่เป็น **Iterator** ด้วย

```rust
// สมมติว่า Range เป็น Copy...
let mut r = 0..5;
for _ in r { ... } // r ถูก copy ไปใช้ใน loop จนหมด
// r ตัวเดิมยังอยู่ที่ 0..5 เหมือนเดิม เพราะไม่ได้ถูก Move ไป
for _ in r { ... } // ลูปทำงานซ้ำอีกรอบ!
```

พฤติกรรมนี้จะสร้างบั๊กที่ตามหาได้ยาก Rust จึงบังคับให้ Iterator ต้องถูก **Move** เสมอ เพื่อให้แน่ใจว่าสถานะการวนลูป (Current state) มีอยู่แค่ที่เดียว

## สรุป Checklist ในการออกแบบ

เมื่อคุณเห็นหรือเขียน Method Signature ให้ตีความดังนี้

1. **`fn method(self)`**
    - **ถ้าเป็น Copy Type** แค่ทำสำเนาค่าไปใช้ (เช่น `angle.cos()`)
    - **ถ้าไม่ใช่ Copy Type** ยึดครอง (Move) ต้นฉบับไปเลย (เช่น `vec.into_iter()`)
    - *เหมาะสำหรับ* การแปลงค่า (`into_...`) หรือการคำนวณที่คืนค่าใหม่

2. **`fn method(&self)`**
    - ขอยืมดูเฉยๆ ไม่ว่าจะเป็น Type อะไร
    - *เหมาะสำหรับ* การอ่านค่า, Getter (`.len()`, `.is_empty()`)

3. **`fn method(&mut self)`**
    - ขอยืมไปแก้ไข (Mutate)
    - *เหมาะสำหรับ* การเปลี่ยน state ภายใน (`.push()`, `.clear()`)

### Naming Convention Tips

- `into_*` (เช่น `into_string`) → กินค่า (`self`)
- `to_*` (เช่น `to_string`) → ยืมแล้วก๊อปปี้ (`&self`)
- `as_*` (เช่น `as_bytes`) → ยืมแล้วแปลงมุมมอง (`&self`)

### แหล่งอ้างอิง

- [Rust Forum: Does a member function take ownership of a self argument?](https://users.rust-lang.org/t/does-a-member-function-take-ownership-of-a-self-argument/138034)
- [The Rust Programming Language - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust Reference: Copy Trait](https://doc.rust-lang.org/reference/items/traits.html#copy-and-clone)
