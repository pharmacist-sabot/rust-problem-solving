# Unsafe Rust และ FFI

Rust ถูกออกแบบมาให้ปลอดภัยเป็นค่าเริ่มต้น แต่มีบางสถานการณ์ที่ต้องก้าวข้าม Safety Guarantee เหล่านั้น เช่น การเข้าถึง Hardware โดยตรง การ Optimize Performance ในระดับต่ำ หรือการเรียกใช้โค้ดจากภาษาอื่น Rust จึงมี `unsafe` Keyword เพื่อเปิดช่องทางให้ทำสิ่งเหล่านี้ได้อย่างมีขอบเขตชัดเจน

ความท้าทายที่นักพัฒนามักเจอคือการตัดสินใจว่าเมื่อไหร่ที่ `unsafe` จำเป็นจริงๆ การเขียน Safety Invariants ที่ครบถ้วน และการสร้าง Safe Abstraction ที่ครอบ Unsafe Code เพื่อป้องกันไม่ให้ผู้ใช้ API ต้องรับภาระด้านความปลอดภัยเอง นอกจากนี้ การทำ FFI (Foreign Function Interface) เพื่อเรียกใช้ C Library ก็เป็นอีกหัวข้อที่ต้องเข้าใจ Unsafe อย่างถ่องแท้

Part นี้จะครอบคลุมตั้งแต่ Raw Pointers และ Unsafe Block ไปจนถึงการเขียน FFI Binding ที่ปลอดภัยและใช้งานได้จริง

## เนื้อหาในส่วนนี้
<!-- AUTO-INDEX-START -->
<!-- AUTO-INDEX-END -->