import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router";
import App from "./App";
import CoursePage from "./CoursePage";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<App />} />
                <Route path="/courses/:courseId" element={<CoursePage />} />
            </Routes>
        </BrowserRouter>
    </React.StrictMode>,
);
