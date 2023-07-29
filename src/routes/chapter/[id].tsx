import { createResource } from "solid-js";
import { useParams } from "solid-start";
import { ID } from "~/util/props";

const fetch_chapter_data = async (id: string): Promise<any> => { }

export default function ChapterId() {
    const { id } = useParams<ID>();
    const [res, { mutate, refetch }] = createResource(id, fetch_chapter_data, {});

    return (
        <main></main>
    )
}
