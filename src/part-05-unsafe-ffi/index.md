# Part V: Unsafe Rust และ FFI

## เป้าหมายการเรียนรู้ (Learning Objectives)

หลังจากอ่าน Part นี้จบ คุณจะสามารถ:
- [ ] เข้าใจว่าเมื่อไหร่ที่จำเป็นต้องใช้ `unsafe` และเมื่อไหร่ที่ไม่ควรใช้
- [ ] ใช้ Raw Pointers (`*const T`, `*mut T`) ได้อย่างถูกต้องและปลอดภัย
- [ ] เขียน Unsafe Block และ Unsafe Function พร้อม Safety Invariants ที่ชัดเจน
- [ ] เรียกใช้ฟังก์ชันจากภาษา C ผ่าน Foreign Function Interface (FFI)
- [ ] สร้าง Safe Abstraction ครอบ Unsafe Code เพื่อป้องกันข้อผิดพลาด

## บทเรียนในส่วนนี้

- [Raw Pointers]() <!-- Draft - Coming Soon -->
- [Unsafe Blocks และ Functions]() <!-- Draft - Coming Soon -->
- [FFI และ Calling C Code]() <!-- Draft - Coming Soon -->

## แนวทางการอ่าน

- **สำหรับมือใหม่**: ควรอ่าน Part I–IV ให้เข้าใจก่อน เพราะ Unsafe Rust ต้องอาศัยความเข้าใจเรื่อง Ownership, Lifetime และ Type System เป็นพื้นฐาน
- **สำหรับผู้มีประสบการณ์**: สามารถข้ามไปบทที่สนใจได้ โดยเฉพาะ FFI หากต้องการเชื่อมต่อกับ C Library
- **เวลาที่ใช้**: ประมาณ 3–4 ชั่วโมง

## ก่อนอ่านส่วนนี้

ควรรู้พื้นฐาน:
- Ownership, Borrowing และ Lifetime (Part I)
- ระบบประเภทข้อมูลและ Trait (Part II)
- การจัดการข้อผิดพลาดใน Rust (Part III)
- ความเข้าใจพื้นฐานเกี่ยวกับ Pointers และ Memory Layout

## หลังอ่านส่วนนี้ แนะนำให้อ่าน

- [Part VI: Patterns และ Idioms](../part-06-patterns/index.md)
- [A. Glossary - ศัพท์เทคนิค](../appendices/glossary.md)
