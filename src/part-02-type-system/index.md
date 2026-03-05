# ระบบประเภทข้อมูล (Type System)

ระบบ Type System ของ Rust เป็นหนึ่งในจุดเด่นที่ทำให้ภาษานี้ทั้งปลอดภัยและยืดหยุ่น Rust ใช้ Static Typing ร่วมกับ Trait System ที่ทรงพลัง ทำให้สามารถเขียนโค้ดที่ Generic และ Reusable ได้โดยไม่เสียประสิทธิภาพ

ความท้าทายที่นักพัฒนามักเจอในหมวดนี้คือการเลือกระหว่าง Generic Type Parameters กับ Associated Types, การตัดสินใจใช้ Static Dispatch หรือ Dynamic Dispatch, และการออกแบบ API ที่ใช้ Trait Bounds อย่างเหมาะสมโดยไม่ทำให้ Type Signature ซับซ้อนเกินไป

Part นี้จะพาคุณเจาะลึกแนวคิดเหล่านี้ผ่านปัญหาจริงที่เกิดขึ้นในการพัฒนาซอฟต์แวร์ด้วย Rust ตั้งแต่การใช้ Generics อย่างมีประสิทธิภาพ ไปจนถึงการทำ Type Erasure สำหรับ Dynamic Dispatch

## เนื้อหาในส่วนนี้
<!-- AUTO-INDEX-START -->
<!-- AUTO-INDEX-END -->
