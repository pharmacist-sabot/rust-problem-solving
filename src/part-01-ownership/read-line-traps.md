## OB 003: กับดักของ `read_line` ในลูป (The Infinite Append & Borrowing)

การรับ Input จากผู้ใช้ผ่าน Console เป็นเรื่องพื้นฐาน แต่ใน Rust การใช้ `read_line` ภายในลูป (`loop`, `while`) มีหลุมพรางซ่อนอยู่ 2 ชั้น ได้แก่ **"บั๊กเงียบที่โปรแกรมไม่พังแต่ทำงานผิด"** และ **"Error จาก Borrow Checker"**

บทความนี้จะพาไปดูสาเหตุ เบื้องหลังการออกแบบ และวิธีจัดการให้ถูกต้องครับ

### ทำไม `read_line` ถึงออกแบบให้รับ Buffer เข้ามา?

ก่อนจะเข้าเรื่องกับดัก เรามาทำความเข้าใจก่อนว่า ทำไม `read_line` ถึงไม่ได้ออกแบบให้ **return ค่า** กลับมาตรงๆ แบบนี้

```rust
// ❌ แบบนี้ไม่มีจริง แค่สมมติ
let input = io::stdin().read_line().unwrap();
```

แต่กลับออกแบบให้ **รับ Buffer** เข้าไปแก้ไขแทน:

```rust
// ✅ แบบที่ Rust ใช้จริง
let mut buf = String::new();
io::stdin().read_line(&mut buf).unwrap();
```

เหตุผลหลักคือ **Performance** — เมื่อเราเป็นผู้ถือ Buffer เอง เราสามารถ **นำ Buffer กลับมาใช้ซ้ำ** ได้ในรอบถัดไป โดยไม่ต้อง Allocate หน่วยความจำใหม่ทุกครั้ง ซึ่งในงานที่ต้องอ่านข้อมูลจำนวนมาก (เช่น อ่านไฟล์หลายล้านบรรทัด) ความต่างนี้มีผลอย่างมาก

นี่คือ Pattern ดั้งเดิมที่ใช้กันมาตั้งแต่ C โดยมีข้อดี 3 ประการ:

- **ลด Allocation** — ใช้ Buffer เดิมซ้ำได้ ไม่ต้องจองหน่วยความจำใหม่ทุกรอบ
- **แยกความรับผิดชอบ** — ตัว I/O ทำหน้าที่แค่อ่าน ส่วนการจัดการ Memory เป็นเรื่องของผู้เรียก
- **ยืดหยุ่นเรื่อง Lifetime** — Buffer และ Stream มี Lifetime เป็นอิสระต่อกัน

แต่การออกแบบนี้ก็แลกมาด้วย "กับดัก" ที่เราจะพูดถึงต่อไป

---

### 1. กับดักที่ 1: "บั๊กเงียบ" (The Phantom Input)

ลองดูโค้ดตัวอย่างที่มือใหม่ (และมือเก๋าที่เผลอ) มักเขียน:

```rust
use std::io;

fn main() {
    let mut input = String::new(); // สร้าง buffer นอกลูป

    loop {
        println!("Type something:");
        // อ่านค่าใส่ buffer ตัวเดิม
        io::stdin().read_line(&mut input).expect("Failed to read");

        println!("You typed: {:?}", input.trim()); // ใช้ {:?} เพื่อดูค่าจริงๆ

        if input.trim() == "exit" { break; }
    }
}
```

**ผลลัพธ์ที่ได้ (ความหายนะ):**

| รอบ | พิมพ์ | ค่าจริงใน buffer | `.trim()` ได้ | ตรวจ `== "exit"`? |
|------|---------|--------------------------|---------------------|---------------------|
| 1    | `A`     | `"A\n"`                  | `"A"`               | ❌                  |
| 2    | `B`     | `"A\nB\n"`               | `"A\nB"`            | ❌                  |
| 3    | `exit`  | `"A\nB\nexit\n"`         | `"A\nB\nexit"`      | ❌ ตลอดกาล!       |

**สาเหตุ:**
ฟังก์ชัน `read_line` ถูกออกแบบมาให้ **Append (ต่อท้าย)** ข้อมูลลงใน String buffer เสมอ ไม่ใช่ **Overwrite (เขียนทับ)** — เอกสารของ Rust ระบุชัดเจนว่า:

> *"Previous content of the buffer will be preserved. To avoid appending to the buffer, you need to `clear` it first."*

นอกจากนี้ `read_line` ยัง **รวมตัวอักษร `\n` (newline)** เข้ามาใน buffer ด้วย ทำให้เกิดปัญหาซ้อนกัน 2 ชั้น:

