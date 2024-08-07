<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/button/text-button';
	import '@material/web/textfield/filled-text-field';
	import '@material/web/select/filled-select';
	import '@material/web/select/select-option';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import ColorPicker, { ChromeVariant } from 'svelte-awesome-color-picker';
	import { createBucket } from '$lib/bindings';
	import type { SerializedError } from '$lib/types';
	import { createEventDispatcher } from 'svelte';
	import { logError, logMsg } from '$lib/errorutils';

	enum View {
		Form,
		Error
	}

	let show: View = View.Form;
	let name_element: MdFilledTextField;
	let valid = false;
	const recommendations = ['#576063', '#95324e', '#81377e', '#c39598', '#d864bc'];
	// Choose random color from recommendations
	let hex = recommendations[Math.floor(Math.random() * recommendations.length)];

	const dispatch = createEventDispatcher();

	function handleInput() {
		if (!name_element) return;
		let name = name_element.value;
		name_element.setCustomValidity('');
		name_element.reportValidity();
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
		// Truncate to 6 characters, because the ColorPicker 'can' return 9 characters
		if (hex.length > 7) {
			hex = hex.slice(0, 7);
		}
		logMsg('Create a new bucket...', { name, hex });
		let newbucket = await createBucket(name, hex).catch(async (e: SerializedError) => {
			if (e.status !== 400) {
				show = View.Error;
				return await logError('forms/CreateBucket.svelte:handleSubmit', { name, hex })(e);
			}
			logMsg('Could not create new bucket, because of validation error', { name, hex, e });
			name_element.setCustomValidity(e.message);
			name_element.reportValidity();
			valid = false;
			return false;
		});
		if (newbucket) {
			logMsg('New bucket created with id ' + newbucket);
			dispatch('created', newbucket);
		}
	}
</script>

<h1 class="headline">Create a new bucket</h1>
<form class="card" style="background-color: {hex};">
	{#if show === View.Form}
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
	{:else}
		<p>500 | An unexpected error occured.</p>
		<div class="buttons-row">
			<md-filled-button
				on:click={() => location.reload()}
				on:keydown={() => location.reload()}
				role="button"
				tabindex="0">Reload</md-filled-button
			>
		</div>
	{/if}
</form>

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
