<script>
  /** @type {import('./$types').PageData} */
  export let data;
</script>

<div>
  <a href="/">Home</a>
  <a href="/manga/{data.id}">Back</a>
  <p>Manga: {data.id}, Chapter: {data.chapterId}</p>
  <pre>{JSON.stringify(data.chapterImageData, null, 2)}</pre>

  <!-- <img src="{data.chapter_data.baseUrl}/data/{data.chapter_data.chapter.hash}/{data.chapter_data.chapter.data[0]}" alt="page_1"> -->
  <!-- <div class="items-center"> -->
    {#each data.chapterImageData as img, i}
      <img class="m-auto" src="{data.baseUrl}/data/{data.chapterHash}/{img}" alt="page {i}">
    {/each}
  <!-- </div> -->

</div>