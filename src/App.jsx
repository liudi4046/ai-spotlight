import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [commandMsg, setCommandMsg] = useState("");

  const [command, setCommand] = useState("");

  async function sendCommand() {
    try {
      const response = await invoke("handle_command", { command });
      setCommandMsg(response);
    } catch (error) {
      setCommandMsg("Error invoking command: " + error);
    }
  }
  const handleKeyDown = (e) => {
    if (e.key === "Enter") {
      sendCommand();
    }
  };

  return (
    <div className="container">
      <input
        id="command-input"
        onChange={(e) => setCommand(e.currentTarget.value)}
        onKeyDown={handleKeyDown}
        placeholder="Enter a command..."
      />

      <p>{commandMsg}</p>
    </div>
  );
}

export default App;
