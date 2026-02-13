# OB 002: Reborrowing ในเมธอดและการแปลงโครงสร้างข้อมูล

ในบทนี้ เราจะมาเจาะลึกแนวคิดเรื่อง **Reborrowing** (การยืมซ้ำ) เมื่อใช้งานกับโครงสร้างข้อมูลที่มีการยืมแบบ Mutable (`&mut`) และการแปลง (Transform) ระหว่าง Context (บริบท) ที่แตกต่างกัน

ปัญหานี้มักจะเกิดขึ้นเมื่อเราพยายามสร้าง Struct ใหม่จากข้อมูลที่มีอยู่ใน Struct เดิม โดยยังคงรักษาความสัมพันธ์ของ Lifetime ไว้ ซึ่งอาจทำให้เกิดความสับสนและข้อผิดพลาดจาก Borrow Checker ได้หากไม่เข้าใจกลไกการทำงานของ Rust อย่างถ่องแท้

## 1. ปัญหาที่พบ

พิจารณาโค้ดตัวอย่างต่อไปนี้ ซึ่งจำลองสถานการณ์ที่มี `LayoutCtx` ถือ Mutable Reference ของ `State` และต้องการแปลงเป็น `OtherCtx` เพื่อใช้งานชั่วคราว

```rust
struct State { }

struct LayoutCtx<'s> {
    window_state: &'s mut State
}

struct OtherCtx<'s> {
    window_state: &'s mut State
}

impl<'s> LayoutCtx<'s> {
    fn fee(&self) { }
    
    // พยายามแปลงเป็น OtherCtx โดยใช้ lifetime 's เดิม
    fn to_other_ctx(&mut self) -> OtherCtx<'s> {
        OtherCtx {
            window_state: self.window_state
        }
    }
}

impl<'s> OtherCtx<'s> {
    fn faa(&self) { }
}

fn layout(lo_ctx: &mut LayoutCtx) {
    let mut other_ctx = lo_ctx.to_other_ctx();
    other_ctx.faa();
    lo_ctx.fee();
}
```

เมื่อพยายามคอมไพล์โค้ดข้างต้น Rust จะแจ้ง Error ดังนี้:

```text
error: lifetime may not live long enough
  --> src/main.rs:15:9
   |
13 |     fn to_other_ctx(&mut self) -> OtherCtx<'s> {
   |                     - let's call the lifetime of this reference `'1`
   |