1. **ค่าเก่าพอกพูน** — ข้อมูลจากรอบก่อนๆ ไม่หายไปไหน
2. **เงื่อนไขออกจากลูปพังไปด้วย** — `input.trim()` จะไม่มีทางเท่ากับ `"exit"` อีกเลยหลังรอบแรก เพราะมีข้อมูลเก่าค้างอยู่ข้างหน้า ทำให้เกิด **Infinite Loop** โดยไม่รู้ตัว

> 💡 **สังเกต:** `.trim()` ตัดแค่ Whitespace ที่ **หัว** และ **ท้าย** เท่านั้น ไม่ได้ตัด `\n` ที่อยู่ **ตรงกลาง** ของ String

### วิธีแก้ (Solution A): ล้าง Buffer ทุกรอบ

ต้องสั่ง `.clear()` ก่อนอ่านค่าใหม่ในแต่ละรอบ:

```rust
loop {
    input.clear(); // ✅ ล้างค่าเก่าทิ้งก่อนอ่านใหม่
    println!("Type something:");
    io::stdin().read_line(&mut input).unwrap();

    println!("You typed: {}", input.trim());
    if input.trim() == "exit" { break; } // ✅ ทำงานถูกต้องแล้ว
}
```

---

### 2. กับดักที่ 2: Borrow Checker Error (E0502)

ปัญหานี้จะเกิดขึ้นเมื่อเราพยายาม **"ยืมค่า"** จาก `input` มาถือไว้ แล้ววนลูปกลับไปเรียก `read_line` ใหม่ โดยที่ Reference เก่ายังไม่ถูกคืน

**ตัวอย่างโค้ดที่ Compile ไม่ผ่าน:**

```rust
fn main() {
    let mut input = String::new();
    let mut history = Vec::new(); // เราอยากเก็บประวัติทุกอย่างที่ผู้ใช้พิมพ์

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim(); // ได้ &str ที่ชี้ไปยังข้อมูลใน input
        history.push(trimmed);      // ❌ Error! เก็บ reference ของ input ใส่ vector

        // จบรอบ -> วนกลับไป read_line(&mut input)
        // 💥 BOOM: cannot borrow `input` as mutable because it is also borrowed as immutable
    }
}
```

**ทำไมถึง Compile ไม่ผ่าน?**

1. `input.trim()` คืนค่าเป็น `&str` ซึ่งเป็น **Immutable Reference** ที่ชี้ไปยังข้อมูลภายใน `input`
2. `history.push(trimmed)` ทำให้ `history` ถือ Reference นั้นค้างไว้ — มันยังไม่ถูก Drop
3. เมื่อวนลูปกลับไป `read_line(&mut input)` ต้องการยืม `input` แบบ **Mutable** เพื่อแก้ไขค่า
4. **กฎเหล็กของ Rust:** ณ เวลาใดเวลาหนึ่ง ถ้ามีใครถือ Immutable Reference (`&`) อยู่ จะไม่อนุญาตให้เกิด Mutable Borrow (`&mut`) ซ้อนขึ้นมาอีก
5. เนื่องจาก `history` ยังคงถือ `&str` ที่ชี้มาที่ `input` อยู่ Rust จึงปฏิเสธไม่ให้ `read_line` แก้ไข `input`

> 📝 **หมายเหตุ:** Error นี้คือ **E0502** (cannot borrow as mutable because it is also borrowed as immutable) ไม่ใช่ E0499 ซึ่งเป็น error ของการยืม Mutable ซ้ำ 2 ครั้ง

### วิธีแก้ (Solution B): สร้างค่าใหม่ที่เป็น Owned Type

ถ้าต้องการเก็บค่าข้ามรอบลูป เราเก็บ Reference ไม่ได้ ต้อง **สร้าง String ใหม่** ที่เป็นเจ้าของข้อมูลเอง:

```rust
// เปลี่ยน history ให้เก็บ String (Owned) แทน &str (Borrowed)
let mut history: Vec<String> = Vec::new();

loop {
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    history.push(input.trim().to_string()); // ✅ .to_string() สร้าง String ใหม่ที่เป็นอิสระจาก input
    // ...
}
```

`.to_string()` (หรือ `String::from(...)`) จะ Copy ข้อมูลออกมาเป็น String ใหม่ ทำให้ `history` ไม่ได้ถือ Reference ที่ชี้ไปยัง `input` อีกต่อไป — Borrow Checker จึงพอใจ

---

### 3. แนวทางการเขียนลูปรับ Input (Best Practices)

มี 3 แนวทางหลัก ขึ้นอยู่กับว่าคุณให้ความสำคัญกับ Performance หรือ Convenience มากกว่ากัน

#### ท่าที่ 1: Buffer Reuse (เน้น Performance)

เหมาะกับงานที่ต้องอ่านข้อมูลจำนวนมหาศาล (เช่น อ่านไฟล์หลายล้านบรรทัด) เพราะจอง Memory เพียงครั้งเดียวแล้วใช้ซ้ำ:

