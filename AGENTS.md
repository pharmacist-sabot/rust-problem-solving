# AGENTS.md
# AI Agent Instructions for Rust Problem-Solving Handbook
# Version: 1.0.0 | Last Updated: 2026-02-19
# Status: PRODUCTION | Criticality: HIGH

---

## 1. MISSION OVERVIEW

### 1.1 Objective
Refactor `src/SUMMARY.md` from a **linear list structure** to a **hierarchical structure with part dividers** while maintaining 100% backward compatibility and zero downtime.

### 1.2 Context
- **Current State**: Linear chapter list (`ob001`, `ob002`, `ob003`) with empty part placeholders
- **Target State**: Hierarchical structure with semantic grouping, clear navigation paths, and production-grade metadata
- **Risk Level**: MEDIUM-HIGH (affects site navigation and SEO)

### 1.3 Success Criteria
- [ ] All existing content remains accessible at identical URLs
- [ ] Navigation depth ≤ 3 levels (mdBook best practice)
- [ ] No broken internal links
- [ ] Mobile-responsive TOC rendering
- [ ] Preserved SEO rankings for existing pages

---

## 2. ARCHITECTURAL PRINCIPLES

### 2.1 Documentation Hierarchy (Diátaxis Framework)
```
Part (Conceptual Group)
├── Chapter (Major Topic)
│   ├── Section (Sub-topic)
│   └── Section (Sub-topic)
└── Chapter (Major Topic)
    └── Section (Sub-topic)
```

### 2.2 Naming Conventions (CRITICAL - DO NOT VIOLATE)

#### File Naming
```
# BEFORE (Anti-pattern)
ob001-ownership-of-a-self-argument.md
ob002-reborrowing-and-struct-transformation.md

# AFTER (Semantic naming)
part-01-ownership/self-ownership.md
part-01-ownership/reborrowing-struct-transform.md
part-01-ownership/read-line-loop-traps.md
```

**Rules:**
1. **Part prefix**: `part-XX-category/` (zero-padded, 2 digits)
2. **Kebab-case**: lowercase, hyphens for spaces
3. **Semantic meaning**: Name must describe content, not sequence
4. **No numbering in filenames**: Use `SUMMARY.md` for ordering
5. **Language suffix**: Only if multi-lang support added later (e.g., `.th.md`)

#### Directory Structure
```
src/
├── SUMMARY.md                    # Navigation manifest (single source of truth)
├── index.md                      # Landing page / book intro
├── part-01-ownership/
│   ├── index.md                  # Part overview + learning objectives
│   ├── self-ownership.md         # Former ob001
│   ├── reborrowing-transform.md  # Former ob002
│   └── read-line-traps.md        # Former ob003
├── part-02-type-system/
│   ├── index.md
│   └── ...
├── part-03-error-handling/
├── part-04-concurrency/
├── part-05-unsafe-ffi/
├── part-06-patterns/
└── appendices/
    ├── glossary.md
    ├── faq.md
    ├── troubleshooting.md
    └── contributors.md
```

### 2.3 URL Stability Contract
**ABSOLUTE REQUIREMENT**: All existing URLs must redirect or remain valid.

Current URLs:
- `https://pharmacist-sabot.github.io/rust-problem-solving/ob001-ownership-of-a-self-argument.html`

Must redirect to:
- `https://pharmacist-sabot.github.io/rust-problem-solving/part-01-ownership/self-ownership.html`

**Implementation**: Use mdBook's `output.html.redirect` or GitHub Pages redirect files.

---

## 3. SUMMARY.md SPECIFICATION

### 3.1 Structure Template

