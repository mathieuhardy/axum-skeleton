const MAX_VISIBLE_ROWS_IN_TABLE = 5;
const MAX_VISIBLE_PAGINATION_HANDLES = 5;
const SIDE_PAGINATION_HANDLES = (MAX_VISIBLE_PAGINATION_HANDLES - 1) / 2;

function range(start, end) {
  return Array.from({ length: end - start + 1 }, (_, i) => i)
}

function subset(data, index, max_visible) {
    const start = index * max_visible;
    const end = start + max_visible;

    return data.slice(start, end); 
}

function clearNode(node) {
    while (node.firstChild) {
        node.removeChild(node.lastChild);
    }
}

function fetchJson(url, callback) {
    fetch(url, callback, true);
};

function fetch(url, callback, json = false) {
    let xhr = new XMLHttpRequest();

    xhr.open("GET", url, true);

    if (json) {
        xhr.responseType = "json";
    }

    xhr.onload = function() {
        var status = xhr.status;
      
        if (status === 200) {
            callback(null, xhr.response);
        } else {
            callback(status, xhr.response);
        }
    };

    xhr.send();
};

function createCard(container, id, title) {
    const div = document.createElement("div");
    div.id = id;
    div.classList.add("col");
    div.classList.add("col-sm-12");
    div.classList.add("col-md-12");
    div.classList.add("col-lg-6");
    div.classList.add("col-xl-3");

    const card = document.createElement("div");
    card.classList.add("card");

    const header = document.createElement("h3");
    header.classList.add("card-title");
    header.textContent = title;

    const content = document.createElement("div");
    card.classList.add("card-content");

    const actions = document.createElement("ul");
    actions.classList.add("card-actions");
    actions.classList.add("float-left");

    actions.appendChild(document.createElement("li"));
    card.appendChild(header);
    card.appendChild(content);
    card.appendChild(actions);
    div.appendChild(card);

    container.appendChild(div);
    
    return [
        content,
        actions.querySelector("li")
    ];
}

function createTable(container, columns) {
    const table = document.createElement("table");
    const thead = document.createElement("thead");
    const tbody = document.createElement("tbody");

    thead.appendChild(document.createElement("tr"));
    table.appendChild(thead);
    table.appendChild(tbody);

    container.appendChild(table);

    let head = container.querySelector("table > thead > tr");

    for (const column in columns) {
        const th = document.createElement("th");
        th.textContent = columns[column];

        head.appendChild(th);
    }

    return container.querySelector("table > tbody");
}

function addTableCol(container, value) {
    const td = document.createElement("td");
    td.textContent = value;
    container.appendChild(td); 
}

function addEmptyTableRows(container, count, columns_count) {
    for (let row = 0; row < count; row++) {
        const tr = document.createElement("tr");

        for (let col = 0; col < columns_count; col++) {
            const td = document.createElement("td");
            td.textContent = "\u00a0";
            tr.appendChild(td);
        }

        container.appendChild(tr);
    }
}

function createPagination(container, data_count, current_index, callback) {
    if (data_count <= MAX_VISIBLE_ROWS_IN_TABLE) {
        return;
    }

    const pages = Math.ceil(data_count / MAX_VISIBLE_ROWS_IN_TABLE);

    let start;
    let end;

    if (pages > MAX_VISIBLE_PAGINATION_HANDLES) {
        start = Math.max(current_index - SIDE_PAGINATION_HANDLES, 0);
        end = start + MAX_VISIBLE_PAGINATION_HANDLES;

        if (end > pages) {
            end = pages;
            start = Math.max(end - MAX_VISIBLE_PAGINATION_HANDLES, 0);
        }
    }
    else {
        start = 0;
        end = pages;
    }

    // Get or create pagination object
    let pagination = container.querySelector("ul.pagination");

    if (pagination === null) {
        pagination = document.createElement("ul");
        pagination.classList.add("pagination");

        container.appendChild(pagination);
    }

    // Reset
    clearNode(pagination);

    // Previous button
    const prev = document.createElement("a");
    prev.textContent = "«";

    prev.addEventListener("click", (e) => {
        const active = activePaginationHandle(pagination);
        let index = parseInt(active.getAttribute("data-index"));

        if (index > 0) {
            createPagination(
                container,
                data_count,
                index - 1,
                callback
            );

            callback(index - 1);
        }
        
        e.stopPropagation();
    });

    const li_prev = document.createElement("li");
    li_prev.appendChild(prev);

    pagination.appendChild(li_prev);

    // Dots
    if (pages > MAX_VISIBLE_PAGINATION_HANDLES && start > 0) {
        const dots = document.createElement("a");
        dots.textContent = "…";

        const li = document.createElement("li");
        li.appendChild(dots);

        pagination.appendChild(li);
    }

    // Add entries for each page
    for (let idx = start ; idx < end; idx++) {
        const a = document.createElement("a");
        a.textContent = idx + 1;
        a.setAttribute("data-index", idx);

        if (idx === current_index) {
            a.classList.add("active");
        }

        a.addEventListener("click", (e) => {
            createPagination(
                container,
                data_count,
                idx,
                callback
            );

            callback(idx);
            e.stopPropagation();
        });

        const li = document.createElement("li");
        li.appendChild(a);

        pagination.appendChild(li);
    }

    // Dots
    if (pages > MAX_VISIBLE_PAGINATION_HANDLES && end < pages) {
        const dots = document.createElement("a");
        dots.textContent = "…";

        const li = document.createElement("li");
        li.appendChild(dots);

        pagination.appendChild(li);
    }

    // Next button
    const next = document.createElement("a");
    next.textContent = "»";

    next.addEventListener("click", (e) => {
        const active = activePaginationHandle(pagination);
        let index = parseInt(active.getAttribute("data-index"));

        if (index < (pages - 1)) {
            createPagination(
                container,
                data_count,
                index + 1,
                callback
            );

            callback(index + 1);
        }
        
        e.stopPropagation();
    });

    const li_next = document.createElement("li");
    li_next.appendChild(next);

    pagination.appendChild(li_next);

    return pagination;
}

function setAllGood(container) {
    clearNode(container);

    const p = document.createElement("p");
    p.textContent = "All good";
    p.classList.add("alert");
    p.classList.add("alert-success");

    container.appendChild(p);
}

function activePaginationHandle(container) {
    return container.querySelector("a.active");
}
