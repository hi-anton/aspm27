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

loadUsers();
