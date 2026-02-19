# FAQ — คำถามที่พบบ่อย

รวบรวมคำถามที่นักพัฒนามักถามเกี่ยวกับ Rust และเนื้อหาในหนังสือเล่มนี้ พร้อมคำตอบสั้นๆ และลิงก์ไปยังบทที่เกี่ยวข้อง

---

## ทั่วไป

### Q: หนังสือเล่มนี้เหมาะกับใคร?

**A:** เหมาะกับนักพัฒนาที่มีพื้นฐาน Rust เบื้องต้นแล้ว (เช่น อ่าน [The Rust Programming Language](https://doc.rust-lang.org/book/) จบแล้ว) และต้องการเข้าใจปัญหาที่พบบ่อยในการเขียน Rust จริงๆ พร้อมวิธีแก้ไขอย่างเป็นระบบ

### Q: ต้องอ่านตามลำดับไหม?

**A:** ไม่จำเป็นครับ แต่ละบทเขียนให้อ่านแยกกันได้ อย่างไรก็ตาม หากคุณเป็นมือใหม่ แนะนำให้อ่านตามลำดับ Part โดยเริ่มจาก [Part I: Ownership, Borrowing และ Lifetime](../part-01-ownership/index.md)

### Q: Rust เวอร์ชันไหนที่ใช้ในหนังสือ?

**A:** โค้ดตัวอย่างทั้งหมดทดสอบกับ Rust 1.75 ขึ้นไป (Stable) แต่แนวคิดส่วนใหญ่ใช้ได้กับทุกเวอร์ชันที่รองรับ Edition 2021

---

## Ownership, Borrowing & Lifetime

### Q: ทำไมเรียก `angle.cos()` แล้ว `angle` ยังใช้ได้อยู่ ทั้งๆ ที่ `cos` รับ `self`?

**A:** เพราะ `f32` implement `Copy` Trait ทำให้ Rust ทำสำเนาค่าไปใช้ในฟังก์ชันแทนการ Move ตัวแปรต้นทางจึงยังใช้ได้ อ่านเพิ่มเติมได้ที่ [การเป็นเจ้าของของ `self`](../part-01-ownership/self-ownership.md)

### Q: `self`, `&self`, `&mut self` ต่างกันอย่างไร?

**A:**
- `self` — ยึดครอง (Move) หรือทำสำเนา (Copy) ค่าเข้าไปในฟังก์ชัน
- `&self` — ยืมอ่านอย่างเดียว (Shared/Immutable Borrow)
- `&mut self` — ยืมไปแก้ไข (Exclusive/Mutable Borrow)

อ่านรายละเอียดและตัวอย่างได้ที่ [การเป็นเจ้าของของ `self`](../part-01-ownership/self-ownership.md)

### Q: Anonymous Lifetime `'_` คืออะไร? ใช้เมื่อไหร่?

**A:** `'_` คือ Lifetime ที่บอกให้ Rust อนุมาน (infer) ให้อัตโนมัติ ใช้เมื่อต้องการระบุว่า Return Type มี Lifetime แต่ไม่ต้องการเขียน Lifetime Parameter แบบเต็ม เช่น:

```rust
fn to_other_ctx(&mut self) -> OtherCtx<'_> { ... }
```

อ่านเพิ่มเติมได้ที่ [Reborrowing และการแปลงโครงสร้างข้อมูล](../part-01-ownership/reborrowing-transform.md)

### Q: ทำไม `read_line` ถึงต่อท้าย (append) ข้อมูลแทนที่จะเขียนทับ?

**A:** เป็นการออกแบบเพื่อ Performance — ผู้เรียกสามารถนำ Buffer กลับมาใช้ซ้ำได้โดยไม่ต้อง Allocate หน่วยความจำใหม่ทุกครั้ง วิธีแก้คือเรียก `.clear()` ก่อน `read_line` ทุกรอบของลูป อ่านเพิ่มเติมได้ที่ [กับดักของ `read_line` ในลูป](../part-01-ownership/read-line-traps.md)

### Q: Reborrowing คืออะไร? ต่างจาก Borrowing ปกติอย่างไร?

**A:** Reborrowing คือการที่ Rust ยืม Reference ที่มีอยู่แล้วมาใช้ต่อ โดยสร้าง Reference ใหม่ที่มี Lifetime สั้นกว่า Reference ต้นฉบับ ซึ่งช่วยให้ Mutable Reference สามารถถูกส่งต่อไปยังฟังก์ชันอื่นได้โดยไม่เสีย Ownership ของ Reference เดิม อ่านเพิ่มเติมได้ที่ [Reborrowing และการแปลงโครงสร้างข้อมูล](../part-01-ownership/reborrowing-transform.md)

---

## การมีส่วนร่วม

### Q: พบข้อผิดพลาดในหนังสือ จะแจ้งได้ที่ไหน?

**A:** สามารถเปิด Issue ได้ที่ [GitHub Repository](https://github.com/pharmacist-sabot/rust-problem-solving/issues) ของโปรเจกต์ครับ

### Q: อยากเขียนบทใหม่หรือแก้ไขเนื้อหา ทำได้ไหม?

**A:** ยินดีเลยครับ สามารถ Fork Repository แล้วส่ง Pull Request เข้ามาได้ ดูรายละเอียดเพิ่มเติมที่ [รายชื่อผู้ร่วมพัฒนา](contributors.md)

---

> **มีคำถามอื่นที่ไม่อยู่ในรายการนี้?** สามารถเปิด Issue ใน [GitHub Repository](https://github.com/pharmacist-sabot/rust-problem-solving/issues) ได้เลยครับ เราจะเพิ่มคำตอบเข้ามาในหน้านี้
