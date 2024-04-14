<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/button/text-button';
	import '@material/web/textfield/filled-text-field';
	import '@material/web/select/filled-select';
	import '@material/web/select/select-option';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	// import { liveInput, create } from './bindings';
	import ColorPicker from 'svelte-awesome-color-picker';
	import { createBucket } from '$lib/bindings';

	let name_element: MdFilledTextField;

	let hex: string = '#000000';

	let valid = false;
	async function handleInput() {
		let name = name_element.value;
		if (name.length === 0) {
			valid = false;
			return;
		}
		valid = true;
	}

	async function handleSubmit() {
		let name = name_element.value;
		if (name.length === 0) {
			valid = false;
			return;
		}
		console.log({
			name,
			hex
		});
		let newbucket = await createBucket(name, hex);
		console.log(newbucket);
		location.reload();
	}
</script>

<div class="card">
	<h3>Create a new bucket</h3>
	<div class="container">
		<md-filled-text-field
			label="Bucket name"
			bind:this={name_element}
			on:change={handleInput}
			on:input={handleInput}
		/>
		<ColorPicker bind:hex />
	</div>

	<div class="buttons-row">
		<md-filled-button
			on:click={handleSubmit}
			on:keydown={() => handleSubmit}
			role="button"
			tabindex="0"
			disabled={!valid}>Create</md-filled-button
		>
	</div>
</div>

<style>
	.card {
		background: var(--md-sys-color-tertiary-container);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		padding: 16px;
		max-width: 600px;
		margin: 0 auto;
	}

	.container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		grid-gap: 10px;
		grid-auto-flow: dense;
		margin: 16px 0;
	}

	.buttons-row {
		display: flex;
		justify-content: flex-end;
	}
</style>