```markdown
# Summary

<!-- Part I: Ownership & Borrowing -->
# Part I: พื้นฐานการจัดการ Ownership, Borrowing และ Lifetime

- [บทนำสู่ Part I](part-01-ownership/index.md)
- [การเป็นเจ้าของของ `self`](part-01-ownership/self-ownership.md)
  - [ความแตกต่างระหว่าง Move และ Copy](part-01-ownership/self-ownership.md#move-vs-copy)
  - [เมื่อไหร่ที่ไม่ควรทำเป็น Copy](part-01-ownership/self-ownership.md#when-not-to-copy)
- [Reborrowing และการแปลงโครงสร้างข้อมูล](part-01-ownership/reborrowing-transform.md)
  - [ปัญหา Lifetime ที่พบบ่อย](part-01-ownership/reborrowing-transform.md#lifetime-problems)
  - [การใช้ Anonymous Lifetime](part-01-ownership/reborrowing-transform.md#anonymous-lifetime)
- [กับดักของ `read_line` ในลูป](part-01-ownership/read-line-traps.md)
  - [บั๊กเงียบจาก Buffer Reuse](part-01-ownership/read-line-traps.md#phantom-input)
  - [Borrow Checker Error E0499](part-01-ownership/read-line-traps.md#borrow-checker-error)

<!-- Part II: Type System -->
# Part II: ระบบประเภทข้อมูล (Type System)

- [บทนำสู่ Part II](part-02-type-system/index.md)
- [Generic Types และ Trait Bounds]()
- [Associated Types]()
- [Type Erasure และ Dynamic Dispatch]()

<!-- Part III: Error Handling -->
# Part III: การจัดการข้อผิดพลาด (Error Handling)

- [บทนำสู่ Part III](part-03-error-handling/index.md)
- [Result และ Option Types]()
- [Error Propagation Patterns]()
- [Custom Error Types]()

<!-- Part IV: Concurrency -->
# Part IV: Concurrency และ Parallelism

- [บทนำสู่ Part IV](part-04-concurrency/index.md)
- [Ownership ใน Multi-threading]()
- [Channel และ Message Passing]()
- [Shared State ด้วย Mutex และ RwLock]()

<!-- Part V: Unsafe Rust -->
# Part V: Unsafe Rust และ FFI

- [บทนำสู่ Part V](part-05-unsafe-ffi/index.md)
- [Raw Pointers]()
- [Unsafe Blocks และ Functions]()
- [FFI และ Calling C Code]()

<!-- Part VI: Patterns -->
# Part VI: Patterns และ Idioms

- [บทนำสู่ Part VI](part-06-patterns/index.md)
- [Builder Pattern]()
- [RAII และ Drop Trait]()
- [Interior Mutability Patterns]()

<!-- Appendices -->
# Appendices

- [A. Glossary - ศัพท์เทคนิค](appendices/glossary.md)
- [B. FAQ - คำถามที่พบบ่อย](appendices/faq.md)
- [C. Troubleshooting Index](appendices/troubleshooting.md)
- [D. รายชื่อผู้ร่วมพัฒนา](appendices/contributors.md)
```

### 3.2 Syntax Rules

#### Part Dividers (H1 with #)
```markdown
# Part X: [English Title] ([Thai Title])
```
- Must use H1 (`# `) for part separation
- Format: `Part X: English (Thai)`
- X is zero-padded number (01, 02, ...)

#### Chapter Entries (Bullet list)
```markdown
- [Display Title](path/to/file.md)
```
- Use `-` not `*` for consistency
- One space after `-`
- Title in brackets, path in parentheses
- Path relative to `src/`

#### Nested Sections (Indentation)
```markdown
- [Chapter Title](path/file.md)
  - [Section Anchor](path/file.md#anchor)
  - [Another Section](path/file.md#another-anchor)
```
- 2 spaces indentation (not tab)
- Anchor links must exist in target file

#### Draft Placeholders
```markdown
- [Topic Title]()  <!-- Draft - Coming Soon -->
```
- Empty parentheses `()` for draft chapters
- HTML comment with status

---

## 4. REFACTORING WORKFLOW

### 4.1 Pre-Flight Checklist

Before modifying any file:

```bash
# 1. Verify current state
cat src/SUMMARY.md

# 2. Check existing links
mdbook-linkcheck --standalone  # If installed

# 3. Backup current state
cp src/SUMMARY.md src/SUMMARY.md.backup.$(date +%Y%m%d)

# 4. Verify build works
mdbook build
mdbook serve --hostname 127.0.0.1 --port 3000
```

### 4.2 Step-by-Step Migration

#### Phase 1: File Structure (NO CONTENT CHANGES)
1. Create new directory structure
2. Move existing files to new locations with semantic names
3. Create placeholder `index.md` for each part
4. Update `SUMMARY.md` with new paths
5. **VERIFY**: `mdbook build` succeeds

#### Phase 2: Content Migration
1. Update internal links in moved files
2. Add frontmatter to each file:
   ```markdown
   ---
   title: "Thai Title"
   description: "Brief description"
   part: 1
   chapter: 1
   tags: ["ownership", "copy-trait"]
   ---
   ```
3. Add anchor targets for section links

