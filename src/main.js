const { invoke } = window.__TAURI__.core;

let todoInput;
let todoList;
let newsList;
let workList;
let workProjectInput;
let workHoursInput;

async function refreshNews() {
  const news = await invoke("get_news");
  newsList.innerHTML = "";
  news.forEach((item) => {
    const el = document.createElement("div");
    el.className = "card news-item";
    el.innerHTML = `
      <h3>${item.title}</h3>
      <p>${item.summary}</p>
      <a href="${item.url}" target="_blank" class="news-link">Read More &rarr;</a>
    `;
    newsList.appendChild(el);
  });
}

async function refreshTodos() {
  const todos = await invoke("get_todos");
  todoList.innerHTML = "";
  todos.forEach((todo) => {
    const el = document.createElement("div");
    el.className = `card todo-item ${todo.completed ? "completed" : ""}`;
    el.innerHTML = `
      <span>${todo.text}</span>
      <small>${todo.completed ? "Done" : ""}</small>
    `;
    el.onclick = async () => {
      await invoke("toggle_todo", { id: todo.id });
      refreshTodos();
    };
    todoList.appendChild(el);
  });
}

async function addTodo(e) {
  e.preventDefault();
  if (todoInput.value) {
    await invoke("add_todo", { text: todoInput.value });
    todoInput.value = "";
    refreshTodos();
  }
}

async function refreshWorkLogs() {
  const logs = await invoke("get_work_logs");
  workList.innerHTML = "";
  logs.forEach((log) => {
    const el = document.createElement("div");
    el.className = "card log-item";
    el.innerHTML = `
      <span class="log-project">${log.project}</span>
      <span class="log-details">${log.hours}h (${log.date})</span>
    `;
    workList.appendChild(el);
  });
}

async function addWorkLog(e) {
    e.preventDefault();
    if(workProjectInput.value && workHoursInput.value) {
        await invoke("add_work_log", { 
            project: workProjectInput.value, 
            hours: parseFloat(workHoursInput.value) 
        });
        workProjectInput.value = "";
        workHoursInput.value = "";
        refreshWorkLogs();
    }
}

window.addEventListener("DOMContentLoaded", () => {
  todoInput = document.querySelector("#todo-input");
  todoList = document.querySelector("#todo-list");
  newsList = document.querySelector("#news-list");
  workList = document.querySelector("#work-list");
  workProjectInput = document.querySelector("#work-project");
  workHoursInput = document.querySelector("#work-hours");

  document.querySelector("#todo-form").addEventListener("submit", addTodo);
  document.querySelector("#work-form").addEventListener("submit", addWorkLog);

  refreshNews();
  refreshTodos();
  refreshWorkLogs();
});
