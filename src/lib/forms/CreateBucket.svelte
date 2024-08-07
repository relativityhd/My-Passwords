<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/button/text-button';
	import '@material/web/textfield/filled-text-field';
	import '@material/web/select/filled-select';
	import '@material/web/select/select-option';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	// import { liveInput, create } from './bindings';
	import ColorPicker, { ChromeVariant } from 'svelte-awesome-color-picker';
	import { createBucket } from '$lib/bindings';
	import { handleError } from '$lib/errorutils';
	import type { SerializedError } from '$lib/types';

	let name_element: MdFilledTextField;

	const recommendations = ['#576063', '#95324e', '#81377e', '#c39598', '#d864bc'];

	// CHoose random color from recommendations
	let hex = recommendations[Math.floor(Math.random() * recommendations.length)];

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
		let newbucket = await createBucket(name, hex).catch(
			handleError('forms/CreateBucket.svelte:handleSubmit')
		);
		console.log(newbucket);
		location.reload();
	}
</script>

<h1 class="headline">Create a new bucket</h1>
<div class="card" style="background-color: {hex};">
	<div class="container">
		<md-filled-text-field
			label="Bucket name"
			bind:this={name_element}
			on:change={handleInput}
			on:input={handleInput}
		/>
		<div class="color-picker">
			<ColorPicker
				bind:hex
				components={ChromeVariant}
				sliderDirection="horizontal"
				isDialog={false}
				isAlpha={false}
			/>
		</div>
		<div class="fast-colors">
			{#each recommendations as color}
				<div
					class="fast-color {color === hex ? 'selected' : ''}"
					style="background-color: {color}"
					on:click={() => (hex = color)}
					on:keypress={() => (hex = color)}
					tabindex="0"
					role="button"
				/>
			{/each}
		</div>
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
	.headline {
		text-align: center;
	}

	.card {
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		padding: 16px;
		max-width: 300px;
		margin: 0 auto;
	}

	.container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		grid-gap: 10px;
		grid-auto-flow: dense;
		margin: 16px 0;
	}

	.container .color-picker {
		text-align: center;
	}

	.fast-colors {
		display: flex;
		justify-content: center;
	}

	.fast-color {
		width: 30px;
		height: 30px;
		border-radius: 5%;
		margin: 5px;
		cursor: pointer;
		outline: 1px solid #030303;
	}

	.fast-color.selected {
		outline: 2px solid #030303;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
	}

	.buttons-row {
		display: flex;
		justify-content: flex-end;
	}
</style>
