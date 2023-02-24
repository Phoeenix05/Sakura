<script>
  /** @type {import('./$types').PageData} */
  export let data;
  // export let sortOrder = "Ascending"

  // export const changeSortOrder = () => {
  //   if (sortOrder === "Ascending") {
  //     sortOrder = "Descending"
  //   } else {
  //     sortOrder = "Ascending"
  //   }
  //   console.log(sortOrder)
  // }
</script>

<div>
  <a href="/">Home</a>
  <p>Manga: {data.id}</p>
  
  <!-- <button on:click={changeSortOrder}>{sortOrder}</button> -->
  <ul>
    {#each data.mangaData as chapter}
      <li class="p-2 hover:bg-[#242424] transition-all duration-75">
        <a href="/manga/{data.id}/{chapter.id}">
          <p class="bold">Ch. {chapter.attributes.chapter} {chapter.attributes.title}</p>
          <p>{chapter.attributes.publishAt}</p>
        </a>
      </li>
    {/each}
  </ul>
</div>
