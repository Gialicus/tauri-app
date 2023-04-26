import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { BiAddToQueue, BiEdit, BiEraser, BiLink } from "react-icons/bi";
import { useNavigate } from "react-router-dom";

export interface Note {
  id: string;
  text: string;
  title: string;
}
export const NoteList = () => {
  const [notes, setNotes] = useState([] as Note[]);
  const navigate = useNavigate();
  const goToAdd = () => {
    navigate("./note/add", { replace: true });
  };
  const goToEdit = (item: Note) => {
    navigate("./note/edit/" + item.id, { replace: true });
  };
  const goToAddLink = (item: Note) => {
    navigate("./link/add/" + item.id, { replace: true });
  };
  const deleteItem = (item: Note) => {
    const del = async () => {
      await invoke("delete_note", { id: item.id });
    };
    del();
  };
  useEffect(() => {
    const load = async () => {
      const httpResult = await invoke("get_notes");
      const parsedResult: Note[] = JSON.parse(httpResult as string);
      setNotes(parsedResult);
    };
    load();
  }, [notes]);
  return (
    <>
      <div className="mx-4 text-end">
        <button
          type="submit"
          className="btn btn-success btn-circle btn-outline"
          onClick={goToAdd}
        >
          <BiAddToQueue />
        </button>
      </div>
      {notes.map((i) => (
        <div className="grid grid-cols-5 gap-4 m-4" key={i.id}>
          <div className="text-secondary text-xl font-bold">{i.title}</div>
          <div className="col-span-3 tooltip" data-tip={i.text}>
            <p className="text-lg truncate text-ellipsis">{i.text}</p>
          </div>
          <div className="text-end">
            <button
              className="btn btn-secondary btn-outline btn-circle btn-sm"
              onClick={() => goToEdit(i)}
              type="button"
            >
              <BiEdit />
            </button>
            <button
              className="btn btn-info btn-outline btn-circle btn-xs mx-2"
              onClick={() => goToAddLink(i)}
              type="button"
            >
              <BiLink />
            </button>
            <button
              className="btn btn-error btn-outline btn-circle btn-xs "
              onClick={() => deleteItem(i)}
              type="button"
            >
              <BiEraser />
            </button>
          </div>
        </div>
      ))}
    </>
  );
};
