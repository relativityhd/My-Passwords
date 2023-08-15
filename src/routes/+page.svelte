<script lang="ts">
	import { Grid, Row, Column } from 'carbon-components-svelte';
	import Create from '$lib/Create.svelte';
	import Buckets from '$lib/Buckets.svelte';
	import Accounts from '$lib/Accounts.svelte';

	export let data;
</script>

<Grid>
	<Row>
		<Column sm={4}>
			<h1>Welcome to My-Password</h1>
		</Column>
		<Column sm={4}>
			<Create />
			<a href="/password/1">Password 1</a>
		</Column>
		<Column sm={4}>
			<Buckets buckets={data.buckets} />
		</Column>
		<Column sm={4}>
			<Accounts accounts={data.accounts} />
		</Column>
	</Row>
</Grid>
