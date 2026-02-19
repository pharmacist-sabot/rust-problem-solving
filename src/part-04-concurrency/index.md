# Part IV: Concurrency และ Parallelism

## เป้าหมายการเรียนรู้ (Learning Objectives)

หลังจากอ่าน Part นี้จบ คุณจะสามารถ:
- [ ] เข้าใจว่า Ownership Model ของ Rust ช่วยป้องกัน Data Race ได้อย่างไร
- [ ] ใช้ Channel สำหรับ Message Passing ระหว่าง Thread ได้
- [ ] ใช้ `Mutex`, `RwLock` และ `Arc` สำหรับ Shared State อย่างปลอดภัย
- [ ] แยกแยะความแตกต่างระหว่าง Concurrency กับ Parallelism ได้

## บทเรียนในส่วนนี้

- Ownership ใน Multi-threading *(coming soon)*
- Channel และ Message Passing *(coming soon)*
- Shared State ด้วย Mutex และ RwLock *(coming soon)*

## แนวทางการอ่าน

- **สำหรับมือใหม่**: ควรอ่าน [Part I: พื้นฐานการจัดการ Ownership, Borrowing และ Lifetime](../part-01-ownership/index.md) ให้แม่นยำก่อน เพราะแนวคิด Ownership เป็นหัวใจของ Fearless Concurrency ใน Rust
- **สำหรับผู้มีประสบการณ์**: สามารถข้ามไปหัวข้อที่สนใจได้ เช่น Shared State หรือ Channel
- **เวลาที่ใช้**: ประมาณ 2-3 ชั่วโมง

## ก่อนอ่านส่วนนี้

ควรรู้พื้นฐาน:
- Ownership, Borrowing และ Lifetime ([Part I](../part-01-ownership/index.md))
- Trait Bounds และ Generic Types ([Part II](../part-02-type-system/index.md))
- Error Handling เบื้องฐาน ([Part III](../part-03-error-handling/index.md))

## หลังอ่านส่วนนี้ แนะนำให้อ่าน

- [Part V: Unsafe Rust และ FFI](../part-05-unsafe-ffi/index.md) — เรียนรู้ว่า Unsafe Code เกี่ยวข้องกับ Concurrency Primitives อย่างไร
- [Part VI: Patterns และ Idioms](../part-06-patterns/index.md) — รูปแบบการออกแบบที่ใช้ร่วมกับ Concurrent Code
