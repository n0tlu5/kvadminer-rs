document.addEventListener('DOMContentLoaded', function () {
    const infoBar = document.createElement('div');
    infoBar.className = 'info-bar';
    document.body.prepend(infoBar);

    const host = localStorage.getItem('redis_host');
    const port = localStorage.getItem('redis_port');
    const username = localStorage.getItem('redis_user');
    const password = localStorage.getItem('redis_password');

    if (host && port) {
        infoBar.textContent = `Connected to ${host}:${port} as ${username ? username : 'anonymous'}`;
    } else {
        infoBar.textContent = `Not connected`;
    }

    function showAlert(message, type = 'error') {
        const alertDiv = document.createElement('div');
        alertDiv.className = `alert ${type}`;
        alertDiv.innerHTML = `${message}<span class="closebtn" onclick="this.parentElement.style.display='none';">&times;</span>`;
        document.body.prepend(alertDiv);
        setTimeout(() => alertDiv.style.display = 'none', 3000);
    }

    const params = new URLSearchParams(window.location.search);
    const key = params.get('key');

    document.getElementById('key').value = key;

    fetch(`/get/${key}?host=${host}&port=${port}&username=${username}&password=${password}`)
        .then(response => response.text())
        .then(value => {
            document.getElementById('value').value = value;
        });

    document.getElementById('edit-form').addEventListener('submit', function (event) {
        event.preventDefault();
        const value = document.getElementById('value').value;

        fetch(`/set?host=${host}&port=${port}&username=${username}&password=${password}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ key, value })
        })
        .then(response => {
            if (response.ok) {
                showAlert('Key saved successfully', 'success');
            } else {
                showAlert('Failed to save key');
            }
        });
    });
});
