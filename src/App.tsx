import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Note, NoteList } from "./components/note";
import { BiAddToQueue } from "react-icons/bi";

function App() {
  const empty: Note[] = [
    {
      id: "",
      title: "",
      text: "",
    },
  ];
  const [notes, setNotes] = useState(empty);
  const [title, setTitle] = useState("");
  const [text, setText] = useState("");
  const [loader, setLoader] = useState(false);

  async function addNote() {
    await invoke("add_note", { title: title, text: text });
    setLoader(!loader);
  }

  useEffect(() => {
    async function getNotes() {
      let result = await invoke("get_notes");
      const items = JSON.parse(result as string);
      setNotes(items);
    }
    getNotes();
  }, [loader]);

  return (
    <div className="container">
      <h1 className="text-lg text-primary text-center">Welcome to Tauri!</h1>
      <form
        className="grid grid-cols-3 gap-4 ml-4"
        onSubmit={(e) => {
          e.preventDefault();
          addNote();
        }}
      >
        <div className="col-span-2">
          <input
            className="input input-bordered input-primary w-full"
            onChange={(e) => setTitle(e.currentTarget.value)}
            placeholder="Enter a title..."
          />
        </div>
        <div className="text-end">
          <button type="submit" className="btn btn-info">
            <BiAddToQueue />
          </button>
        </div>
        <div className="col-span-2">
          <textarea
            className="textarea textarea-primary w-full h-full"
            onChange={(e) => setText(e.currentTarget.value)}
            placeholder="Enter a text..."
          />
        </div>
        <div>
          <NoteList notes={notes} />
        </div>
      </form>
    </div>
  );
}

export default App;
