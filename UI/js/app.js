async function loadUsers() {
  try {
    console.log("Start");

    const response = await fetch("http://127.0.0.1:3000/users");

    console.log("Status:", response.status);

    const users = await response.json();

    console.log("Users:", users);

    const container = document.querySelector(".contacts");

    users.forEach((user) => {
      const div = document.createElement("div");

      div.className = "contact";

      div.innerHTML = `
                <div class="contact-info">

                    <p class="contact-heading">
                        ${user.username}
                    </p>

                    <p>
                        ID: ${user.id}
                    </p>

                </div>
            `;

      container.appendChild(div);
    });
  } catch (error) {
    console.error("Fetch Fehler:", error);
  }
}

const messageInput = document.querySelector(".message-text-bar");
const sendButton = document.querySelector(".send-button");
const chatArea = document.querySelector(".chat-area");

function displayMessage() {
  const text = messageInput.value.trim();

  if (!text) {
    return;
  }

  const messageWrapper = document.createElement("div");
  messageWrapper.className = "message-me";
  messageWrapper.innerHTML = `
    <p class="sender">Du</p>
    <div class="message-cont">
      <p class="message"></p>
    </div>
  `;

  messageWrapper.querySelector(".message").textContent = text;
  chatArea.appendChild(messageWrapper);
  messageInput.value = "";
  chatArea.scrollTop = chatArea.scrollHeight;
}

sendButton.addEventListener("click", displayMessage);

messageInput.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    event.preventDefault();
    displayMessage();
  }
});
displayMessage();
loadUsers();
