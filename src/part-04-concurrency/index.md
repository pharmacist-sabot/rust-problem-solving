# Concurrency และ Parallelism

Concurrency เป็นหนึ่งในจุดแข็งที่โดดเด่นของ Rust ด้วยแนวคิด "Fearless Concurrency" ที่ระบบ Ownership และ Type System ช่วยป้องกัน Data Race ได้ตั้งแต่ตอนคอมไพล์ ทำให้สามารถเขียนโปรแกรม Multi-threaded ได้อย่างปลอดภัยโดยไม่ต้องพึ่งพา Runtime Overhead

ความท้าทายที่นักพัฒนามักเจอในหมวดนี้คือการทำความเข้าใจว่า Ownership Rules ส่งผลต่อ Multi-threading อย่างไร การเลือกระหว่าง Channel (Message Passing) กับ Shared State (`Mutex`, `RwLock`) และการใช้ `Arc` เพื่อแชร์ข้อมูลข้าม Thread อย่างถูกต้อง

Part นี้จะครอบคลุมแนวคิดหลักของ Concurrent Programming ใน Rust ตั้งแต่พื้นฐานของ Thread Spawning ไปจนถึง Synchronization Primitives ที่ใช้กันในการพัฒนาจริง

## เนื้อหาในส่วนนี้
<!-- AUTO-INDEX-START -->
<!-- AUTO-INDEX-END -->
