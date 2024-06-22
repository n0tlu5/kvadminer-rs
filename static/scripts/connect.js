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

    document.getElementById('connect-form').addEventListener('submit', function (event) {
        event.preventDefault();
        const host = document.getElementById('host').value;
        const port = document.getElementById('port').value;
        const username = document.getElementById('username').value;
        const password = document.getElementById('password').value;

        localStorage.setItem('redis_host', host);
        localStorage.setItem('redis_port', port);
        localStorage.setItem('redis_user', username);
        localStorage.setItem('redis_password', password);

        window.location.href = '/static/db-main.html';
    });
});
