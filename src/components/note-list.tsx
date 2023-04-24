import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { BiAddToQueue, BiEdit, BiEraser } from "react-icons/bi";
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
    navigate("./add", { replace: true });
  };
  const editItem = (item: Note) => {
    console.log(item);
  };
  const deleteItem = (item: Note) => {
    console.log(item);
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
          <div className="text-lg col-span-3 truncate text-ellipsis">
            {i.text}
          </div>
          <div className="text-end">
            <button
              className="btn btn-secondary btn-outline btn-circle mx-2"
              onClick={() => editItem(i)}
              type="button"
            >
              <BiEdit />
            </button>
            <button
              className="btn btn-error btn-outline btn-circle"
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
