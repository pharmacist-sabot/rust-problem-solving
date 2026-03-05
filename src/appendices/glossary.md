# Glossary — ศัพท์เทคนิค

รวมคำศัพท์เทคนิคที่พบบ่อยในหนังสือเล่มนี้และในโลกของ Rust พร้อมคำอธิบายภาษาไทยที่เข้าใจง่าย เรียงตามลำดับตัวอักษร A–Z

---

## A

### Aliasing
การที่มี Reference มากกว่าหนึ่งตัวชี้ไปยังข้อมูลเดียวกันในเวลาเดียวกัน Rust บังคับกฎ **Aliasing XOR Mutation** คือ ณ เวลาใดเวลาหนึ่ง จะมีได้แค่ Immutable Reference หลายตัว **หรือ** Mutable Reference เพียงตัวเดียว แต่จะมีทั้งสองแบบพร้อมกันไม่ได้

### Anonymous Lifetime (`'_`)
Lifetime แบบไม่ระบุชื่อ ใช้เพื่อบอก Rust ว่า "ให้คำนวณ Lifetime ที่เหมาะสมจากบริบทเอง" มักใช้ใน Return Type ของฟังก์ชันที่คืน Reference ซึ่งผูกกับ `&self` หรือ `&mut self`

### Associated Type
Type ที่ถูกกำหนดไว้ภายใน Trait Definition ทำให้ผู้ Implement สามารถเลือก Concrete Type ได้ เช่น `type Item` ใน `Iterator` Trait

## B

### Borrow / Borrowing (การยืม)
การเข้าถึงข้อมูลโดยไม่ย้าย Ownership มี 2 แบบ:

- **Shared Borrow** (`&T`) — ยืมอ่านอย่างเดียว มีได้หลายตัวพร้อมกัน
- **Mutable Borrow** (`&mut T`) — ยืมไปแก้ไข มีได้เพียงตัวเดียว

### Borrow Checker
ส่วนของ Rust Compiler ที่ตรวจสอบว่ากฎ Borrowing ถูกปฏิบัติตามอย่างถูกต้อง หากมีการละเมิดกฎ (เช่น มี Mutable Borrow ซ้อนกับ Immutable Borrow) จะแจ้ง Compile Error

### Builder Pattern
รูปแบบการออกแบบที่ใช้ Method Chaining เพื่อสร้าง Object ทีละขั้นตอน เหมาะสำหรับ Struct ที่มี Field จำนวนมากหรือมี Optional Field

## C

### Clone
Trait ที่ให้ความสามารถในการ "ทำสำเนาเชิงลึก" (Deep Copy) ของข้อมูล ต่างจาก `Copy` ตรงที่อาจมีต้นทุนสูง (เช่น ต้อง Allocate หน่วยความจำใหม่)

### Copy
Trait ที่บ่งบอกว่า Type นั้นสามารถ **ทำสำเนาบิต (Bitwise Copy)** ได้โดยอัตโนมัติ เมื่อ Type มี `Copy` การส่งค่าเข้าฟังก์ชันจะเป็นการ Copy แทนการ Move ตัวอย่าง: `i32`, `f64`, `bool`, `char`

### Concurrency (การทำงานพร้อมกัน)
ความสามารถในการจัดการงานหลายอย่างในเวลาที่ทับซ้อนกัน ใน Rust การทำ Concurrency มีความปลอดภัยสูงเพราะ Borrow Checker ช่วยป้องกัน Data Race ตั้งแต่ตอน Compile

## D

### Data Race
สถานการณ์ที่ Thread สองตัว (หรือมากกว่า) เข้าถึงข้อมูลเดียวกันพร้อมกัน โดยอย่างน้อยหนึ่งตัวกำลังเขียนข้อมูล และไม่มีการ Synchronization ระบบ Ownership ของ Rust ป้องกันปัญหานี้ได้ในระดับ Compile Time

### Dereference (การเข้าถึงค่าจาก Reference)
การใช้ `*` เพื่อเข้าถึงค่าที่ Reference ชี้อยู่ เช่น `*ptr` จะได้ค่าที่ `ptr` ชี้ไป

### Drop
Trait ที่กำหนดพฤติกรรมเมื่อค่าถูกทำลาย (ออกจาก Scope) ใช้สำหรับจัดการทรัพยากรเช่นการปิดไฟล์หรือคืนหน่วยความจำ เป็นพื้นฐานของรูปแบบ RAII

### Dynamic Dispatch
กลไกที่ Rust เลือกฟังก์ชันที่จะเรียกตอน Runtime (ผ่าน vtable) โดยใช้ Trait Object (`dyn Trait`) ต่างจาก Static Dispatch ที่เลือกตอน Compile Time

## E

