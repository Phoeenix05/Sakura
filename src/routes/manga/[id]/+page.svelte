<script>
  /** @type {import('./$types').PageData} */
  export let data;
</script>

<div>
  <a href="/">Home</a>
  <a href="/chapter/{data.id}">Chapter</a>
  <p>{data.id}</p>
</div>
