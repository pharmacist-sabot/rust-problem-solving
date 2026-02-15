## OB 003: กับดักของ `read_line` ในลูป (The Infinite Append & Borrowing)

การรับ Input จากผู้ใช้ผ่าน Console เป็นเรื่องพื้นฐาน แต่ใน Rust การใช้ `read_line` ภายในลูป (`loop`, `while`) มีหลุมพรางซ่อนอยู่ 2 ชั้น คือ **"บั๊กเงียบที่โปรแกรมไม่พังแต่ทำงานผิด"** และ **"Error จาก Borrow Checker"**

บทความนี้จะพาไปดูสาเหตุและวิธีจัดการให้ถูกต้องแบบมืออาชีพครับ

### 1. กับดักที่ 1: "บั๊กเงียบ" (The Phantom Input)

ลองดูโค้ดตัวอย่างนี้ ที่มือใหม่ (และมือเก๋าที่เผลอ) มักจะเขียน:

```rust
use std::io;

fn main() {
    let mut input = String::new(); // 1. สร้าง buffer นอกลูป (เพื่อประหยัด mem)

    loop {
        println!("Type something:");
        // 2. อ่านค่าใส่ buffer ตัวเดิม
        io::stdin().read_line(&mut input).expect("Failed to read");

        println!("You typed: {}", input.trim());
        
        if input.trim() == "exit" { break; }
    }
}
```

**ผลลัพธ์ที่ได้ (ความหายนะ):**
*   รอบที่ 1 พิมพ์ "A" -> Output: "A" (ดูปกติ)
*   รอบที่ 2 พิมพ์ "B" -> Output: **"AB"** (เฮ้ย! A มาจากไหน?)
*   รอบที่ 3 พิมพ์ "C" -> Output: **"ABC"**

**สาเหตุ:**
ฟังก์ชัน `read_line` ถูกออกแบบมาให้ **Append (ต่อท้าย)** ข้อมูลลงใน String buffer เสมอ ไม่ใช่การ **Overwrite (เขียนทับ)**
เมื่อเราประกาศ `input` ไว้นอกลูป ค่าเก่าจึงค้างอยู่และพอกพูนขึ้นเรื่อยๆ

**วิธีแก้ (Solution A): ล้าง Buffer**
ต้องสั่ง `.clear()` ก่อนหรือหลังการใช้งานในแต่ละรอบ

```rust
loop {
    input.clear(); // ✅ ล้างค่าเก่าทิ้งก่อนอ่านใหม่
    io::stdin().read_line(&mut input).unwrap();
    // ...
}
```

---

### 2. กับดักที่ 2: Borrow Checker Error (E0499)

ปัญหานี้จะเกิดขึ้นเมื่อเราพยายาม **"ยืมค่า"** จาก `input` มาถือไว้ แล้ววนลูปกลับไปเรียก `read_line` ใหม่ โดยที่ Reference เก่ายังไม่ถูกคืน

**ตัวอย่างโค้ดที่พัง:**

```rust
fn main() {
    let mut input = String::new();
    let mut history = Vec::new(); // เราอยากเก็บประวัติสิ่งที่พิมพ์

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let trimmed = input.trim(); 
        history.push(trimmed); // ❌ Error! เราพยายามเก็บ reference ของ input ใส่ vector
        
        // จบลูป -> วนกลับไป read_line(&mut input)
        // 💥 BOOM: cannot borrow `input` as mutable because it is also borrowed as immutable
    }
}
```

**ทำไมถึงพัง?**
1.  `history.push(trimmed)` เป็นการเก็บ **Reference** (`&str`) ที่ชี้ไปยัง `input`
2.  เมื่อวนลูปกลับไป `read_line(&mut input)` ต้องการยืม `input` แบบ **Mutable** เพื่อแก้ไขค่า
3.  **กฎเหล็กของ Rust:** ห้ามแก้ไขข้อมูล (Mutable Borrow) ถ้ายังมีคนอื่นถือ Reference (Immutable Borrow) ชี้มาที่ข้อมูลนั้นอยู่
4.  เนื่องจาก `history` ยังถือ Reference ของ `input` อยู่ Rust จึงไม่อนุญาตให้ `read_line` แก้ไข `input` ได้

