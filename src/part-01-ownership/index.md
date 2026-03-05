# พื้นฐานการจัดการ Ownership, Borrowing และ Lifetime

ระบบ Ownership คือหัวใจสำคัญที่ทำให้ Rust แตกต่างจากภาษาอื่น มันคือกลไกที่ช่วยจัดการหน่วยความจำอย่างปลอดภัย โดยไม่ต้องพึ่ง Garbage Collector และตรวจจับบั๊กที่เกี่ยวกับ Memory ได้ตั้งแต่ตอนคอมไพล์

แต่ระบบนี้ก็มาพร้อมกับ **กับดัก** และ **ความสับสน** มากมายที่แม้แต่นักพัฒนาที่มีประสบการณ์ก็ยังเจอ Part นี้จะพาคุณไปเจาะลึกปัญหาเหล่านั้นผ่านกรณีศึกษาจริง

## เนื้อหาในส่วนนี้
<!-- AUTO-INDEX-START -->
- [OB 001: การเป็นเจ้าของของ self ในฟังก์ชันเมมเบอร์ (Self Ownership)](self-ownership.md)
- [OB 002: Reborrowing ในเมธอดและการแปลงโครงสร้างข้อมูล](reborrowing-transform.md)
- [OB 003: กับดักของ read_line ในลูป (The Infinite Append & Borrowing)](read-line-traps.md)
<!-- AUTO-INDEX-END -->
