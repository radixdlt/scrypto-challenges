import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "./index.css";
import Dashboard from "./pages/Dashboard";
import Savings from "./pages/Savings";
import Integration from "./pages/Integration";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
  },
  {
    path: "/dashboard",
    element: <Dashboard />,
  },
  {
    path: "/savings",
    element: <Savings />,
  },
  {
    path: "/integration",
    element: <Integration />,
  },
  {
    path: "*",
    element: (
      <div className="w-full h-screen flex items-center justify-center text-center text-primary-1 text-3xl font-bold">
        {" "}
        Error 404{" "}
      </div>
    ),
  },
]);
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