15 |         OtherCtx {
   |         ^ assignment requires that `'1` must outlive `'s`
```

Error นี้บอกว่า Method นี้ควรจะ return ข้อมูลที่มี Lifetime `'s` แต่กำลัง return ข้อมูลที่มี Lifetime `'1` (ซึ่งคือ Lifetime ของ `&mut self`)

## 2. สิ่งที่กำลังเกิดขึ้น

เพื่อเข้าใจปัญหานี้ เราต้องแยกแยะความแตกต่างระหว่างสองสิ่งนี้ให้ชัดเจน:

1. **Reference เดิม (`'s`)**: คือ `&'s mut State` ที่ถูกเก็บไว้ใน `LayoutCtx` ซึ่งมีอายุยาวนานเท่ากับ `'s`
2. **Reborrow**: คือการยืม Reference นั้นมาใช้ต่อในช่วงเวลาสั้นๆ ภายในฟังก์ชันหรือเมทอด

### แผนภาพแสดงช่วงเวลาของ Lifetime

ลองจินตนาการเส้นเวลาของการทำงาน:

```text
's (Lifetime ของ State):  [==================================================]
'1 (Lifetime ของ &mut self):       [=============]  <-- ช่วงเวลาของ to_other_ctx
                                          ^
                                          |
                        เราพยายามคืน 's ออกไปตรงนี้ (ซึ่งทำไม่ได้)
```

> **ข้อสังเกต:** การทำแบบนี้ **ไม่ใช่การย้าย (Move)** `window_state` ออกจาก `LayoutCtx` แต่เป็นการ **ยืมซ้ำ (Reborrow)** มาใช้สร้าง `OtherCtx` ชั่วคราวเท่านั้น ข้อมูลต้นฉบับยังคงอยู่ที่เดิม แต่ถูก "ล็อก" ไว้ไม่ให้ใช้ซ้อนกันจนกว่า `OtherCtx` จะคืนสิทธิ์กลับมา

เมื่อเราเรียกเมธอด:

```rust
let mut other_ctx = lo_ctx.to_other_ctx();
```

`lo_ctx` ถูกส่งเข้ามาในฐานะ `&mut LayoutCtx` ซึ่งมี Lifetime ชั่วคราว (สมมติว่าเป็น `'1`) ที่สั้นกว่า `'s` มาก

ในเมทอด `to_other_ctx`:

- เราเข้าถึง `self.window_state` ผ่าน `&mut self`
- Rust ไม่สามารถให้เรา "ดึง" `&'s mut State` ออกไปตรงๆ ได้ เพราะมันติดอยู่กับ `&mut self` ที่มีอายุสั้นกว่า (`'1`)
- การพยายาม return `OtherCtx<'s>` คือการพยายามบอกว่า "ฉันจะคืน Struct ที่ถือ Reference ยาวนานเท่ากับ `'s`"
- แต่ในความเป็นจริง เรากำลังสร้าง Struct จากการ Reborrow ผ่าน `self` ซึ่งมีอายุแค่ `'1` เท่านั้น

จึงเกิดความขัดแย้งกันระหว่าง **สิ่งที่สัญญาว่าจะคืน (`'s`)** กับ **สิ่งที่ทำได้จริง (`'1`)**

## 3. การแก้ปัญหาโดยใช้ Anonymous Lifetime

วิธีแก้ไขที่ถูกต้องและเป็น Idiomatic Rust คือการยอมรับความจริงว่า `OtherCtx` ที่เราสร้างขึ้นมาใหม่นี้ **เป็นเพียงลูกหนี้ชั่วคราว (Temporary Reborrow)** เท่านั้น ไม่ใช่เจ้าของสิทธิ์การยืมยาวนานเท่ากับต้นฉบับ

เราสามารถใช้ **Anonymous Lifetime (`'_`)** เพื่อบอก Rust ว่าให้คำนวณ Lifetime ตามบริบทการใช้งานจริง:

```rust
```rust
impl<'s> LayoutCtx<'s> {
    // แบบย่อ (Recommended)
    fn to_other_ctx(&mut self) -> OtherCtx<'_> {
        OtherCtx {
            window_state: self.window_state
        }
    }
    
    // แบบเต็ม (Explicit Lifetime) เพื่อให้เห็นภาพชัดเจน
    // 'a คือ lifetime ของการยืม &mut self
    // เราคืน OtherCtx<'a> ที่มีอายุเท่ากับ 'a
    fn to_other_ctx_explicit<'a>(&'a mut self) -> OtherCtx<'a> {
        OtherCtx {
            window_state: self.window_state
        }
    }
}
```

ในทางปฏิบัติ การใช้ `'_` (Anonymous Lifetime) เป็นที่นิยมมากกว่าเพราะเขียนสั้นกว่าและ Rust สามารถอนุมาน Lifetime `'a` ให้เราได้เองโดยอัตโนมัติ แต่ความหมายเบื้องหลังนั้นเหมือนกันคือ **"อายุของสิ่งที่คืนกลับไป จะผูกอยู่กับอายุของการยืม `self`"**

### ทำไมวิธีนี้ถึงได้ผล?

การใช้ `OtherCtx<'_>` (หรือเขียนเต็มๆ ว่า `OtherCtx<'a>` โดยที่ `'a` ผูกกับ lifetime ของ `&'a mut self`) เป็นการบอก Rust ว่า:

> "ค่า `OtherCtx` ที่คืนออกไป จะมีอายุยืนยาวเท่าที่จำเป็นสำหรับความถูกต้องของการยืมนี้เท่านั้น (ไม่เกินอายุของ `&mut self`)"

ด้วยวิธีนี้ Rust จะเข้าใจว่า:

1. `to_other_ctx` ทำการ **Reborrow** `window_state` จาก `self`
2. `OtherCtx` ที่ได้มา จะถือ Reference ที่ถูก Reborrow นี้
3. เมื่อ `OtherCtx` หมดอายุ (เช่น จบ scope หรือเลิกใช้งาน) สิทธิ์การยืมจะถูกคืนกลับไปที่ `LayoutCtx` ทำให้เราสามารถเรียก `lo_ctx.fee()` ต่อได้

## 4. ทำไม `OtherCtx<'s>` ถึงใช้ไม่ได้?