### Enum (Enumeration)
ชนิดข้อมูลที่สามารถเป็นได้หลาย "รูปแบบ" (Variant) แต่ละ Variant สามารถมีข้อมูลแนบได้ เช่น `Option<T>` (`Some(T)` หรือ `None`) และ `Result<T, E>` (`Ok(T)` หรือ `Err(E)`)

## F

### FFI (Foreign Function Interface)
กลไกสำหรับเรียกใช้ฟังก์ชันจากภาษาอื่น (โดยเฉพาะ C) หรือให้ภาษาอื่นเรียกใช้ฟังก์ชัน Rust ต้องใช้ `extern` block และมักต้องเขียนภายใน `unsafe`

## G

### Generic Type
Type ที่ยังไม่ระบุ Concrete Type เช่น `Vec<T>` โดยที่ `T` จะถูกแทนที่ด้วย Type จริงตอนใช้งาน ช่วยให้เขียนโค้ดที่ใช้ซ้ำได้กับหลาย Type

## I

### Immutable (ไม่เปลี่ยนแปลง)
ค่าเริ่มต้นของตัวแปรใน Rust — ไม่สามารถแก้ไขค่าได้หลังจากกำหนดค่าแล้ว หากต้องการแก้ไขต้องประกาศด้วย `mut`

### Interior Mutability
รูปแบบที่ยอมให้แก้ไขข้อมูลภายในได้แม้จะถือ Immutable Reference อยู่ ใช้ Types เช่น `Cell<T>`, `RefCell<T>` หรือ `Mutex<T>` ซึ่งเลื่อนการตรวจสอบ Borrowing Rules ไปทำตอน Runtime แทน

### Iterator
Trait ที่ให้ความสามารถในการวนซ้ำ (Iterate) ผ่านลำดับของค่า ต้อง Implement `fn next(&mut self) -> Option<Self::Item>` เป็นหัวใจของ Functional Programming Style ใน Rust

## L

### Lifetime (อายุการใช้งาน)
ช่วงเวลาที่ Reference ยังคงใช้งานได้ (Valid) Rust ใช้ Lifetime Annotation (`'a`, `'b`, ฯลฯ) เพื่อบอก Compiler ว่า Reference ตัวไหนต้องมีอายุยาวนานเพียงใดเพื่อป้องกัน Dangling Reference

## M

### Match
Expression ที่ใช้สำหรับ Pattern Matching กับ Enum หรือค่าต่างๆ คล้ายกับ `switch` ในภาษาอื่น แต่ทรงพลังกว่ามากเพราะ Rust บังคับให้จัดการทุก Case (Exhaustive Matching)

### Move (การย้ายความเป็นเจ้าของ)
การโอน Ownership จากตัวแปรหนึ่งไปยังอีกตัวแปรหนึ่ง หลังจาก Move แล้ว ตัวแปรเดิมจะใช้งานไม่ได้อีก เกิดขึ้นกับ Type ที่ไม่ได้ Implement `Copy`

### Mutable (`mut`)
คำสำคัญที่ใช้ประกาศว่าตัวแปรสามารถเปลี่ยนแปลงค่าได้ เช่น `let mut x = 5;`

### Mutex (Mutual Exclusion)
โครงสร้างข้อมูลสำหรับ Synchronization ที่อนุญาตให้ Thread เดียวเท่านั้นเข้าถึงข้อมูลภายในได้ในเวลาใดเวลาหนึ่ง ใช้คู่กับ `Arc` สำหรับ Shared Ownership ข้าม Thread

## O

### Option\<T\>
Enum ที่แทน "มีค่า" (`Some(T)`) หรือ "ไม่มีค่า" (`None`) ใช้แทน `null` ในภาษาอื่น ช่วยป้องกัน Null Pointer Exception ได้ตั้งแต่ Compile Time

### Ownership (ความเป็นเจ้าของ)
กฎหลักของ Rust ที่กำหนดว่า ข้อมูลทุกชิ้นในหน่วยความจำมี "เจ้าของ" (Owner) ได้เพียงตัวเดียว เมื่อ Owner ออกจาก Scope ข้อมูลจะถูกทำลาย (Drop) โดยอัตโนมัติ

## P

### Pattern Matching
กลไกการเปรียบเทียบค่ากับรูปแบบ (Pattern) ต่างๆ ใช้ผ่าน `match`, `if let`, `while let` เป็นเครื่องมือสำคัญสำหรับจัดการ `Option`, `Result` และ Enum

### Pointer (ตัวชี้)
ค่าที่เก็บ Memory Address ของข้อมูล ใน Rust มี 2 ประเภทหลัก:

- **Reference** (`&T`, `&mut T`) — ปลอดภัย ถูกตรวจสอบโดย Borrow Checker
- **Raw Pointer** (`*const T`, `*mut T`) — ไม่ถูกตรวจสอบ ต้องใช้ภายใน `unsafe`

## R

