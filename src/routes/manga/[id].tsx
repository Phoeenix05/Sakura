import { createResource } from "solid-js";
import { useParams } from "solid-start";
import ChapterCard from "~/components/ChapterCard";
import { ApiObject } from "~/util/bindings/ApiObject";
import { ChapterAttributes } from "~/util/bindings/ChapterAttributes";
import { MangaAttributes } from "~/util/bindings/MangaAttributes";
import { ID } from "~/util/props";
import { fetch_resource } from "~/util/resources";

export default function MangaId() {
    const { id } = useParams<ID>();
    const [info] = createResource(
        () => id,
        async (id: string) =>
            await fetch_resource<ApiObject<MangaAttributes>>(`https://api.mangadex.org/manga/${id}`)
    );
    const [feed] = createResource(
        () => id,
        async (id: string) =>
            await fetch_resource<ApiObject<ChapterAttributes>[]>(`https://api.mangadex.org/manga/${id}/feed?limit=500&translatedLanguage[]=en`)
    );

    return (
        <main>
            {/* {info.loading ? "Loading (Manga info)..." : <pre>{JSON.stringify(info()?.data.attributes.title, null, 2)}</pre>} */}
            {/* {feed.loading ? "Loading (Manga feed)..." : <pre>{JSON.stringify(feed()?.data.map(e => e.id), null, 2)}</pre>} */}
            {
                feed.loading ? "Loading (Manga feed)..." : feed()?.data.map(e => <ChapterCard data={e} />)
            }
        </main>
    )
}
