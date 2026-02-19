---
title: "Part II: ระบบประเภทข้อมูล (Type System)"
description: "เรียนรู้ระบบ Type System อันทรงพลังของ Rust ตั้งแต่ Generics, Trait Bounds, Associated Types ไปจนถึง Dynamic Dispatch"
part_number: 2
lang: th
---

# Part II: ระบบประเภทข้อมูล (Type System)

## เป้าหมายการเรียนรู้ (Learning Objectives)

หลังจากอ่าน Part นี้จบ คุณจะสามารถ:
- [ ] เข้าใจและใช้งาน Generic Types ร่วมกับ Trait Bounds ได้อย่างมีประสิทธิภาพ
- [ ] แยกแยะความแตกต่างระหว่าง Associated Types และ Generic Type Parameters ได้
- [ ] เลือกใช้ Static Dispatch และ Dynamic Dispatch ได้เหมาะสมกับสถานการณ์
- [ ] ออกแบบ API ที่ยืดหยุ่นโดยใช้ระบบ Type System ของ Rust

## บทเรียนในส่วนนี้

> 🚧 **อยู่ระหว่างการจัดทำ** — บทเรียนในส่วนนี้กำลังถูกเขียนและจะเผยแพร่เร็วๆ นี้

- Generic Types และ Trait Bounds *(Coming Soon)*
- Associated Types *(Coming Soon)*
- Type Erasure และ Dynamic Dispatch *(Coming Soon)*

## แนวทางการอ่าน

- **สำหรับมือใหม่**: อ่านตามลำดับ เริ่มจากบทแรก เพราะแต่ละบทต่อยอดจากความรู้ก่อนหน้า
- **สำหรับผู้มีประสบการณ์**: สามารถข้ามไปบทที่สนใจได้ โดยเฉพาะเรื่อง Type Erasure ที่เป็นหัวข้อขั้นสูง
- **เวลาที่ใช้**: ประมาณ 2-3 ชั่วโมง

## ก่อนอ่านส่วนนี้

ควรรู้พื้นฐาน:
- Ownership, Borrowing และ Lifetime ([Part I](../part-01-ownership/index.md))
- Trait พื้นฐานใน Rust (เช่น `Display`, `Debug`, `Clone`)
- การใช้งาน Struct และ Enum เบื้องต้น

## หลังอ่านส่วนนี้ แนะนำให้อ่าน

- [Part III: การจัดการข้อผิดพลาด (Error Handling)](../part-03-error-handling/index.md) — นำ Type System ไปประยุกต์ใช้กับ Error Types
- [Part VI: Patterns และ Idioms](../part-06-patterns/index.md) — รูปแบบการออกแบบที่ใช้ Type System เป็นแกนหลัก
