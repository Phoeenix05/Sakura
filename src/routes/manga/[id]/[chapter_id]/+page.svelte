<script>
  /** @type {import('./$types').PageData} */
  export let data;
</script>

<nav class="fixed top-0 left-0 bg-[#161616] py-2 px-1 pl-16 w-screen">
  <a href="/">Home</a>
  <a href="/manga/{data.id}">Back</a>
  <p>Manga: {data.id}, Chapter: {data.chapterId}</p>
</nav>

<div>
  <!-- <pre>{JSON.stringify(data.imageData, null, 2)}</pre> -->

  {#each data.imageData as img, i}
    <img class="m-auto" src="{data.baseUrl}/data/{data.chapterHash}/{img}" alt="page {i}">
  {/each}
</div>