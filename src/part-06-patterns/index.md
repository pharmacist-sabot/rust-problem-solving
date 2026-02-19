# Part VI: Patterns และ Idioms

## เป้าหมายการเรียนรู้ (Learning Objectives)

หลังจากอ่าน Part นี้จบ คุณจะสามารถ:
- [ ] เข้าใจและนำ Builder Pattern ไปใช้สร้าง API ที่ใช้งานง่ายและปลอดภัย
- [ ] ใช้ RAII และ Drop Trait เพื่อจัดการทรัพยากรอัตโนมัติ
- [ ] เลือกใช้ Interior Mutability Patterns (`Cell`, `RefCell`, `OnceCell`) ได้อย่างเหมาะสม
- [ ] เขียนโค้ด Rust ที่เป็น Idiomatic และบำรุงรักษาง่าย

## บทเรียนในส่วนนี้

- Builder Pattern *(เร็วๆ นี้)*
- RAII และ Drop Trait *(เร็วๆ นี้)*
- Interior Mutability Patterns *(เร็วๆ นี้)*

## แนวทางการอ่าน

- **สำหรับมือใหม่**: แนะนำให้อ่าน Part I–III ก่อน แล้วค่อยกลับมาอ่าน Part นี้ เพราะ Patterns หลายตัวอาศัยความเข้าใจเรื่อง Ownership, Type System และ Error Handling
- **สำหรับผู้มีประสบการณ์**: สามารถข้ามไปบทที่สนใจได้โดยตรง แต่ละบทเขียนให้อ่านแยกกันได้
- **เวลาที่ใช้**: ประมาณ 2–3 ชั่วโมง

## ก่อนอ่านส่วนนี้

ควรรู้พื้นฐาน:
- Ownership, Borrowing และ Lifetime ([Part I](../part-01-ownership/index.md))
- Generic Types และ Trait Bounds ([Part II](../part-02-type-system/index.md))
- การจัดการ Error ด้วย Result และ Option ([Part III](../part-03-error-handling/index.md))

## หลังอ่านส่วนนี้ แนะนำให้อ่าน

- [Glossary — ศัพท์เทคนิค](../appendices/glossary.md)
- [FAQ — คำถามที่พบบ่อย](../appendices/faq.md)
