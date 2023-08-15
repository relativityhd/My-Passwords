<script lang="ts">
	import { Grid, Row, Column } from 'carbon-components-svelte';
	import { Button, Modal } from 'carbon-components-svelte';
	import { FluidForm, TextInput, PasswordInput } from 'carbon-components-svelte';
	import { Select, SelectItem } from 'carbon-components-svelte';
	import Add from 'carbon-icons-svelte/lib/Send.svelte';
	import { DataTable } from 'carbon-components-svelte';

	import type { RetrievedSecretAccount } from '$lib/bindings';

	let colors = ['red', 'blue', 'green', 'yellow', 'purple', 'cyan', 'magenta', 'orange'];
	let bucketName = '';
	let color = colors[Math.floor(Math.random() * colors.length)];

	export let accounts: RetrievedSecretAccount[] = [];
</script>

<DataTable
	title="Accounts"
	headers={[
		{ key: 'institution_name', value: 'Institution' },
		{ key: 'account_name', value: 'Account/User name' },
		{ key: 'created_at', value: 'Created' },
		{ key: 'industry', value: 'Industry' }
	]}
	rows={accounts}
/>