**วิธีแก้ (Solution B): Clone ข้อมูล (Owned Type)**
ถ้าต้องการเก็บค่าข้ามรอบลูป เราไม่สามารถเก็บ Reference ได้ ต้องเก็บเป็นค่าใหม่ (`String`) แทน

```rust
// เปลี่ยน history เป็นเก็บ String (Owned) แทน &str (Borrowed)
let mut history: Vec<String> = Vec::new(); 

loop {
    // ... read_line ...
    history.push(input.trim().to_string()); // ✅ สร้าง String ใหม่แยกอิสระจาก input
}
```

---

### 3. สรุปแนวทางการเขียน (Best Practices)

เรามี 2 ท่ามาตรฐานในการเขียน Loop รับ Input ขึ้นอยู่กับว่าคุณแคร์เรื่อง Performance หรือ Convenience มากกว่ากัน

#### ท่าที่ 1: Performance (Buffer Reuse)
เหมาะสำหรับโปรแกรมที่ต้องวนลูปอ่านค่ามหาศาล (High Frequency) เพราะลดการจองหน่วยความจำ (Allocation) ใหม่ทุกรอบ

```rust
let mut input = String::new(); // จองครั้งเดียว
loop {
    input.clear(); // แค่ reset pointer, ไม่คืน memory (เร็ว)
    std::io::stdin().read_line(&mut input)?;
    
    // Process input...
}
```

#### ท่าที่ 2: Safety & Convenience (Fresh Scope)
เหมาะสำหรับ CLI ทั่วไป เขียนง่าย ปลอดภัย ไม่ต้องกลัวลืม `clear()` และไม่ต้องกังวลเรื่อง Borrow Checker ข้ามรอบ

```rust
loop {
    let mut input = String::new(); // จองใหม่ทุกรอบ (Scope อยู่แค่ในนี้)
    std::io::stdin().read_line(&mut input)?;
    
    // Process input...
    // input ถูก Drop ทิ้งเมื่อจบลูป ไม่มีทางมี state ค้าง
}
```

#### ท่าที่ 3: The "Rustacean" Way (Iterator)
ใช้ Iterator `lines()` ซึ่งจัดการเรื่อง Buffer และ Result ให้เราแบบสวยงาม

```rust
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // lock() ช่วยให้ทำงานเร็วขึ้นเมื่ออ่านต่อเนื่อง
    for line in stdin.lock().lines() {
        let content = line.unwrap(); // ได้ String ใหม่มาเลย
        println!("You typed: {}", content);
    }
}
```

### Checklist สรุป

1.  **`read_line` คือการ Append:** ถ้าใช้ Buffer เดิมซ้ำๆ ต้องสั่ง `.clear()` เสมอ
2.  **ระวัง Reference ค้าง:** ถ้าประกาศ Buffer ไว้นอกลูป ห้ามเก็บ Reference ของ Buffer นั้นข้ามรอบลูป (เช่น เก็บใส่ Vec)
3.  **เลือกท่าให้ถูก:**
    *   ถ้าเน้นเร็วมาก -> ใช้ Buffer Reuse + `.clear()`
    *   ถ้าเน้นชัวร์/เขียนง่าย -> ประกาศ `String::new()` ในลูป
    *   ถ้าชอบสไตล์ Functional -> ใช้ `stdin().lines()`

---
**แหล่งอ้างอิง:**
- [Rust Forum: Error when using read_line in a loop](https://users.rust-lang.org/t/error-when-using-read-line-in-a-loop/138278)
- [Rust Documentation: std::io::Stdin::read_line](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)
