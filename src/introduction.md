# Rust Problem-Solving Handbook (TH)

**Rust Problem-Solving Handbook (TH)** คือคู่มือแก้ปัญหา Rust Programming ภาษาไทย ที่รวบรวมปัญหาซึ่งนักพัฒนา Rust มักเจอในการทำงานจริง พร้อมวิธีแก้ไขที่อธิบายอย่างละเอียดผ่านกรณีศึกษา

เหมาะสำหรับนักพัฒนาที่เริ่มเขียน Rust แล้วเจอ Error ที่เข้าใจยาก หรือผู้ที่ต้องการเข้าใจแนวคิดเบื้องลึกของ Rust เช่น Ownership, Lifetime และ Borrow Checker ให้ถ่องแท้มากขึ้น

แต่ละบทเรียนเขียนขึ้นจากปัญหาจริง อธิบายที่มาของ Error และนำเสนอวิธีแก้ไขพร้อมโค้ดตัวอย่างที่รันได้ทันที

## โครงสร้างเนื้อหา

- [Part I: พื้นฐานการจัดการ Ownership, Borrowing และ Lifetime](part-01-ownership/index.md)
- [Part II: ระบบประเภทข้อมูล (Type System)](part-02-type-system/index.md)
- [Part III: การจัดการข้อผิดพลาด (Error Handling)](part-03-error-handling/index.md)
- [Part IV: Concurrency และ Parallelism](part-04-concurrency/index.md)
- [Part V: Unsafe Rust และ FFI](part-05-unsafe-ffi/index.md)
- [Part VI: Patterns และ Idioms](part-06-patterns/index.md)

## วิธีใช้งานหนังสือเล่มนี้

- **สำหรับมือใหม่:** แนะนำให้อ่านตามลำดับรายตอน โดยเฉพาะ Part I: Ownership & Borrowing
- **สำหรับการแก้ปัญหาเฉพาะจุด:** สามารถค้นหา Error Message หรือชื่อหัวข้อที่ตรงกับปัญหาในสารบัญ แล้วเลือกข้ามไปอ่านในส่วน Troubleshooting ได้เลย

ทุกคนสามารถร่วมกันพัฒนาเนื้อหาหนังสือเล่มนี้ได้ทางบอร์ด [GitHub Issues & Pull Requests](https://github.com/suradet-ps/rust-problem-solving) ของโปรเจกต์นี้
