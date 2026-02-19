---
title: "Part I: พื้นฐานการจัดการ Ownership, Borrowing และ Lifetime"
description: "เรียนรู้แนวคิดหลักของ Rust เรื่อง Ownership, Borrowing และ Lifetime ผ่านปัญหาจริงที่นักพัฒนาพบบ่อย"
part_number: 1
lang: th
---

# Part I: พื้นฐานการจัดการ Ownership, Borrowing และ Lifetime

ระบบ Ownership คือหัวใจสำคัญที่ทำให้ Rust แตกต่างจากภาษาอื่น — มันคือกลไกที่ช่วยจัดการหน่วยความจำอย่างปลอดภัย โดยไม่ต้องพึ่ง Garbage Collector และตรวจจับบั๊กที่เกี่ยวกับ Memory ได้ตั้งแต่ตอนคอมไพล์

แต่ระบบนี้ก็มาพร้อมกับ **กับดัก** และ **ความสับสน** มากมายที่แม้แต่นักพัฒนาที่มีประสบการณ์ก็ยังเจอ Part นี้จะพาคุณไปเจาะลึกปัญหาเหล่านั้นผ่านกรณีศึกษาจริง

## เป้าหมายการเรียนรู้ (Learning Objectives)

หลังจากอ่าน Part นี้จบ คุณจะสามารถ:

- [ ] อธิบายความแตกต่างระหว่าง `self`, `&self` และ `&mut self` ได้อย่างชัดเจน
- [ ] เข้าใจว่า `Copy` Trait ส่งผลต่อ Ownership อย่างไร และเมื่อไหร่ที่ไม่ควรใช้
- [ ] เข้าใจกลไก Reborrowing และสามารถแก้ปัญหา Lifetime Error ที่เกิดจากการแปลง Struct ได้
- [ ] ใช้ Anonymous Lifetime (`'_`) ได้อย่างถูกต้อง
- [ ] ระบุและหลีกเลี่ยงกับดักของ `read_line` ในลูปได้ทั้ง Silent Bug และ Borrow Checker Error
- [ ] เลือกรูปแบบการรับ Input (Buffer Reuse, Fresh Scope, Iterator) ได้เหมาะสมกับงาน

## บทเรียนในส่วนนี้

1. **[การเป็นเจ้าของของ `self`](self-ownership.md)** — ทำไม `angle.cos()` ถึงไม่กิน `angle` หายไป? เจาะลึก Move vs Copy Semantics และหลุมพรางของ `mut self`
2. **[Reborrowing และการแปลงโครงสร้างข้อมูล](reborrowing-transform.md)** — เมื่อ Borrow Checker ไม่ยอมให้แปลง Struct ที่ถือ `&mut` แก้ไขอย่างไรด้วย Anonymous Lifetime
3. **[กับดักของ `read_line` ในลูป](read-line-traps.md)** — บั๊กเงียบจาก Buffer ที่ไม่ถูก Clear และ Error E0502 จากการเก็บ Reference ข้ามรอบ

## แนวทางการอ่าน

- **สำหรับมือใหม่**: อ่านตามลำดับ เริ่มจากบทแรก (`self-ownership`) เพราะแต่ละบทต่อยอดจากแนวคิดก่อนหน้า
- **สำหรับผู้มีประสบการณ์**: สามารถข้ามไปบทที่สนใจได้เลย แต่ละบทเป็นอิสระจากกัน
- **เวลาที่ใช้**: ประมาณ 1-2 ชั่วโมง

## ก่อนอ่านส่วนนี้

ควรรู้พื้นฐาน:

- Syntax พื้นฐานของ Rust (ประกาศตัวแปร, ฟังก์ชัน, Struct)
- แนวคิด Ownership เบื้องต้น (Move, Borrow) จาก [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- การใช้ `impl` block สำหรับ Method Definition