#### Phase 3: URL Redirects (CRITICAL FOR SEO)
Create `book.toml` redirects:
```toml
[output.html.redirect]
"/ob001-ownership-of-a-self-argument.html" = "/part-01-ownership/self-ownership.html"
"/ob002-reborrowing-and-struct-transformation.html" = "/part-01-ownership/reborrowing-transform.html"
"/ob003-read-line-traps-in-loops.html" = "/part-01-ownership/read-line-traps.html"
```

#### Phase 4: Validation
```bash
# Build test
mdbook clean
mdbook build

# Link check
# If mdbook-linkcheck installed:
mdbook-linkcheck

# Manual verification
# - Check all parts appear in sidebar
# - Click through each chapter
# - Verify mobile menu works
# - Check search index rebuilt
```

### 4.3 Rollback Plan

If build fails at any stage:

```bash
# Immediate rollback
mv src/SUMMARY.md.backup.20250219 src/SUMMARY.md

# Or restore specific files from git
git checkout HEAD -- src/SUMMARY.md

# Verify recovery
mdbook build
```

---

## 5. QUALITY GATES

### 5.1 Pre-Commit Checklist

- [ ] `mdbook build` exits with code 0
- [ ] No warnings about missing files
- [ ] All part dividers render correctly in sidebar
- [ ] Chapter nesting ≤ 3 levels deep
- [ ] All existing URLs redirect correctly (test 3-5 samples)
- [ ] Mobile view: sidebar navigation works
- [ ] Search function returns results from all parts

### 5.2 Content Validation Rules

#### Rule 1: Path Consistency
```python
# Pseudocode validation
def validate_summary(summary_md):
    for line in summary_md:
        if "]( " in line and not "]()":
            path = extract_path(line)
            assert path.startswith(("part-", "appendices/", "index.md")), \
                f"Invalid path: {path}"
            assert not path.startswith("ob"), \
                f"Legacy path detected: {path}"
```

#### Rule 2: No Orphaned Files
Every `.md` file in `src/` must be referenced in `SUMMARY.md`.

#### Rule 3: Unique Anchors
All `#anchor` references must exist in target files.

---

## 6. ERROR HANDLING

### 6.1 Common Failure Modes

| Error | Cause | Solution |
|-------|-------|----------|
| `File not found` | Path typo in SUMMARY.md | Verify relative path from src/ |
| `Circular reference` | Symlink or include loop | Check for `{{#include}}` cycles |
| `Duplicate entry` | Same file listed twice | Remove duplicate line |
| `Missing chapter` | File moved but SUMMARY not updated | Update path or restore file |
| `Build warning: unused file` | Orphaned markdown file | Add to SUMMARY or delete |

### 6.2 Emergency Contacts (Metaphorical)

If agent encounters:
- **Ambiguous structure decision**: Refer to Diátaxis framework (Tutorial/How-to/Explanation/Reference)
- **Naming conflict**: Use suffixes `-overview`, `-advanced`, `-examples`
- **Circular dependency**: Flatten hierarchy, don't nest > 3 levels

---

## 7. METADATA SPECIFICATIONS

### 7.1 Part Index Template (`part-XX-category/index.md`)

```markdown
---
title: "Part X: English Title (Thai Title)"
description: "Learning objectives for this part"
part_number: 1
---

# Part X: English Title (Thai Title)

## เป้าหมายการเรียนรู้ (Learning Objectives)

หลังจากอ่าน Part นี้จบ คุณจะสามารถ:
- [ ] Objective 1
- [ ] Objective 2
- [ ] Objective 3

## บทเรียนในส่วนนี้

{{#include ../../SUMMARY.md:partX}}  <!-- If using include -->

## แนวทางการอ่าน

- **สำหรับมือใหม่**: อ่านตามลำดับ เริ่มจากบทแรก
- **สำหรับผู้มีประสบการณ์**: สามารถข้ามไปบทที่สนใจได้
- **เวลาที่ใช้**: ประมาณ X ชั่วโมง

## ก่อนอ่านส่วนนี้

ควรรู้พื้นฐาน:
- Prerequisite 1
- Prerequisite 2

## หลังอ่านส่วนนี้ แนะนำให้อ่าน

- [Link to next part](../part-02-.../index.md)
- [Link to related appendix](../../appendices/...)
```

### 7.2 Chapter Frontmatter Template

