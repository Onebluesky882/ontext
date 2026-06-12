# Production Test Checklist — ontext (Wails)

Checklist สำหรับทดสอบก่อนปล่อย production ของ ontext (macOS / Windows)

## 1. Build & Packaging

- [ ] `wails build` สำเร็จทั้ง macOS และ Windows (release mode, ไม่มี debug flags)
- [ ] Binary ขนาดและ icon/metadata ถูกต้อง (ชื่อแอป, version, bundle ID)
- [ ] macOS: app ลงนาม (codesign) และผ่าน notarization (ไม่มี Gatekeeper warning)
- [ ] Windows: ติดตั้งผ่าน installer ได้ ไม่มี SmartScreen block ที่ไม่คาดคิด
- [ ] ทดสอบบนเครื่อง clean (ไม่มี dev tools/dependencies ติดตั้งไว้)

## 2. Permissions (macOS)

- [ ] ขอ Microphone permission ถูกต้อง เมื่อกด Start ครั้งแรก
- [ ] ขอ Accessibility permission ถูกต้อง (สำหรับ focus restore + paste)
- [ ] กรณีไม่ได้ให้ Accessibility permission: fallback เป็น button-driven start/stop ทำงานได้ ไม่ crash
- [ ] Global hotkey (hold-to-talk, ADR 010) ทำงานเมื่อให้ permission, ไม่ crash เมื่อไม่ได้ permission

## 3. Core Pipeline Flow

- [ ] กด Start → ไมค์เริ่มจับเสียง (ตรวจสถานะ UI เปลี่ยนเป็น recording)
- [ ] พูดประโยคสั้น → VAD (RMS-VAD) ตัด segment คำพูดได้ถูกต้อง ไม่ตัดขาดกลางคำ
- [ ] เงียบ (silence) ไม่ถูกส่งไป transcribe (ตรวจ log ว่าไม่เรียก Groq API ตอนเงียบ)
- [ ] Segment ถูกส่งไป Groq Whisper (`whisper-large-v3`, language=th) และได้ผลลัพธ์ข้อความ
- [ ] ข้อความถูก paste เข้า input field ของแอปที่ focus อยู่ก่อนหน้า (real-time ต่อ segment)
- [ ] กด Stop → recording หยุด, ไม่มี segment ใหม่ถูกประมวลผลต่อ
- [ ] ทดสอบ hold-to-talk hotkey: hotkey-down เริ่ม, hotkey-up หยุด, `startedAt`/`endedAt` ถูกส่งไป usage-metering ถูกต้อง

## 4. Focus / Paste Behavior

- [ ] สลับไปแอปอื่น (เช่น TextEdit, browser, Slack) แล้วลองพูด → ข้อความ paste เข้าแอปนั้นถูกต้อง ไม่ paste เข้า ontext เอง
- [ ] Focus restore มี settle delay เพียงพอ ไม่ paste ผิดที่เมื่อสลับแอปเร็ว
- [ ] ทดสอบกับแอปที่ไม่มี text field ใด ๆ focus อยู่ → ไม่ crash, จัดการ error อย่างเหมาะสม

## 5. Transcription Quality / Hallucination Filter

- [ ] พูดประโยคปกติภาษาไทย → ได้ข้อความที่อ่านได้ตรงกับที่พูด
- [ ] พูดเสียงรบกวน/ไม่มีคำพูด (noise only) → `IsLikelyHallucination()` กรองออก ไม่ paste ข้อความเพี้ยน
- [ ] ทดสอบ noise filtering ด้วย synthetic noise fixtures (ตาม stage 17) ยังผ่านบน production build

## 6. Network / API Failures

- [ ] ตัดอินเทอร์เน็ตระหว่างใช้งาน → แอป handle error อย่างเหมาะสม (แสดง status, ไม่ crash)
- [ ] Groq API key ไม่ถูกต้อง/หมดอายุ → แสดง error ที่เข้าใจได้, ไม่ crash
- [ ] Groq API ตอบช้า/timeout → UI ไม่ค้าง, recording ยังหยุดได้

## 7. Usage Metering / Billing (ADR 010)

- [ ] `durationMs` ที่ส่งไป usage-metering backend ตรงกับเวลาที่กดค้าง hotkey จริง
- [ ] ทดสอบ session สั้นมาก (< 1 วินาที) และยาว (หลายนาที) ค่า duration ถูกต้อง
- [ ] ตรวจสอบว่า usage data ถูกส่งแม้แอป crash/ปิดกลางคัน (หรือมี retry/queue)

## 8. UI / Step-by-step Transcript Streaming (Stage 15)

- [ ] Partial transcripts สตรีมขึ้น UI แบบ step-by-step ตามลำดับ ไม่สลับลำดับ
- [ ] UI status (idle/recording/processing/error) sync กับ backend events ถูกต้อง

## 9. Resource & Stability

- [ ] รันแอปทิ้งไว้นาน (เช่น 30+ นาที) ไม่มี memory leak (ตรวจ Activity Monitor / Task Manager)
- [ ] ใช้งานหลาย session ต่อเนื่อง (start/stop ซ้ำ ๆ 20+ ครั้ง) ไม่มี goroutine/audio stream รั่ว
- [ ] CPU usage ขณะ idle ต่ำ (ไม่มี busy loop)

## 10. Cross-platform

- [ ] ทดสอบ flow ทั้งหมดข้างต้นซ้ำบน Windows (ถ้า build รองรับ Windows ใน release นี้)
- [ ] ตรวจ keyboard shortcut/hotkey ไม่ชนกับ shortcut ของระบบปฏิบัติการ

## 11. Regression / Final Sign-off

- [ ] รัน automated test suite ทั้งหมด (`go test ./...` ใน `app/ontext-wails`)
- [ ] ตรวจ gate-outs ของทุก stage ที่เกี่ยวข้องว่า status DONE
- [ ] ไม่มี debug/console.log ที่ไม่จำเป็นหลุดไปใน production build
