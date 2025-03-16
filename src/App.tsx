import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Course = {
    courseCode: string;
    sectionId: string;
    sectionName: string;
};

function App() {
    const [courses, setCourses] = useState<Course[]>([]);
    useEffect(() => {
        const res: Promise<Course[]> = invoke("get_courses", {});
        res.then(data => {
            setCourses(data);
        })
    }, []);

    return (
        <main className="container">
            <h1>Select a Course</h1>

            <ul>
            {courses.map((res) => (<li>{res.courseCode}</li>))}
            </ul>
        </main>
    );
}

export default App;
