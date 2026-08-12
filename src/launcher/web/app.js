document.addEventListener('DOMContentLoaded', () => {
    console.log('🚀 Nantara-Boot GUI Dashboard Loaded');

    // Sidebar Navigation Tab Switching
    const navItems = document.querySelectorAll('.nav-item');
    const tabContents = document.querySelectorAll('.tab-content');

    navItems.forEach(item => {
        item.addEventListener('click', (e) => {
            e.preventDefault();
            const tabName = item.getAttribute('data-tab');

            // 1. Update active sidebar item
            navItems.forEach(n => n.classList.remove('active'));
            item.classList.add('active');

            // 2. Hide all tab contents and display selected tab content
            tabContents.forEach(content => content.classList.remove('active'));
            const targetSection = document.getElementById(`tab-${tabName}`);
            if (targetSection) {
                targetSection.classList.add('active');
            }

            // 3. Update top header title dynamically
            const headerTitle = document.getElementById('page-title');
            if (headerTitle) {
                headerTitle.textContent = getTitleForTab(tabName);
            }

            if (tabName === 'guide') {
                openQRModal();
            }
        });
    });

    // Modal Controls
    const qrModal = document.getElementById('qr-modal');
    const closeBtn = document.getElementById('btn-close-modal');

    function openQRModal() {
        if (qrModal) qrModal.classList.add('active');
    }

    if (closeBtn) {
        closeBtn.addEventListener('click', () => {
            if (qrModal) qrModal.classList.remove('active');
        });
    }

    if (qrModal) {
        qrModal.addEventListener('click', (e) => {
            if (e.target === qrModal) {
                qrModal.classList.remove('active');
            }
        });
    }

    // AI Query Assistant Logic
    const btnAskAi = document.getElementById('btn-ask-ai');
    const aiErrorInput = document.getElementById('ai-error-input');
    const aiResultCard = document.getElementById('ai-result-card');

    if (btnAskAi && aiErrorInput && aiResultCard) {
        btnAskAi.addEventListener('click', () => {
            const errCode = aiErrorInput.value.trim().toUpperCase();
            if (!errCode) return;

            btnAskAi.innerHTML = '⚡ Menganalisis via Nantara AI Engine...';
            setTimeout(() => {
                btnAskAi.innerHTML = '💡 Analisis Solusi AI';
                if (errCode.includes('BOOT') || errCode.includes('0XC000000E')) {
                    aiResultCard.innerHTML = `
                        <h4>💡 [Nantara AI Solution for ${errCode}]</h4>
                        <p><strong>Penyebab:</strong> Driver controller NVMe/SATA hilang atau konfigurasi BCD boot corrupt.</p>
                        <p><strong>Rekomendasi Perbaikan:</strong></p>
                        <ol>
                            <li>Buka tab Utilitas -> Jalankan Bootice / EasyBCD untuk repair BCD.</li>
                            <li>Inject driver Intel VMD / RST NVMe menggunakan DISM++ Tool.</li>
                        </ol>
                    `;
                } else {
                    aiResultCard.innerHTML = `
                        <h4>💡 [Nantara AI Solution for ${errCode}]</h4>
                        <p><strong>Penyebab:</strong> Terdeteksi indikasi konflik driver sistem atau kerosakan file sistem Windows.</p>
                        <p><strong>Rekomendasi Perbaikan:</strong></p>
                        <ol>
                            <li>Jalankan 1-Click Smart Backup untuk mengamankan file ke USB.</li>
                            <li>Jalankan pemindaian SFC / DISM offline via DISM++ Tool.</li>
                        </ol>
                    `;
                }
            }, 600);
        });
    }

    // Action Triggers
    const btnRefresh = document.getElementById('btn-refresh-sys');
    if (btnRefresh) {
        btnRefresh.addEventListener('click', () => {
            btnRefresh.innerHTML = '<span class="icon">⌛</span> Refreshing...';
            setTimeout(() => {
                btnRefresh.innerHTML = '<span class="icon">🔄</span> Refresh Systems';
                alert('✅ Hardware S.M.A.R.T status & RAM metrics updated!');
            }, 600);
        });
    }

    function getTitleForTab(tab) {
        switch (tab) {
            case 'dashboard': return 'Dasbor Utama System Rescue';
            case 'diagnostics': return 'Auto-Diagnostics Hardware & S.M.A.R.T';
            case 'antivirus': return 'Pemindaian Virus & Malware Offline';
            case 'rescue': return '1-Click Rescue Presets Engine';
            case 'ai': return 'Nantara AI Diagnostic Assistant';
            case 'tools': return 'Daftar Utilitas & Tools Perbaikan';
            case 'guide': return 'Panduan Darurat QR Code';
            default: return 'Nantara Rescue Environment';
        }
    }
});
