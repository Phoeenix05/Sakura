// import { error } from "@sveltejs/kit";

import type { RouteParams } from "./$types";

/** @type {import('./$types').PageLoad} */
export function load({ params }: { params: RouteParams }) {
  return {
    id: params.id
  }
}