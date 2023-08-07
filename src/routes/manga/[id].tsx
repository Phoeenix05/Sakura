import { createResource } from "solid-js";
import { useParams } from "solid-start";
import { get_manga, get_manga_feed } from "~/util/data";

export default function MangaId() {
    const { id } = useParams<{ id: string }>();
    const [info] = createResource(id, async (id: string) => {
        const data = await get_manga(id)
        return data
    });
    const [feed, { refetch: refetch_feed }] = createResource(id, async (id: string) => {
        const data = await get_manga_feed(id)
        return data
    });

    return (
        <main>
            <button onClick={refetch_feed}>refresh</button>
            {info.loading && <span>Loading...</span>}
            {!info.loading && <pre>{JSON.stringify(info(), null, 2)}</pre>}
            {feed.loading && <span>Loading...</span>}
            {!feed.loading && <pre>{JSON.stringify(feed(), null, 2)}</pre>}
        </main>
    )
}
