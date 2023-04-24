import React from "react";

export interface Note {
  id: string;
  text: string;
  title: string;
}
type Props = {
  notes: Note[];
};

export const NoteList: React.FC<Props> = (props) => {
  return (
    <div>
      {props.notes.map((i) => (
        <>
          <div className="row">
            {i.id} - {i.title}
          </div>
          <div>{i.text}</div>
        </>
      ))}
    </div>
  );
};
