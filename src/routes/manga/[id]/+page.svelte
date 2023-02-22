<script>
  /** @type {import('./$types').PageData} */
  export let data;
</script>

<div>
  <a href="/">Home</a>
  <a href="/{data.id}/1">Chapter</a>
  <p>{data.id}</p>
</div>
