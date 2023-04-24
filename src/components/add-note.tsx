import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { BiArrowBack, BiHighlight } from "react-icons/bi";
import { useNavigate, useParams } from "react-router-dom";
import { Note } from "./note-list";

export default function AddNote() {
  const [title, setTitle] = useState("");
  const [text, setText] = useState("");
  const navigate = useNavigate();
  let { id } = useParams();
  if (id) {
    useEffect(() => {
      const load = async () => {
        const noteResponse = await invoke("get_note", { id });
        const note: Note = JSON.parse(noteResponse as string);
        setTitle(note.title);
        setText(note.text);
      };
      load();
    }, []);
  }
  async function addNote() {
    await invoke("add_note", { title: title, text: text });
  }
  async function editNote() {
    await invoke("update_note", { id, title, text });
  }

  return (
    <form
      className="grid grid-cols-3 gap-4 ml-4"
      onSubmit={(e) => {
        e.preventDefault();
        if (id) {
          editNote();
        } else {
          addNote();
        }
        navigate("../");
      }}
    >
      <div className="col-span-2">
        <div className="form-control">
          <label className="input-group input-group-vertical">
            <span>Title</span>
            <input
              className="input input-bordered input-sm w-full"
              onChange={(e) => setTitle(e.currentTarget.value)}
              placeholder="Enter a title..."
              value={title}
            />
          </label>
        </div>
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
      <div className="col-span-3 w-full h-[420px]">
        <textarea
          className="textarea textarea-secondary w-full h-full"
          onChange={(e) => setText(e.currentTarget.value)}
          placeholder="Enter a text..."
          value={text}
        />
      </div>
    </form>
  );
}
