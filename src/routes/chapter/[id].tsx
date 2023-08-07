import { useParams } from "solid-start";

const fetch_chapter_data = async (id: string): Promise<any> => { }

export default function ChapterId() {
    const { id } = useParams<{ id: string }>();

    return (
        <main>
            Being implemented ({id})
        </main>
    )
}
