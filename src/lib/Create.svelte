<script>
	import { Button, Modal } from 'carbon-components-svelte';
	import { FluidForm, TextInput, PasswordInput } from 'carbon-components-svelte';
	import { Select, SelectItem } from 'carbon-components-svelte';
	import Add from 'carbon-icons-svelte/lib/Send.svelte';
	import { Industry } from '$lib/types';
	import { invoke } from '@tauri-apps/api/tauri';

	let open = false;

	// Form data
	let accountName = '';
	let institutionName = '';
	let industry = Industry.Other;

	async function submit() {
		console.log('submit');
		let acc_id = await invoke('add_acc', {
			institutionName,
			accountName,
			industry
		}).catch((err) => {
			console.log(err);
		});
		console.log(acc_id);
	}
</script>

<Button on:click={() => (open = true)}>Add Account</Button>

<Modal
	bind:open
	modalHeading="Add Account"
	primaryButtonText="Add"
	primaryButtonIcon={Add}
	secondaryButtonText="Cancel"
	hasScrollingContent
	on:click:button--secondary={() => (open = false)}
	on:open
	on:close
	on:submit={submit}
>
	<FluidForm>
		<TextInput
			bind:value={institutionName}
			labelText="Institution"
			placeholder="E.g. Google"
			required
		/>
		<TextInput
			bind:value={accountName}
			labelText="Account/User Name"
			placeholder="E.g. the email or username of that account"
			required
		/>
	</FluidForm>
	<Select labelText="Industry" bind:selected={industry}>
		<SelectItem value={Industry.Other} />
		<SelectItem value={Industry.Tech} />
		<SelectItem value={Industry.Games} />
		<SelectItem value={Industry.Social} />
		<SelectItem value={Industry.Finance} />
		<SelectItem value={Industry.Shopping} />
		<SelectItem value={Industry.Science} />
	</Select>
</Modal>
