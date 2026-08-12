# Panduan Berkontribusi pada Nantara-Boot 🤝

Terima kasih telah berminat untuk berkontribusi pada proyek **Nantara-Boot**! Proyek ini sepenuhnya open-source dan bertujuan membantu komunitas IT serta masyarakat luas dalam menyelamatkan sistem komputer mereka.

---

## 🛠️ Cara Berkontribusi

### 1. Menambahkan Utilitas / Portable App Baru
Jika Anda ingin menyumbang aplikasi perbaikan baru ke dalam Nantara-Boot:
1. Pastikan aplikasi tersebut bersifat **Portable** (bisa berjalan tanpa proses install).
2. Tambahkan script definisi pengunduhan di folder `tools/<kategori>/<nama_app>.ps1`.
3. Daftarkan aplikasi di file `build/config.json`.

### 2. Melaporkan Bug atau Masalah
Jika Anda menemukan bug saat booting, driver yang hilang pada laptop tertentu, atau error pada launcher:
1. Buka halaman **Issues** di GitHub.
2. Gunakan template laporan bug.
3. Cantumkan spesifikasi komputer target (Model Laptop, Jenis SSD NVMe/SATA, Mode UEFI/Legacy).

### 3. Mengembangkan GUI Launcher (`src/`)
Kami menerima kontribusi perbaikan UI/UX pada Nantara Launcher:
- Bahasa utama UI: HTML/CSS/JS (untuk Web/Tauri Launcher) atau C#/WPF.
- Tema: Mengikuti panduan *Modern Dark Theme*.

---

## 📜 Panduan Kode & Commit

- Gunakan format commit yang jelas, contoh:
  - `feat(tools): add CrystalDiskInfo v9.2 script`
  - `fix(launcher): resolve RAM status calculation bug`
  - `docs: update build instruction in README`

---

## ⚖️ Etika & Komunitas

Harap saling menghormati dalam setiap diskusi di *Issues*, *Pull Requests*, maupun forum komunitas. Mari kita bangun alat penyelamat terbaik bersama-sama!
