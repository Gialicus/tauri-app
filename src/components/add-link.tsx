import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Note } from "./note-list";
import { BiArrowBack, BiEdit, BiEraser } from "react-icons/bi";

export default function AddLink() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [notes, setNotes] = useState([] as Note[]);
  const [links, setLinks] = useState([] as Note[]);
  const [sel, setSel] = useState("");
  async function addLink() {
    await invoke("add_link", { target: sel, source: id });
  }
  useEffect(() => {
    const load = async () => {
      const notesResponse = await invoke("get_notes");
      const _notes: Note[] = JSON.parse(notesResponse as string);
      setNotes(_notes);
      const linkResponse = await invoke("get_link", { id });
      const _links = JSON.parse(linkResponse as string);
      const ls = JSON.parse(_links as string);
      setLinks(ls);
    };
    load();
  }, []);
  return (
    <div>
      <div className="grid grid-cols-5 gap-4 m-4">
        <div className="col-span-3">
          <select
            className="select select-bordered w-full max-w-xs"
            onChange={(e) => setSel(e.currentTarget.value)}
          >
            <option disabled defaultValue={""}>
              seleziona nota da relazionare
            </option>
            {notes.map((note) => (
              <option key={note.id} value={note.id}>
                {note.title}
              </option>
            ))}
          </select>
        </div>
        <div className="text-end col-span-2">
          <button
            className="btn btn-secondary btn-outline btn-circle mx-2"
            type="button"
            onClick={() => addLink()}
          >
            <BiEdit />
          </button>
          <button
            type="button"
            className="btn btn-info btn-outline btn-circle"
            onClick={() => navigate("../")}
          >
            <BiArrowBack />
          </button>
        </div>
      </div>
      {notes.map((i) => (
        <div className="grid grid-cols-5 gap-4 m-4" key={i.id}>
          <div className="text-secondary text-xl font-bold">{i.title}</div>
          <div className="col-span-3 tooltip" data-tip={i.text}>
            <p className="text-lg truncate text-ellipsis">{i.text}</p>
          </div>
          <div className="text-end">
            <button
              className="btn btn-error btn-outline btn-circle btn-xs "
              type="button"
            >
              <BiEraser />
            </button>
          </div>
        </div>
      ))}
    </div>
  );
}
