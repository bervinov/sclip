const { invoke } = window.__TAURI__.core;

let bufferDisplayEl; // Элемент для отображения буфера
let buttonClip;
let buttonClear;

async function sendToClipboard(content) {
  try {
    await invoke("set_buffer", { content: content });
    console.log(`"${content}" sent to clipboard`);
  } catch (error) {
    console.error("Failed to send to clipboard:", error);
  }
}

async function cleanBuffer() {
  try {
    await invoke("clean_buffer"); // Вызываем команду clean_buffer
    console.log("Buffer cleared");
    fetchBuffer(); // Обновляем отображение буфера
  } catch (error) {
    console.error("Failed to clear buffer:", error);
  }
}

async function fetchBuffer() {
  try {
    const buffer = await invoke("get_buffer");

    bufferDisplayEl.innerHTML = "";

    buffer.forEach((item) => {
      if (item.trim() !== "") {
        const itemButton = document.createElement("button");
        itemButton.textContent = item;
        itemButton.classList.add("word-button");
        itemButton.addEventListener("click", () => sendToClipboard(item));
        bufferDisplayEl.appendChild(itemButton);
      }
    });
  } catch (error) {
    console.error("Failed to fetch buffer:", error);
    bufferDisplayEl.textContent = "Error fetching buffer";
  }
}

window.addEventListener("DOMContentLoaded", () => {
  bufferDisplayEl = document.querySelector("#buffer-display");
  buttonClip = document.getElementById("button-clip");
  buttonClear = document.getElementById("button-clear"); // Находим кнопку очистки
  buttonClip.addEventListener("click", fetchBuffer);
  buttonClear.addEventListener("click", cleanBuffer); // Добавляем обработчик для очистки
});