import { useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { TauriCommand } from "./app/commands/commands";


function App() {


  const [message, setMessage] = useState("");
  return (
    <main className="container">
      <input type="text" value={message} onChange={(e) => setMessage(e.target.value)} />
      <h1>MPOKER</h1>
      <button onClick={()=> invoke(TauriCommand.RUST_TO_REACT, { message })}>Calling rust_to_react</button>
    </main>
  );
}

export default App;
