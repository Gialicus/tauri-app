import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const empty = [
    {
      id: null,
      title: null,
      text: null,
    },
  ];
  const [greetMsg, setGreetMsg] = useState(empty);
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let result = await invoke("greet", { name });
    const items = JSON.parse(result as string);
    setGreetMsg(items);
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="submit">Greet</button>
        </form>
      </div>
      <div>
        {greetMsg.map((i) => (
          <>
            <div className="row">
              {i.id} - {i.title}
            </div>
            <div>{i.text}</div>
          </>
        ))}
      </div>
    </div>
  );
}

export default App;
