<script>
  /** @type {import('./$types').PageData} */
  export let data;
</script>

<div>
  <a href="/">Home</a>
  <!-- <a href="/manga/{data.id}/{data.mangaData.data.attributes.latestUploadedChapter}">Chapter</a> -->
  <p>Manga: {data.id}</p>

  <ul>
    {#each data.mangaData as chapter, i}
      <!-- {#if i > 0} -->
        <li>
          <!-- <p>{chapter.attributes.publisher}</p> -->
          <a href="/manga/{data.id}/{chapter.id}">{chapter.attributes.chapter} {chapter.attributes.title}</a>
        </li>
      <!-- {/if} -->
    {/each}
  </ul>

  <!-- <pre>{JSON.stringify(data.mangaData, null, 2)}</pre> -->
</div>
