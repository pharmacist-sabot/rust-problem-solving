# Part III: การจัดการข้อผิดพลาด (Error Handling)

## เป้าหมายการเรียนรู้ (Learning Objectives)

หลังจากอ่าน Part นี้จบ คุณจะสามารถ:
- [ ] เข้าใจความแตกต่างระหว่าง `Result<T, E>` และ `Option<T>` และเลือกใช้ได้อย่างเหมาะสม
- [ ] ใช้ `?` Operator เพื่อส่งต่อข้อผิดพลาดอย่างกระชับ
- [ ] ออกแบบ Custom Error Types ที่รองรับการใช้งานจริง
- [ ] เลือก Error Handling Pattern ที่เหมาะกับแต่ละสถานการณ์

## บทเรียนในส่วนนี้

> 📝 บทเรียนในส่วนนี้กำลังอยู่ระหว่างการจัดทำ

- Result และ Option Types *(Coming Soon)*
- Error Propagation Patterns *(Coming Soon)*
- Custom Error Types *(Coming Soon)*

## แนวทางการอ่าน

- **สำหรับมือใหม่**: อ่านตามลำดับ เริ่มจาก Result และ Option Types ก่อน
- **สำหรับผู้มีประสบการณ์**: สามารถข้ามไปบทที่สนใจได้ เช่น Custom Error Types
- **เวลาที่ใช้**: ประมาณ 2-3 ชั่วโมง

## ก่อนอ่านส่วนนี้

ควรรู้พื้นฐาน:
- Ownership, Borrowing และ Lifetime ([Part I](../part-01-ownership/index.md))
- Pattern Matching (`match`, `if let`)
- Generic Types และ Enum เบื้องต้น

## หลังอ่านส่วนนี้ แนะนำให้อ่าน

- [Part IV: Concurrency และ Parallelism](../part-04-concurrency/index.md) — การจัดการ Error ใน Multi-threaded Context
- [Part VI: Patterns และ Idioms](../part-06-patterns/index.md) — Design Patterns ที่เกี่ยวข้อง
