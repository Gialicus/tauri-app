import AddLink from "./components/add-link";
import AddNote from "./components/add-note";
import { NoteList } from "./components/note-list";
import { RouterProvider, createBrowserRouter } from "react-router-dom";

function App() {
  const router = createBrowserRouter([
    {
      path: "/",
      element: <NoteList />,
    },
    {
      path: "note/add",
      element: <AddNote></AddNote>,
    },
    {
      path: "note/edit/:id",
      element: <AddNote></AddNote>,
    },
    {
      path: "link/add/:id",
      element: <AddLink></AddLink>,
    },
  ]);
  return (
    <div className="container h-100 w-full">
      <h1 className="text-4xl text-primary text-center font-extrabold my-2">
        Gialicus Notes
      </h1>
      <RouterProvider router={router}></RouterProvider>
    </div>
  );
}

export default App;
