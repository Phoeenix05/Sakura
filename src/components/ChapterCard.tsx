import { A } from "solid-start"

export type Props = {
    data: any
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