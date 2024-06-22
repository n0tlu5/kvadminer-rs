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
    const pageSizeDropdownTop = document.getElementById('page-size-top');
    const pageSizeDropdownBottom = document.getElementById('page-size-bottom');
    pageSizeDropdownTop.value = defaultPageSize;
    pageSizeDropdownBottom.value = defaultPageSize;

    async function fetchKeys(searchQuery = '') {
        const pageSize = parseInt(pageSizeDropdownTop.value) || defaultPageSize;
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

        document.getElementById('current-page-top').value = paginationData.current_page + 1;
        document.getElementById('total-pages-top').textContent = paginationData.total_pages;
        document.getElementById('current-page-bottom').value = paginationData.current_page + 1;
        document.getElementById('total-pages-bottom').textContent = paginationData.total_pages;
        currentPage = paginationData.current_page;
    }

    function updatePage(newPage) {
        if (newPage >= 0) {
            currentPage = newPage;
            fetchKeys(document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value).then(displayKeys);
        }
    }

    document.getElementById('prev-page-top').addEventListener('click', () => {
        updatePage(currentPage - 1);
    });

    document.getElementById('next-page-top').addEventListener('click', () => {
        updatePage(currentPage + 1);
    });

    document.getElementById('prev-page-bottom').addEventListener('click', () => {
        updatePage(currentPage - 1);
    });

    document.getElementById('next-page-bottom').addEventListener('click', () => {
        updatePage(currentPage + 1);
    });

    document.getElementById('current-page-top').addEventListener('change', (event) => {
        const newPage = parseInt(event.target.value) - 1;
        updatePage(newPage);
    });

    document.getElementById('current-page-bottom').addEventListener('change', (event) => {
        const newPage = parseInt(event.target.value) - 1;
        updatePage(newPage);
    });

    pageSizeDropdownTop.addEventListener('change', () => {
        pageSizeDropdownBottom.value = pageSizeDropdownTop.value;
        currentPage = 0;
        fetchKeys(document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value).then(displayKeys);
    });

    pageSizeDropdownBottom.addEventListener('change', () => {
        pageSizeDropdownTop.value = pageSizeDropdownBottom.value;
        currentPage = 0;
        fetchKeys(document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value).then(displayKeys);
    });

    window.deleteKey = async function (key) {
        const queryParams = new URLSearchParams({ host, port, username, password }).toString();
        const response = await fetch(`/delete/${key}?${queryParams}`, { method: 'DELETE' });
        if (response.ok) {
            showAlert('Key deleted successfully', 'success');
            fetchKeys(document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value).then(displayKeys);
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
            fetchKeys(document.getElementById('search-input-top').value || document.getElementById('search-input-bottom').value).then(displayKeys);
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
