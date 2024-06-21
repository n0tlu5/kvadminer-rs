document.addEventListener('DOMContentLoaded', function () {
    const host = localStorage.getItem('redis_host');
    const port = localStorage.getItem('redis_port');
    const username = localStorage.getItem('redis_user');
    const password = localStorage.getItem('redis_password');

    if (document.getElementById('connect-form')) {
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
    }

    if (document.getElementById('create-form')) {
        document.getElementById('create-form').addEventListener('submit', async function (event) {
            event.preventDefault();
            const newKey = document.getElementById('new-key').value;
            const newValue = document.getElementById('new-value').value;


            const queryParams = new URLSearchParams({ host, port, username, password }).toString();
            await fetch(`/set?${queryParams}`, {

                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ key: newKey, value: newValue })
            });

            document.getElementById('new-key').value = '';
            document.getElementById('new-value').value = '';
            fetchKeys().then(displayKeys);
        });
    }

    if (document.getElementById('keys-list')) {
        let currentPage = 0;
        const pageSize = 10;

        async function fetchKeys() {
            const queryParams = new URLSearchParams({ host, port, username, password }).toString();
            const response = await fetch(`/keys?${queryParams}`);

            const keys = await response.json();
            return keys;
        }

        function displayKeys(keys) {
            const keysList = document.getElementById('keys-list');
            keysList.innerHTML = '';

            const start = currentPage * pageSize;
            const end = start + pageSize;
            const pagedKeys = keys.slice(start, end);

            pagedKeys.forEach(key => {
                const li = document.createElement('li');
                li.innerHTML = `<a href="/static/key-edit.html?key=${encodeURIComponent(key)}">${key}</a> <button onclick="deleteKey('${key}')">Delete</button>`;
                keysList.appendChild(li);
            });
        }

        document.getElementById('prev-page').addEventListener('click', () => {
            if (currentPage > 0) {
                currentPage--;

                fetchKeys().then(displayKeys);
            }

        });


        document.getElementById('next-page').addEventListener('click', () => {

            currentPage++;

            fetchKeys().then(displayKeys);
        });

        async function deleteKey(key) {
            const queryParams = new URLSearchParams({ host, port, username, password }).toString();
            await fetch(`/delete/${key}?${queryParams}`, { method: 'DELETE' });
            fetchKeys().then(displayKeys);
        }

        fetchKeys().then(displayKeys);
    }

    if (document.getElementById('edit-form')) {
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
                    alert('Key saved successfully');
                } else {
                    alert('Failed to save key');
                }
            });
        });
    }
});

