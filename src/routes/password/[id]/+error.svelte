<script>
	import { Breadcrumb, BreadcrumbItem } from 'carbon-components-svelte';
	import { page } from '$app/stores';
</script>

<Breadcrumb noTrailingSlash>
	<BreadcrumbItem href="/">Home</BreadcrumbItem>
	<BreadcrumbItem isCurrentPage>Undefined Password</BreadcrumbItem>
</Breadcrumb>

<p>{$page.error?.message}</p>
