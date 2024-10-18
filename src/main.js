const { invoke } = window.__TAURI__.core;

let displayedValue;

async function process_num_button(content) {
  displayedValue.textContent = await invoke("process_num_button", {button: content});
}

async function process_operation_button(content) {
  displayedValue.textContent = await invoke("process_operation_button", {button: content});
}

window.addEventListener("DOMContentLoaded", () => {
  displayedValue = document.querySelector(".value");

  const num_buttons = document.querySelectorAll(".number");
  const math_ops_buttons = document.querySelectorAll(".math-operator");
  const opers_buttons = document.querySelectorAll(".operation");

  num_buttons.forEach(button => {
    button.addEventListener("click", function (e) {
      e.preventDefault();
      process_num_button(button.textContent);
      if (this.textContent != "+/-") {
        math_ops_buttons.forEach(operator => operator.classList.remove('active'));
      }
    });
  });

  math_ops_buttons.forEach(button => {
    button.addEventListener("click", function (e) {
      e.preventDefault();
      process_operation_button(button.textContent);
      math_ops_buttons.forEach(operator => operator.classList.remove("active"));
      this.classList.add("active");
    });
  });

  opers_buttons.forEach(button => {
    button.addEventListener("click", function (e) {
      e.preventDefault();
      process_operation_button(button.textContent);
      if (this.textContent == "AC") {
        math_ops_buttons.forEach(operator => operator.classList.remove("active"));
      };
    });
  });
});
