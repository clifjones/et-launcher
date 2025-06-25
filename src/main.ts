import "./style.css";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

// Define the UserConfig interface
interface UserConfig {
  callsign: string;
  grid: string;
  winlinkPasswd: string;
}

document.addEventListener("DOMContentLoaded", () => {
  const app = document.createElement("div");
  app.id = "app";
  app.innerHTML = `
    <div class="console-container">
      <label for="console-output">Console Output</label>
      <textarea id="console-output" readonly></textarea>
    </div>
    <div id="user-config-dialog" class="dialog" style="display: none;">
      <div class="dialog-content">
        <h2>User Configuration</h2>
        <form id="user-config-form">
          <div class="form-group">
            <label for="dialog-callsign">Callsign:</label>
            <input type="text" id="dialog-callsign" name="callsign" required />
          </div>
          <div class="form-group">
            <label for="dialog-grid">Grid Square:</label>
            <input type="text" id="dialog-grid" name="grid" required />
            <button type="button" id="calculate-grid">Calculate</button>
          </div>
          <div class="form-group">
            <label for="dialog-winlinkPasswd">Winlink Password:</label>
            <input type="password" id="dialog-winlinkPasswd" name="winlinkPasswd" required />
          </div>
          <div class="dialog-buttons">
            <button type="submit">Save</button>
            <button type="button" id="cancel-dialog">Cancel</button>
          </div>
        </form>
      </div>
    </div>
  `;
  
  // Remove existing app element if it exists
  const existingApp = document.getElementById("app");
  if (existingApp && existingApp.parentNode) {
    existingApp.parentNode.removeChild(existingApp);
  }
  
  document.body.appendChild(app);

  // Function to append output to the console textarea
  const appendToConsole = (message: string) => {
    const consoleOutput = document.getElementById(
      "console-output"
    ) as HTMLTextAreaElement;
    if (consoleOutput) {
      consoleOutput.value += message + "\n";
      consoleOutput.scrollTop = consoleOutput.scrollHeight; // Auto-scroll to bottom
    }
  };

  // Console toggle setup
  const consoleContainer = document.querySelector(
    ".console-container"
  ) as HTMLElement;
  if (!consoleContainer) {
    console.warn("No element with class .console-container found");
  }

  // Run app function
  const runApp = (appName: string) => {
    invoke<string>("run_app", { appName })
      .then(msg => appendToConsole(msg))
      .catch(err => appendToConsole(`${appName} Error: ${err}`));
  };

  // Register toggle console listener
  listen<boolean>("toggle-console", (event) => {
    console.log("[toggle-console] payload:", event.payload);
    if (consoleContainer) {
      consoleContainer.style.display = event.payload ? "block" : "none";
    }
  });

  // Listen for run-app events from menu
  listen<string>("run-app", (event) => {
    runApp(event.payload);
  });

  // Listen for app exit events
  listen<string>("app-exited", async ({ payload }) => {
    appendToConsole(`${payload} exited`);
  });

  // Handle user config dialog
  const dialog = document.getElementById("user-config-dialog") as HTMLElement;
  const form = document.getElementById("user-config-form") as HTMLFormElement;
  const cancelButton = document.getElementById("cancel-dialog") as HTMLButtonElement;
  const calculateButton = document.getElementById("calculate-grid") as HTMLButtonElement;

  listen("open-user-config", async () => {
    try {
      const config = await invoke<UserConfig>("read_user_config");
      (document.getElementById("dialog-callsign") as HTMLInputElement).value = config.callsign;
      (document.getElementById("dialog-grid") as HTMLInputElement).value = config.grid;
      (document.getElementById("dialog-winlinkPasswd") as HTMLInputElement).value = config.winlinkPasswd;
      dialog.style.display = "block";
    } catch (error) {
      appendToConsole(`Failed to load user config: ${error}`);
      console.error("Failed to load user config:", error);
    }
  });

  // Calculate grid square
  calculateButton?.addEventListener("click", async () => {
    try {
      const gridSquare = await invoke<string>("get_gridsquare");
      (document.getElementById("dialog-grid") as HTMLInputElement).value = gridSquare;
      appendToConsole(`Grid square calculated: ${gridSquare}`);
    } catch (error) {
      appendToConsole(`Failed to calculate grid square: ${error}`);
      console.error("Failed to calculate grid square:", error);
    }
  });

  // Save config form submission
  form?.addEventListener("submit", async (e) => {
    e.preventDefault();
    const config: UserConfig = {
      callsign: (document.getElementById("dialog-callsign") as HTMLInputElement).value,
      grid: (document.getElementById("dialog-grid") as HTMLInputElement).value,
      winlinkPasswd: (document.getElementById("dialog-winlinkPasswd") as HTMLInputElement).value,
    };
    try {
      await invoke("write_user_config", { config });
      appendToConsole("User config saved successfully");
      dialog.style.display = "none";
    } catch (error) {
      appendToConsole(`Failed to save user config: ${error}`);
      console.error("Failed to save user config:", error);
    }
  });

  // Cancel dialog
  cancelButton?.addEventListener("click", () => {
    dialog.style.display = "none";
  });
});
