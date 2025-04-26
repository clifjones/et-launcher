import "./style.css";
import { invoke } from "@tauri-apps/api";

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
    <div class="config-container">
      <h2>User Configuration</h2>
      <form id="user-config-form">
        <div class="form-group">
          <label for="callsign">Callsign:</label>
          <input type="text" id="callsign" name="callsign" required />
        </div>
        <div class="form-group">
          <label for="grid">Grid Square:</label>
          <input type="text" id="grid" name="grid" required />
        </div>
        <div class="form-group">
          <label for="winlinkPasswd">Winlink Password:</label>
          <input type="password" id="winlinkPasswd" name="winlinkPasswd" required />
        </div>
        <button type="submit" id="save-config-btn">Save</button>
      </form>
    </div>
    
    <div class="mode-container">
      <h2>Mode</h2>
      <div class="form-group">
        <label for="radio-display">Radio:</label>
        <input type="text" id="radio-display" readonly />
      </div>
      <div class="form-group">
        <label for="mode-display">Mode:</label>
        <input type="text" id="mode-display" readonly />
      </div>
    </div>
    
    <div class="button-container">
      <button id="app1-btn">App 1</button>
      <button id="app2-btn">App 2</button>
    </div>
    
    <div class="console-container">
      <label for="console-output">Console Output</label>
      <textarea id="console-output" readonly></textarea>
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

  // Load user config
  const loadUserConfig = async () => {
    try {
      const config = await invoke<UserConfig>("read_user_config");
      (document.getElementById("callsign") as HTMLInputElement).value = config.callsign;
      (document.getElementById("grid") as HTMLInputElement).value = config.grid;
      (document.getElementById("winlinkPasswd") as HTMLInputElement).value = config.winlinkPasswd;
      appendToConsole("User config loaded successfully");
    } catch (error) {
      appendToConsole(`Failed to load user config: ${error}`);
      console.error("Failed to load user config:", error);
    }
  };

  // Load mode
  const loadMode = async () => {
    try {
      const mode = await invoke<string>("read_et_mode");
      (document.getElementById("mode-display") as HTMLInputElement).value = mode;
      appendToConsole("Mode loaded successfully");
    } catch (error) {
      appendToConsole(`Failed to load mode: ${error}`);
      console.error("Failed to load mode:", error);
    }
  };

  // Load active radio
  const loadActiveRadio = async () => {
    try {
      const radio = await invoke<string>("read_active_radio");
      (document.getElementById("radio-display") as HTMLInputElement).value = radio;
      appendToConsole("Active radio loaded successfully");
    } catch (error) {
      appendToConsole(`Failed to load active radio: ${error}`);
      console.error("Failed to load active radio:", error);
    }
  };

  // Initialize config, mode, and radio
  loadUserConfig();
  loadMode();
  loadActiveRadio();

  // Save config form submission
  document.getElementById("user-config-form")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    const config: UserConfig = {
      callsign: (document.getElementById("callsign") as HTMLInputElement).value,
      grid: (document.getElementById("grid") as HTMLInputElement).value,
      winlinkPasswd: (document.getElementById("winlinkPasswd") as HTMLInputElement).value,
    };
    try {
      await invoke("write_user_config", { config });
      appendToConsole("User config saved successfully");
    } catch (error) {
      appendToConsole(`Failed to save user config: ${error}`);
      console.error("Failed to save user config:", error);
    }
  });

  // Attach event listeners to app buttons
  document.getElementById("app1-btn")?.addEventListener("click", async () => {
    try {
      const result = await invoke("run_app", { appName: "emacs" });
      appendToConsole(`App 1: ${result}`);
      console.log("App 1 result:", result);
    } catch (error) {
      appendToConsole(`App 1 Error: ${error}`);
      console.error("Failed to run App 1:", error);
    }
  });

  document.getElementById("app2-btn")?.addEventListener("click", async () => {
    try {
      const result = await invoke("run_app", { appName: "date" });
      appendToConsole(`App 2: ${result}`);
      console.log("App 2 result:", result);
    } catch (error) {
      appendToConsole(`App 2 Error: ${error}`);
      console.error("Failed to run App 2:", error);
    }
  });
});
