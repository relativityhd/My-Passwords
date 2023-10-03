<script>
	import { page } from '$app/stores';
</script>

<h1>{$page.status} Something went wrong :(</h1>
<p>{$page.error?.message}</p>