หากสมมติว่า Rust ยอมให้เรา return `OtherCtx<'s>` ได้ จะเกิดอะไรขึ้น?

1. `OtherCtx` จะถือ `&'s mut State` ซึ่งเป็น Reference ตัวเดียวกับที่ `LayoutCtx` ถืออยู่
2. ทั้ง `OtherCtx` และ `LayoutCtx` จะมีสิทธิ์เข้าถึง Mutable Reference ของ `State` พร้อมกัน ในช่วงเวลา `'s` เดียวกัน
3. สิ่งนี้จะละเมิดกฎ **Aliasing XOR Mutation** (มี Mutable Reference ได้เพียงตัวเดียวในขณะใดขณะหนึ่ง) อย่างรุนแรง

การใช้ Anonymous Lifetime (`'_`) ช่วยป้องกันปัญหานี้โดยการบังคับให้ Lifetime ของ `OtherCtx` สั้นลงเหลือแค่ช่วงที่เราใช้งานมันจริงๆ (Nested Scope) ทำให้ไม่เกิดการซ้อนทับกันของ Mutable Reference ที่อันตราย

## 5. ตัวอย่างการใช้งานจริง

สถานการณ์นี้มีประโยชน์มากเมื่อเราต้องการ "แตก" (Split) หรือ "แปลง" (Transform) Context ใหญ่ๆ ให้เป็น Context ย่อยเพื่อส่งให้ฟังก์ชันอื่นทำงานเฉพาะทาง โดยที่ Context หลักยังคงกลับมาใช้งานต่อได้หลังจาก Context ย่อยทำงานเสร็จ

```rust
// จำลองการใช้งานในระบบ UI
fn build_ui(mut layout_ctx: LayoutCtx) {
    // 1. แปลงเป็น Context ย่อยเพื่อวาดปุ่ม
    {
        let mut other_ctx = layout_ctx.to_other_ctx(); 
        other_ctx.faa(); // ใช้ other_ctx ทำงานบางอย่าง...
    } // จบ scope: other_ctx คืนสิทธิ์การยืมกลับไปที่ layout_ctx

    // 2. LayoutCtx กลับมาใช้งานต่อได้ (ไม่ถูก move หายไป)
    layout_ctx.fee(); 
}
```

หากเราใช้ `OtherCtx<'s>` (แบบที่ผิด) การเรียก `layout_ctx.to_other_ctx()` จะทำให้ `layout_ctx` ถูกล็อกยาวนานเท่ากับ `'s` (ตลอดอายุของ `State`) ส่งผลให้บรรทัดที่ 2 (`layout_ctx.fee()`) คอมไพล์ไม่ผ่านเพราะติดกฎการยืมซ้อน

## สรุป

- **Reborrowing เป็นกลไกอัตโนมัติ**: Rust ทำการ Reborrow Reference ให้เราโดยอัตโนมัติเมื่อมีการส่งต่อ Mutable Reference ผ่านฟังก์ชัน เพื่อลด Lifetime ลงให้เหมาะสมกับบริบทนั้นๆ
- **Struct Transformation ต้องระวัง Lifetime**: เมื่อแปลง Struct หนึ่งไปเป็นอีก Struct หนึ่งที่ถือ Reference ตัวเดิม หากผ่าน `&mut self` มักจะต้องใช้ **Anonymous Lifetime (`'_`)** ใน Return Type เสมอ
- **`'_` คือเพื่อนที่ดี**: การใช้ `OtherCtx<'_>` สื่อความหมายว่า "Struct นี้ยืมข้อมูลมาใช้ชั่วคราว" ซึ่งตรงกับพฤติกรรมที่ Borrow Checker ต้องการ และช่วยให้โค้ดมีความยืดหยุ่น ปลอดภัย

การเข้าใจเรื่องนี้จะช่วยให้คุณออกแบบ API ที่ซับซ้อนขึ้นได้ โดยเฉพาะในรูปแบบ Context Passing หรือ Builder Pattern ที่มีการส่งต่อ State ระหว่างกัน
