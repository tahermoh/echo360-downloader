import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Button } from "./components/ui/button";
import { Link, useParams } from "react-router";
import { ArrowLeft, ChevronLeft } from "lucide-react";

type Video = {

};

export default function CoursePage() {
    const params = useParams();
    const [videos, setVideos] = useState<Video[]>([]);
    useEffect(() => {
        const res: Promise<Video[]> = invoke("get_videos", {id: params.courseId});
        res.then(data => {
            setVideos(data);
        })
    }, []);

    return (
        <div className="">
        <Button variant="ghost" size="lg" className="py-5 px-10 mx-1 my-2" asChild ><a href="/"><ArrowLeft className="scale-200" /> </a></Button>
        </div>
    )
}


