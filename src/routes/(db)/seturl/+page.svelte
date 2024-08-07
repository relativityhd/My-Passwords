<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/icon/icon';
	import '@material/web/textfield/filled-text-field';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import '@material/web/checkbox/checkbox';
	import { connect } from '$lib/bindings';
	import { goto } from '$app/navigation';
	import type { SerializedError } from '$lib/types';
	import { handleError } from '$lib/errorutils';

	let urlElement: MdFilledTextField;

	let isValid = false;

	async function handleSubmit() {
		let url = urlElement.value;
		await connect(url).catch((err: SerializedError) => {
			if (err.status !== 400) {
				handleError('db/seturl/+page.svelte:handleSubmit')(err);
			}
			isValid = false;
			urlElement.setCustomValidity(err.message);
			urlElement.reportValidity();
		});
		goto('/');
	}
</script>

<div class="left">
	<h1>Connect</h1>
	<p>
		Please enter a URL to a MyPassword database. This URL should be given to you by a referrer.
		Please note that the URL must start with either "ws://" or "wss://".
	</p>
</div>
<div class="right">
	<div class="inputs">
		<md-filled-text-field
			bind:this={urlElement}
			label="URL"
			type="text"
			value=""
			required
			on:change={() => {
				urlElement.setCustomValidity('');
				urlElement.reportValidity();
				isValid = urlElement.validity.valid;
			}}
			on:input={() => {
				urlElement.setCustomValidity('');
				urlElement.reportValidity();
				isValid = urlElement.validity.valid;
			}}
		/>
	</div>
	<div class="buttons">
		<md-filled-button
			trailing-icon
			disabled={!isValid}
			on:click={handleSubmit}
			on:keypress={handleSubmit}
			role="button"
			tabindex="0">Connect <md-icon slot="icon">arrow_forward</md-icon></md-filled-button
		>
	</div>
</div>

<style scoped>
	.inputs {
		align-items: flex-start;
		flex-direction: column;
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}

	.buttons {
		display: flex;
		flex-flow: row wrap;
		justify-content: flex-end;
		margin-top: 8px;
	}
	.left {
		max-width: 300px;
	}
</style>
