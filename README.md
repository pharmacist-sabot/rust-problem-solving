# Rust Problem-Solving Handbook (TH) 🦀

[![Deploy mdBook to GitHub Pages](https://github.com/pharmacist-sabot/rust-problem-solving/actions/workflows/deploy.yml/badge.svg)](https://github.com/pharmacist-sabot/rust-problem-solving/actions/workflows/deploy.yml)
[![License: CC BY-SA 4.0](https://img.shields.io/badge/License-CC%20BY--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-sa/4.0/)
[![Book: mdBook](https://img.shields.io/badge/Book-mdBook-rust.svg)](https://rust-lang.github.io/mdBook/)

> **คู่มือแก้ปัญหา Rust Programming ฉบับภาษาไทย** — รวบรวมแนวคิด ปัญหาที่พบบ่อย และเจาะลึกการทำงานเบื้องหลังของภาษา Rust เพื่อให้คุณเข้าใจ "Why" ก่อนที่จะแก้ "How"

---

## อ่านออนไลน์ (Read Online)

คุณสามารถอ่านเนื้อหาทั้งหมดได้ฟรีที่เว็บไซต์
👉 **[https://pharmacist-sabot.github.io/rust-problem-solving/](https://pharmacist-sabot.github.io/rust-problem-solving/)**

---

## เกี่ยวกับโปรเจกต์ (About)

Rust เป็นภาษาที่มี Learning Curve ค่อนข้างสูง โดยเฉพาะเรื่อง Memory Management, Ownership และ Borrow Checker ซึ่งมักจะเป็นกำแพงด่านแรกสำหรับผู้เริ่มต้น

**Rust Problem-Solving Handbook** เกิดขึ้นมาเพื่อรวบรวมกรณีศึกษา (Case Studies) และปัญหาที่นักพัฒนา Rust มักจะเจอ โดยไม่ได้บอกแค่ว่า "แก้โค้ดอย่างไร" แต่จะอธิบายลงลึกไปถึง **"ทำไม Rust ถึงออกแบบมาแบบนี้"** และ **"ทำไมคอมไพเลอร์ถึงด่าเรา"**

### เนื้อหาครอบคลุม
โปรเจกต์นี้แบ่งเนื้อหาออกเป็นส่วนสำคัญ ดังนี้
- **Part I:** พื้นฐานการจัดการ Ownership, Borrowing และ Lifetime
- **Part II:** ระบบประเภทข้อมูล (Type System)
- **Part III:** การจัดการข้อผิดพลาด (Error Handling)
- **Part IV:** Concurrency และ Parallelism
- **Part V:** Unsafe Rust และ FFI
- **Part VI:** Patterns และ Idioms

---

## การติดตั้งและรันบนเครื่อง (Local Development)

โปรเจกต์นี้สร้างด้วย [mdBook](https://github.com/rust-lang/mdBook) หากคุณต้องการรันเซิร์ฟเวอร์เพื่ออ่านบนเครื่องตัวเอง หรือต้องการแก้ไขเนื้อหา สามารถทำได้ตามขั้นตอนดังนี้

### 1. ติดตั้ง Rust และ Cargo
หากยังไม่มี Rust ให้ติดตั้งผ่าน [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. ติดตั้ง mdBook
```bash
cargo install mdbook
```

### 3. Clone Repository
```bash
git clone https://github.com/pharmacist-sabot/rust-problem-solving.git
cd rust-problem-solving
```

### 4. รันเซิร์ฟเวอร์ (Live Reload)
```bash
mdbook serve
```
จากนั้นเปิดเบราว์เซอร์ไปที่ `http://localhost:3000`

---

## การร่วมพัฒนา (Contributing)

เรายินดีต้อนรับทุกการแบ่งปัน ไม่ว่าจะเป็นการแก้คำผิด เพิ่มตัวอย่างโค้ด หรือเขียนบทความหัวข้อใหม่ที่คุณคิดว่าเป็นประโยชน์

1. **Fork** repository นี้
2. สร้าง **Branch** ใหม่ (`git checkout -b feature/my-new-content`)
3. **Commit** การแก้ไขของคุณ (`git commit -m 'feat: add topic about Smart Pointers'`)
4. **Push** ไปยัง Branch ของคุณ (`git push origin feature/my-new-content`)
5. เปิด **Pull Request** เข้ามาที่ branch `main`

> **Note:** โปรดตรวจสอบความถูกต้องของเนื้อหาและการสะกดคำก่อนส่ง Pull Request

---

## ลิขสิทธิ์ (License)

เนื้อหาและสื่อการสอนในโปรเจกต์นี้ เผยแพร่ภายใต้สัญญาอนุญาต
**Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)**

คุณสามารถ:
- **Share:** คัดลอก แจกจ่าย และส่งต่อเนื้อหา
- **Adapt:** ดัดแปลง แก้ไข หรือนำไปต่อยอด

ภายใต้เงื่อนไข:
- **Attribution:** ต้องให้เครดิตเจ้าของผลงานเดิม
- **ShareAlike:** หากมีการดัดแปลง งานใหม่ต้องใช้สัญญาอนุญาตเดียวกัน (CC BY-SA 4.0)

อ่านรายละเอียดฉบับเต็มได้ที่ไฟล์ [LICENSE](LICENSE)
