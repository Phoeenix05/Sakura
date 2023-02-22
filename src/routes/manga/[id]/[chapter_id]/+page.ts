// import { error } from "@sveltejs/kit";

import type { RouteParams } from "./$types";

/** @type {import('./$types').PageLoad} */
export function load({ params }: { params: RouteParams }) {
  console.log(params)
  
  return {
    id: params.id,
    chapter_id: params.chapter_id
  }
}