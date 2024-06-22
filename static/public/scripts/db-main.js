document.addEventListener('DOMContentLoaded', function () {
    const infoBar = document.getElementById('info-bar');

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

    let currentPage = 0;
    const defaultPageSize = 10;
    const pageSizeDropdowns = document.querySelectorAll('[id^="page-size"]');
    pageSizeDropdowns.forEach(dropdown => dropdown.value = defaultPageSize);

    async function fetchKeys(searchQuery = '') {
        const pageSize = parseInt(pageSizeDropdowns[0].value) || defaultPageSize;
        const queryParams = new URLSearchParams({
            host,
            port,
            username,
            password,
            page: currentPage,
            page_size: pageSize,
            search: searchQuery
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

        const currentPageInputs = document.querySelectorAll('[id^="current-page"]');
        const totalPagesSpans = document.querySelectorAll('[id^="total-pages"]');

        currentPageInputs.forEach(input => input.value = paginationData.current_page + 1);
        totalPagesSpans.forEach(span => span.textContent = paginationData.total_pages);
        currentPage = paginationData.current_page;
    }

    function updatePage(newPage) {
        if (newPage >= 0) {
            currentPage = newPage;
            const searchQuery = document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value;
            fetchKeys(searchQuery).then(displayKeys);
        }
    }

    document.querySelectorAll('[id^="prev-page"]').forEach(button => {
        button.addEventListener('click', () => updatePage(currentPage - 1));
    });

    document.querySelectorAll('[id^="next-page"]').forEach(button => {
        button.addEventListener('click', () => updatePage(currentPage + 1));
    });

    document.querySelectorAll('[id^="current-page"]').forEach(input => {
        input.addEventListener('change', (event) => {
            const newPage = parseInt(event.target.value) - 1;
            updatePage(newPage);
        });
    });

    pageSizeDropdowns.forEach(dropdown => {
        dropdown.addEventListener('change', () => {
            pageSizeDropdowns.forEach(d => d.value = dropdown.value);
            currentPage = 0;
            const searchQuery = document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value;
            fetchKeys(searchQuery).then(displayKeys);
        });
    });

    window.deleteKey = async function (key) {
        const queryParams = new URLSearchParams({ host, port, username, password }).toString();
        const response = await fetch(`/delete/${key}?${queryParams}`, { method: 'DELETE' });
        if (response.ok) {
            showAlert('Key deleted successfully', 'success');
            const searchQuery = document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value;
            fetchKeys(searchQuery).then(displayKeys);
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
            const searchQuery = document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value;
            fetchKeys(searchQuery).then(displayKeys);
        } else {
            const errorMessage = await response.text();
            showAlert(`Failed to create key: ${errorMessage}`);
        }
    });

    document.getElementById('search-input-top').addEventListener('input', (event) => {
        currentPage = 0;
        fetchKeys(event.target.value).then(displayKeys);
    });

    document.getElementById('search-input-bottom').addEventListener('input', (event) => {
        currentPage = 0;
        fetchKeys(event.target.value).then(displayKeys);
    });

    fetchKeys().then(displayKeys);
});