### RAII (Resource Acquisition Is Initialization)
รูปแบบการจัดการทรัพยากรที่ผูกอายุของทรัพยากร (เช่น ไฟล์, หน่วยความจำ, Lock) กับอายุของ Object เมื่อ Object ถูก Drop ทรัพยากรจะถูกคืนอัตโนมัติ Rust ใช้รูปแบบนี้เป็นพื้นฐาน

### Reborrowing (การยืมซ้ำ)
กลไกที่ Rust สร้าง Reference ใหม่จาก Reference ที่มีอยู่แล้ว โดย Reference ใหม่จะมี Lifetime สั้นกว่า ช่วยให้ส่งต่อ `&mut T` ให้ฟังก์ชันอื่นได้โดยไม่ต้อง Move Mutable Reference ต้นฉบับ

### Reference (การอ้างอิง)
ค่าที่ "ชี้" ไปยังข้อมูลของตัวแปรอื่น โดยไม่ย้าย Ownership ใช้ `&` สำหรับ Shared Reference และ `&mut` สำหรับ Mutable Reference

### Result\<T, E\>
Enum ที่แทน "สำเร็จ" (`Ok(T)`) หรือ "ผิดพลาด" (`Err(E)`) เป็น Type หลักสำหรับ Error Handling ใน Rust ใช้ร่วมกับ `?` Operator เพื่อส่งต่อ Error ได้อย่างกระชับ

## S

### Scope (ขอบเขต)
ช่วงของโค้ดที่ตัวแปรยังคง Valid อยู่ โดยทั่วไปคือตั้งแต่จุดที่ประกาศจนถึงปีกกาปิด `}` เมื่อออกจาก Scope ค่าจะถูก Drop

### Self / self

- `Self` (ตัวใหญ่) — อ้างถึง Type ของ `impl` block ปัจจุบัน
- `self` (ตัวเล็ก) — อ้างถึง Instance ของ Type ปัจจุบัน ใช้เป็น Parameter แรกของ Method

### Slice
"มุมมอง" (View) ที่ชี้ไปยังส่วนหนึ่งของข้อมูลที่ต่อเนื่องกันในหน่วยความจำ เช่น `&[T]` คือ Slice ของ Array/Vec และ `&str` คือ String Slice

### Static Dispatch
กลไกที่ Rust เลือกฟังก์ชันที่จะเรียกตอน Compile Time (ผ่าน Monomorphization) ทำให้ไม่มี Runtime Cost เพิ่ม ใช้กับ Generics + Trait Bounds

### String vs &str

- `String` — เป็น Owned Type เก็บข้อมูลบน Heap สามารถแก้ไขได้
- `&str` — เป็น Borrowed Type (String Slice) ชี้ไปยังข้อมูลที่อยู่ที่อื่น ไม่สามารถแก้ไขได้โดยตรง

## T

### Trait
ชุดของ Method Signatures ที่กำหนด "ความสามารถ" ของ Type คล้ายกับ Interface ในภาษาอื่น แต่ทรงพลังกว่า เช่น `Display`, `Debug`, `Clone`, `Iterator`

### Trait Bound
เงื่อนไขที่กำหนดว่า Generic Type ต้อง Implement Trait บางตัว เช่น `fn foo<T: Display>(x: T)` หมายความว่า `T` ต้อง Implement `Display`

### Trait Object (`dyn Trait`)
ค่าที่ Type จริงจะถูกกำหนดตอน Runtime ใช้สำหรับ Dynamic Dispatch เช่น `Box<dyn Error>` สามารถเก็บ Error Type ใดก็ได้ที่ Implement `Error` Trait

## U

### Unsafe
คำสำคัญที่ใช้ระบุว่าโค้ดส่วนนั้นอยู่นอกเหนือการรับประกันความปลอดภัยของ Rust Compiler ทำให้สามารถ:

- Dereference Raw Pointer
- เรียกใช้ Unsafe Function
- เข้าถึง/แก้ไข Static Mutable Variable
- Implement Unsafe Trait

## V

### Vec\<T\> (Vector)
โครงสร้างข้อมูลแบบ Dynamic Array ที่เก็บข้อมูลบน Heap สามารถเพิ่ม/ลดขนาดได้ เป็น Collection ที่ใช้บ่อยที่สุดใน Rust

## Z

### Zero-Cost Abstraction
หลักการออกแบบของ Rust ที่ว่า Abstraction ต่างๆ (เช่น Generics, Iterators, Closures) จะไม่มีค่าใช้จ่ายเพิ่มเติมตอน Runtime เทียบกับการเขียนโค้ดเองด้วยมือ (Hand-written code)

---

> **หมายเหตุ:** Glossary นี้จะถูกอัปเดตเพิ่มเติมเมื่อมีบทเรียนใหม่ๆ ในหนังสือ หากพบคำศัพท์ที่ต้องการให้เพิ่ม สามารถ [แจ้งได้ที่ GitHub Issues](https://github.com/pharmacist-sabot/rust-problem-solving/issues)
