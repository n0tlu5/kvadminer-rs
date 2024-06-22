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
        infoBar.textContent = `Connected to ${host}:${port}`;
    }

    function showAlert(message, type = 'error') {
        const alertDiv = document.createElement('div');
        alertDiv.className = `alert ${type}`;
        alertDiv.innerHTML = `${message}<span class="closebtn" onclick="this.parentElement.style.display='none';">&times;</span>`;
        document.body.prepend(alertDiv);
        setTimeout(() => alertDiv.style.display = 'none', 3000);
    }

    let currentPage = 0;
    const pageSize = 10;

    async function fetchKeys() {
        const queryParams = new URLSearchParams({
            host,
            port,
            username,
            password,
            page: currentPage,
            page_size: pageSize
        }).toString();
        const response = await fetch(`/keys?${queryParams}`);
        if (!response.ok) {
            showAlert('Failed to fetch keys');
            return { keys: [], current_page: 0, total_pages: 0, total_keys: 0 };
        }
        return await response.json();
    }

    function displayKeys(paginationData) {
        const keysTable = document.getElementById('keys-table-body');
        keysTable.innerHTML = '';

        paginationData.keys.forEach(([key, value]) => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td class="truncated" title="${key}">${key}</td>
                <td class="truncated" title="${value}">${value}</td>
                <td>
                    <button onclick="editKey('${key}')">Edit</button>
                    <button onclick="deleteKey('${key}')">Delete</button>
                </td>`;
            keysTable.appendChild(row);
        });

        document.getElementById('pagination-info').textContent = `Page ${paginationData.current_page + 1} of ${paginationData.total_pages}`;
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

    window.deleteKey = async function (key) {
        const queryParams = new URLSearchParams({ host, port, username, password }).toString();
        const response = await fetch(`/delete/${key}?${queryParams}`, { method: 'DELETE' });
        if (response.ok) {
            showAlert('Key deleted successfully', 'success');
            fetchKeys().then(displayKeys);
        } else {
            showAlert('Failed to delete key');
        }
    };

    window.editKey = function (key) {
        const params = new URLSearchParams({ host, port, username, password });
        window.location.href = `/static/key-edit.html?key=${encodeURIComponent(key)}&${params.toString()}`;
    };

    document.getElementById('create-form').addEventListener('submit', async function (event) {
        event.preventDefault();
        const newKey = document.getElementById('new-key').value;
        const newValue = document.getElementById('new-value').value;

        const queryParams = new URLSearchParams({ host, port, username, password }).toString();
        const response = await fetch(`/set?${queryParams}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ key: newKey, value: newValue })
        });

        if (response.ok) {
            showAlert('Key created successfully', 'success');
            document.getElementById('new-key').value = '';
            document.getElementById('new-value').value = '';
            fetchKeys().then(displayKeys);
        } else {
            const errorMessage = await response.text();
            showAlert(`Failed to create key: ${errorMessage}`);
        }
    });

    fetchKeys().then(displayKeys);
});
