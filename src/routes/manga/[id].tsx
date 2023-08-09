import { createResource } from "solid-js";
import { useParams } from "solid-start";
import { get_manga, get_manga_feed } from "~/util/data";

export default function MangaId() {
    const { id } = useParams<{ id: string }>();
    const [info] = createResource(id, get_manga);
    // Validation for manga feed is disabled for now.
    const [feed, { refetch: refetch_feed }] = createResource(id,
        async (id: string) => await get_manga_feed(id, false)
    );


    return (
        <main>
            <button onClick={refetch_feed}>refresh</button>
            <br />
            {info.loading && <span>Loading manga info...</span>}
            {!info.loading && <pre>{JSON.stringify(info(), null, 2)}</pre>}
            <br />
            {feed.loading && <span>Loading manga feed...</span>}
            {!feed.loading && <pre>{JSON.stringify(feed(), null, 2)}</pre>}
        </main>
    )
}
