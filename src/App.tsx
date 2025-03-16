import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Card, CardDescription, CardHeader, CardTitle } from "./components/ui/card";
import { Link } from "react-router";

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
        <div className="h-[100vh] w-[100vw] bg-zinc-200">
        <main className="container">
        <h1 className="text-4xl font-extrabold tracking-tight lg:text-4xl mb-[3vh]">Select a Course</h1>

        <div className="grid sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 mx-auto gap-4">
            {courses.map((course) => (
                <Link to={{pathname: "/courses/" + course.sectionId}}>
                <Card className="w-[300px] shadow-md rounded-lg overflow-hidden hover:shadow-xl duration-200 ease-in-out transition-all">
                <CardHeader>
                <CardTitle>{course.courseCode}</CardTitle>
                <CardDescription>{course.sectionName}</CardDescription>
                </CardHeader>
                </Card>
                </Link>
            ))}
            </div>
        </main>
        </div>
    );
}

export default App;
