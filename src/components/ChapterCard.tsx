import { A } from "solid-start"
import { ApiObject } from "~/util/bindings/ApiObject"
import { ChapterAttributes } from "~/util/bindings/ChapterAttributes"

export type Props = {
    data: ApiObject<ChapterAttributes>
}

export default function ChapterCard({ data }: Props) {
    return (
        <>
            <A href={`/chapter/${data.id}`}>
                Ch. {data.attributes.chapter} - {data.attributes.title}
            </A>
            <br />
        </>
    )
}