```rust
let mut input = String::new(); // Allocate ครั้งเดียว
loop {
    input.clear(); // Reset length เป็น 0 แต่ไม่คืน capacity (เร็วมาก)
    std::io::stdin().read_line(&mut input)?;

    // ประมวลผล input...
}
```

> 🔍 **ทำไมเร็วกว่า?** — `String::clear()` แค่ตั้ง length กลับเป็น 0 โดยไม่ Deallocate หน่วยความจำที่จองไว้ ดังนั้น `read_line` ในรอบถัดไปไม่ต้อง Allocate ใหม่ ยกเว้นบรรทัดนั้นยาวกว่า capacity เดิม

#### ท่าที่ 2: Fresh Scope (เน้นความปลอดภัยและความชัดเจน)

เหมาะกับ CLI ทั่วไป เขียนง่าย ปลอดภัย ไม่ต้องกลัวลืม `clear()` และไม่มีปัญหา Borrow Checker ข้ามรอบ:

```rust
loop {
    let mut input = String::new(); // สร้างใหม่ทุกรอบ (Scope อยู่แค่ในลูป)
    std::io::stdin().read_line(&mut input)?;

    // ประมวลผล input...
    // input ถูก Drop เมื่อจบรอบ — ไม่มีทาง state ค้าง
}
```

> **Trade-off:** ท่านี้ Allocate ใหม่ทุกรอบ ช้ากว่าท่าที่ 1 เล็กน้อย แต่สำหรับ Interactive CLI ที่ผู้ใช้พิมพ์ทีละบรรทัด ความต่างนี้ไม่มีผลในทางปฏิบัติเลย

#### ท่าที่ 3: Iterator — The Rustacean Way

ใช้ `lines()` Iterator ซึ่งจัดการเรื่อง Buffer, Newline และ Error ให้เราอย่างสวยงาม:

```rust
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // lock() ล็อก stdin ไว้ครั้งเดียว ไม่ต้อง lock/unlock ทุกรอบ
    for line in stdin.lock().lines() {
        let content = line.unwrap(); // ได้ String ใหม่ (ไม่มี \n ติดมา)
        println!("You typed: {}", content);

        if content == "exit" { break; }
    }
}
```

> **สิ่งที่ `lines()` ทำให้อัตโนมัติ:**
>
> - ตัด `\n` ออกให้เรียบร้อย
> - คืน `String` ใหม่ (Owned) ทุกบรรทัด — ไม่มีปัญหา Borrow ข้ามรอบ
> - Wrap ใน `Result` เพื่อจัดการ I/O Error ได้
>
> **ข้อควรรู้:** `lines()` สร้าง `String` ใหม่ทุกบรรทัดเช่นเดียวกับท่าที่ 2 จึงช้ากว่าท่าที่ 1 (Buffer Reuse) ในงานที่ต้องอ่านจำนวนมาก แต่สำหรับงานทั่วไปถือว่าเหมาะสมที่สุด

---

### Checklist สรุป

1. **`read_line` = Append เสมอ** — ถ้าใช้ Buffer เดิมซ้ำ ต้องสั่ง `.clear()` ก่อนอ่านทุกครั้ง ไม่งั้นข้อมูลเก่าจะค้างและเงื่อนไขต่างๆ จะพัง
2. **`read_line` รวม `\n` มาด้วย** — ใช้ `.trim()` เพื่อตัดออก แต่ระวังว่า `.trim()` ตัดแค่หัวท้าย ไม่ตัดตรงกลาง
3. **ระวัง Reference ค้างข้ามรอบ** — ถ้าประกาศ Buffer นอกลูป ห้ามเก็บ `&str` ที่ชี้ไปยัง Buffer นั้นข้ามรอบลูป ให้ใช้ `.to_string()` สร้างค่าใหม่แทน
4. **เลือกท่าให้เหมาะกับงาน:**
   - เน้นเร็ว → Buffer Reuse + `.clear()`
   - เน้นชัวร์/เขียนง่าย → ประกาศ `String::new()` ในลูป
   - เน้นสไตล์ Idiomatic Rust → ใช้ `stdin().lock().lines()`

---

### แหล่งอ้างอิง

- [Rust Documentation: std::io::Stdin::read\_line](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)
- [Rust by Example: Read Lines](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html)
- [Rust Forum: Error when using read\_line in a loop](https://users.rust-lang.org/t/error-when-using-read-line-in-a-loop/138278)
- [Rust Forum: Why using the read\_lines() iterator is much slower than using read\_line()?](https://users.rust-lang.org/t/why-using-the-read-lines-iterator-is-much-slower-than-using-read-line/92815)
- [SE: Intuition behind why Rust's io::stdin().read\_line() relies on a side effect?](https://softwareengineering.stackexchange.com/questions/454811/intuition-behind-why-rusts-iostdin-read-line-relies-on-a-side-effect)
