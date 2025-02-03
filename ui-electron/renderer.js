// renderer.js

// Toggle theme logic remains the same.
const toggleThemeBtn = document.getElementById("toggleTheme");
toggleThemeBtn.addEventListener("click", () => {
  const body = document.body;
  if (body.classList.contains("dark")) {
    body.classList.replace("dark", "light");
  } else {
    body.classList.replace("light", "dark");
  }
});

// Function to install an app.
function installApp(appId, buttonElem) {
  fetch("http://127.0.0.1:8080/install", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "X-App-ID": "my-app-id", // Use your test app id.
    },
    body: JSON.stringify({ app_id: appId }),
  })
    .then((response) => {
      if (response.ok) {
        buttonElem.textContent = "Installed";
        buttonElem.disabled = true;
      } else {
        buttonElem.textContent = "Install Failed";
      }
    })
    .catch((error) => {
      console.error("Error installing app:", error);
      buttonElem.textContent = "Install Failed";
    });
}

// Fetch and display registered apps from the backend API.
function loadApps() {
  fetch("http://127.0.0.1:8080/apps", {
    headers: {
      "X-App-ID": "my-app-id", // Use your test app id.
    },
  })
    .then((response) => response.json())
    .then((apps) => {
      const appList = document.getElementById("appList");
      appList.innerHTML = "";
      apps.forEach((app) => {
        const container = document.createElement("div");
        container.className = "app-item";

        const title = document.createElement("span");
        title.textContent = `${app.app_name} (${app.app_id})`;
        container.appendChild(title);

        const installBtn = document.createElement("button");
        installBtn.textContent = "Install";
        installBtn.onclick = () => installApp(app.app_id, installBtn);
        container.appendChild(installBtn);

        appList.appendChild(container);
      });
    })
    .catch((error) => {
      console.error("Error fetching apps:", error);
      document.getElementById("appList").textContent = "Failed to load apps.";
    });
}

loadApps();