```markdown
---
title: "Thai Title"
title_en: "English Title"  # For searchability
part: 1
chapter: 1
order: 1  # Within part
description: "One sentence summary"
tags: ["ownership", "copy-trait", "self-parameter"]
difficulty: "beginner"  # beginner|intermediate|advanced
estimated_time: "15 min"
last_updated: "2025-02-19"
rust_version: "1.75+"
---
```

---

## 8. CI/CD INTEGRATION

### 8.1 GitHub Actions Validation

Add to `.github/workflows/deploy.yml`:

```yaml
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup mdBook
        uses: peaceiris/actions-mdbook@v2
        with:
          mdbook-version: "latest"
      
      - name: Install mdbook-linkcheck
        run: |
          curl -sSL https://github.com/Michael-F-Bryan/mdbook-linkcheck/releases/latest/download/mdbook-linkcheck.x86_64-unknown-linux-gnu.tar.gz | tar -xz
          chmod +x mdbook-linkcheck
          mv mdbook-linkcheck /usr/local/bin/
      
      - name: Validate SUMMARY.md structure
        run: |
          # Check for legacy prefixes
          if grep -E "ob[0-9]+" src/SUMMARY.md; then
            echo "ERROR: Legacy obXXX paths found in SUMMARY.md"
            exit 1
          fi
          
          # Check for consistent indentation
          if grep -E "^\t" src/SUMMARY.md; then
            echo "ERROR: Use spaces, not tabs for indentation"
            exit 1
          fi
      
      - name: Build book
        run: mdbook build
      
      - name: Check links
        run: mdbook-linkcheck --standalone
```

---

## 9. MULTI-LANGUAGE PREPARATION

While current scope is Thai-only, structure must support future i18n:

```
src/
├── en/                    # Future: English translation
│   ├── SUMMARY.md
│   └── part-01-ownership/
├── th/                    # Current: Thai (default)
│   ├── SUMMARY.md
│   └── part-01-ownership/
└── .md                    # Root redirects to default language
```

**Current implementation**: Keep Thai at root, add `lang: th` to frontmatter for future migration.

---

## 10. DECISION LOG

### 10.1 Key Decisions

| Date | Decision | Rationale | Impact |
|------|----------|-----------|--------|
| 2025-02-19 | Use `part-XX-` prefix | Zero-padding ensures correct sorting in file explorers | File naming |
| 2025-02-19 | Keep Thai at root | Current audience is Thai-speaking | URL structure |
| 2025-02-19 | 3-level max nesting | mdBook mobile UX degrades beyond 3 levels | UX |
| 2025-02-19 | Semantic filenames | SEO-friendly, maintainable | Long-term maintenance |

### 10.2 Rejected Alternatives

- **Numbered folders (`01-ownership/`)**: Hard to reorder, breaks git history on rename
- **Flat structure with prefixes**: Doesn't scale to 20+ chapters
- **Date-based organization**: Not suitable for technical documentation

---

## 11. APPENDICES

### A. Quick Reference: SUMMARY.md Syntax

```markdown
# Summary  <!-- Required title -->

# Part 1: Title        <!-- Part divider -->
- [Chapter](path.md)   <!-- Chapter link -->
  - [Section](path.md#anchor)  <!-- Nested section -->

# Part 2: Title        <!-- Next part -->
- [Draft Chapter]()    <!-- Draft placeholder -->
```

### B. File Movement Mapping

| Old Path | New Path | Status |
|----------|----------|--------|
| `src/ob001-ownership-of-a-self-argument.md` | `src/part-01-ownership/self-ownership.md` | MANDATORY |
| `src/ob002-reborrowing-and-struct-transformation.md` | `src/part-01-ownership/reborrowing-transform.md` | MANDATORY |
| `src/ob003-read-line-traps-in-loops.md` | `src/part-01-ownership/read-line-traps.md` | MANDATORY |
| `src/SUMMARY.md` | `src/SUMMARY.md` | UPDATE IN PLACE |

### C. Validation Commands

```bash
# Full validation suite
mdbook clean && \
mdbook build && \
echo "Build successful" && \
grep -c "^# Part" src/SUMMARY.md && \
echo "Part count" && \
grep -E "^\s*- \[" src/SUMMARY.md | wc -l && \
echo "Chapter count"
```

---

**END OF AGENTS.md**

**CRITICAL REMINDER**: This file contains production-grade instructions. 
- DO NOT skip validation steps
- DO NOT modify without version bump
- ALWAYS test in isolated branch first
- WHEN IN DOUBT, ask for human review

Version: 1.0.0 | Maintainer: Documentation Architect | Status: ACTIVE
```
