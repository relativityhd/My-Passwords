<script lang="ts">
	import { Grid, Row, Column } from 'carbon-components-svelte';
	import { Button, Modal } from 'carbon-components-svelte';
	import { FluidForm, TextInput, PasswordInput } from 'carbon-components-svelte';
	import { Select, SelectItem } from 'carbon-components-svelte';
	import Add from 'carbon-icons-svelte/lib/Send.svelte';
	import { Industry } from '$lib/types';
	import {
		StructuredList,
		StructuredListHead,
		StructuredListRow,
		StructuredListCell,
		StructuredListBody
	} from 'carbon-components-svelte';

	import type { RetrievedBucket } from '$lib/bindings';

	let colors = ['red', 'blue', 'green', 'yellow', 'purple', 'cyan', 'magenta', 'orange'];
	let bucketName = '';
	let color = colors[Math.floor(Math.random() * colors.length)];

	export let buckets: RetrievedBucket[] = [];
</script>

<h3>Buckets</h3>
<StructuredList condensed>
	<StructuredListBody>
		{#each buckets as bucket}
			<StructuredListRow>
				<StructuredListCell>
					<div class="color-knop" style="background: {bucket.color};" />
					{bucket.name} (0 Passwords -> TODO)
				</StructuredListCell>
			</StructuredListRow>
		{/each}
	</StructuredListBody>
</StructuredList>
<p>Create a new Bucket</p>
<FluidForm>
	<TextInput bind:value={bucketName} light size="sm" hideLabel placeholder="e.g. Private" />
</FluidForm>
<Select bind:selected={color} light size="sm" labelText="Color">
	{#each colors as color}
		<SelectItem value={color} />
	{/each}
</Select>
<Button size="small" icon={Add}>Create</Button>

<style scoped>
	.color-knop {
		display: inline-block;
		width: 1em;
		height: 1em;
		border-radius: 50%;
		margin-right: 0.5em;
	}
</style>
