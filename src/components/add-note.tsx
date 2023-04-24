import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { BiArrowBack, BiHighlight } from "react-icons/bi";
import { useNavigate } from "react-router-dom";

export default function AddNote() {
  const [title, setTitle] = useState("");
  const [text, setText] = useState("");
  const navigate = useNavigate();
  async function addNote() {
    await invoke("add_note", { title: title, text: text });
  }

  return (
    <form
      className="grid grid-cols-3 gap-4 ml-4"
      onSubmit={(e) => {
        e.preventDefault();
        addNote();
        navigate("../");
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
        <button
          type="submit"
          className="btn btn-primary btn-outline btn-circle mr-2"
        >
          <BiHighlight />
        </button>
        <button
          type="button"
          className="btn btn-info btn-outline btn-circle"
          onClick={() => navigate("../")}
        >
          <BiArrowBack />
        </button>
      </div>
      <div className="col-span-3 w-full h-full">
        <textarea
          className="textarea textarea-primary w-full h-full"
          onChange={(e) => setText(e.currentTarget.value)}
          placeholder="Enter a text..."
        />
      </div>
    </form>
  );
}
