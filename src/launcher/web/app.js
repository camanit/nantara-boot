document.addEventListener('DOMContentLoaded', () => {
    console.log('🚀 Nantara-Boot GUI Dashboard Loaded');

    // Navigation Tab Switching
    const navItems = document.querySelectorAll('.nav-item');
    navItems.forEach(item => {
        item.addEventListener('click', (e) => {
            e.preventDefault();
            navItems.forEach(n => n.classList.remove('active'));
            item.classList.add('active');

            const tabName = item.getAttribute('data-tab');
            document.getElementById('page-title').textContent = getTitleForTab(tabName);

            if (tabName === 'guide') {
                openQRModal();
            }
        });
    });

    // Modal Controls
    const qrModal = document.getElementById('qr-modal');
    const closeBtn = document.getElementById('btn-close-modal');

    function openQRModal() {
        qrModal.classList.add('active');
    }

    closeBtn.addEventListener('click', () => {
        qrModal.classList.remove('active');
    });

    qrModal.addEventListener('click', (e) => {
        if (e.target === qrModal) {
            qrModal.classList.remove('active');
        }
    });

    // 1-Click Action Card Event Triggers
    document.getElementById('card-1click-backup').addEventListener('click', () => {
        alert('⚡ Executing 1-Click Smart Backup...\nScanning C:\\Users\\ for Desktop, Documents, Downloads, & Pictures...');
    });

    document.getElementById('card-1click-sam').addEventListener('click', () => {
        alert('🔑 1-Click SAM Password Reset Engine:\nScanning C:\\Windows\\System32\\config\\SAM...\nLocal account passwords cleared successfully!');
    });

    document.getElementById('card-antivirus-scan').addEventListener('click', () => {
        alert('🛡️ Offline Antivirus Scan Engine:\nMounting drive C: in isolated mode...\nExecuting ClamAV / KVRT offline virus scan...');
    });

    document.getElementById('btn-refresh-sys').addEventListener('click', () => {
        const btn = document.getElementById('btn-refresh-sys');
        btn.innerHTML = '<span class="icon">⌛</span> Refreshing...';
        setTimeout(() => {
            btn.innerHTML = '<span class="icon">🔄</span> Refresh Systems';
            alert('✅ Hardware S.M.A.R.T status & RAM metrics updated!');
        }, 800);
    });

    function getTitleForTab(tab) {
        switch (tab) {
            case 'dashboard': return 'Dasbor Utama System Rescue';
            case 'diagnostics': return 'Auto-Diagnostics Hardware & S.M.A.R.T';
            case 'antivirus': return 'Pemindaian Virus & Malware Offline';
            case 'rescue': return '1-Click Rescue Presets Engine';
            case 'tools': return 'Daftar Utilitas & Tools Perbaikan';
            case 'guide': return 'Panduan Darurat QR Code';
            default: return 'Nantara Rescue Environment';
        }
    }
});
