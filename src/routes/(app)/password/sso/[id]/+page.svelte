<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/button/outlined-button';
	import '@material/web/iconbutton/filled-icon-button';
	import '@material/web/textfield/outlined-text-field';
	import '@material/web/icon/icon';
	import '@material/web/list/list';
	import '@material/web/list/list-item';
	import '@material/web/divider/divider';
	import '@material/web/iconbutton/icon-button';
	import { DateTime } from 'luxon';
	import EditSso from '$lib/forms/EditSSO.svelte';
	import { deleteAccount } from '$lib/bindings';
	import { goto } from '$app/navigation';
	import { handleError } from '$lib/errorutils.js';

	enum View {
		Overview,
		Edit,
		Delete
	}

	let show: View = View.Overview;

	async function triggerDelete() {
		await deleteAccount(data.id).catch(handleError('app/password/sso/+page.svelte:triggerDelete'));
		goto('/');
	}

	export let data;
</script>

{#if show === View.Overview}
	<div class="grid-container">
		<div class="back container">
			<md-icon-button href="/">
				<md-icon>arrow_back</md-icon>
			</md-icon-button>
		</div>
		<div class="institution container">
			<div class="insitution-info">
				<h3>{data.account.institution}</h3>
			</div>
		</div>

		{#if data.account.bucket}
			<div class="bucket container">
				<div class="bucket-info">
					<md-icon style="color: {data.account.bucket.color};">category</md-icon>
					<p>{data.account.bucket.name}</p>
				</div>
			</div>
		{:else}
			<div class="bucket container">
				<div class="bucket-info">
					<md-icon>category</md-icon>
					<p>Bucket: Unsorted</p>
				</div>
			</div>
		{/if}

		{#if data.account.twofactor}
			<div class="twofactor container">
				<div class="twofactor-info">
					<md-icon>encrypted</md-icon>
					<p>
						Secured by <b>{data.account.twofactor.name}</b> on
						<b>{data.account.twofactor.device}</b>
					</p>
				</div>
			</div>
		{:else}
			<div class="twofactor container">
				<div class="twofactor-info">
					<md-icon>encrypted</md-icon>
					<p>No 2FA</p>
				</div>
			</div>
		{/if}
		<div class="password container">
			<h1>SSO::{data.account.ssoaccount_institution}</h1>
			<md-filled-icon-button>
				<md-icon>content_copy</md-icon>
			</md-filled-icon-button>
		</div>

		<div class="data container">
			<md-list>
				<md-divider />
				<md-list-item>
					Created: {DateTime.fromISO(data.account.created).toRelativeCalendar()}
				</md-list-item>
				{#if data.account.website}
					<md-divider />
					<md-list-item>
						Website: <a href={data.account.website} target="_blank">{data.account.website}</a>
					</md-list-item>
				{/if}

				{#if data.account.recovery}
					<md-divider />
					<md-list-item>
						Recovery: {data.account.recovery}
					</md-list-item>
				{/if}
			</md-list>

			<div class="actions">
				<md-outlined-button
					trailing-icon
					on:click={() => (show = View.Edit)}
					on:keypress={() => (show = View.Edit)}
					role="button"
					tabindex="0"
				>
					Edit <md-icon slot="icon">edit</md-icon>
				</md-outlined-button>
				<md-outlined-button
					trailing-icon
					on:click={() => (show = View.Delete)}
					on:keypress={() => (show = View.Delete)}
					role="button"
					tabindex="0"
				>
					Delete <md-icon slot="icon">delete</md-icon>
				</md-outlined-button>
			</div>
		</div>
	</div>
{:else if show === View.Edit}
	<div>
		<EditSso
			account={data.account}
			id={data.id}
			buckets={data.buckets}
			accounts={data.nosso_accounts}
			on:close={() => (show = View.Overview)}
		/>
	</div>
{:else if show === View.Delete}
	<div class="delete-container">
		{#if data.deletelocked}
			<h1>Unable to delete</h1>
			<p>
				This account is currently used as SSO authentification for other accounts. Please delete
				them first before deleting this account.
			</p>
		{:else}
			<h1>Are you sure?</h1>
			<p>The account will be archived and hidden from the UI - you can restore it later.</p>
			<div class="actions">
				<md-filled-button
					on:click={() => triggerDelete()}
					on:keypress={() => triggerDelete()}
					role="button"
					tabindex="0">Delete</md-filled-button
				>
			</div>
		{/if}
	</div>
{/if}

<style>
	.grid-container {
		display: grid;
		grid-template-columns: 56px 3fr 1fr 1fr 3fr;
		grid-template-rows: auto;
		grid-gap: 4px;
		grid-template-areas:
			'back institution bucket twofactor data'
			'password password password password data';
		place-items: stretch;
		place-content: stretch;
		max-width: 1600px;
		margin: 10vh auto;
	}

	@media (max-width: 1100px) {
		.grid-container {
			grid-template-columns: 56px 3fr 1fr 1fr;
			grid-template-areas:
				'back institution bucket twofactor'
				'password password password password'
				'data data data data';
		}
	}

	@media (max-width: 730px) {
		.grid-container {
			grid-template-columns: 56px 1fr 1fr 56px;
			grid-template-areas:
				'back institution institution institution'
				'password password password password'
				'bucket bucket twofactor twofactor'
				'data data data data';
		}
	}

	.container {
		display: flex;
		flex-flow: column wrap;
		justify-content: center;
		align-items: stretch;
		height: calc(100% - 32px);
		width: calc(100% - 32px);
		padding: 16px;
	}

	.password {
		grid-area: password;
		flex-flow: row nowrap;
		align-items: center;
		justify-content: center;
	}

	.password md-filled-icon-button {
		margin-left: 16px;
	}

	.back {
		grid-area: back;
		align-items: center;
		justify-content: center;
	}

	.institution {
		grid-area: institution;
		flex-flow: row wrap;
		align-items: center;
		justify-content: flex-start;
	}

	.insitution-info {
		background: var(--md-sys-color-primary);
		color: var(--md-sys-color-on-primary);
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		padding: 10px 17px;
		height: 60px;
		width: 100%;
	}

	.institution h3 {
		margin: 0;
	}

	.bucket {
		grid-area: bucket;
		flex-flow: row wrap;
		align-items: center;
		justify-content: flex-start;
	}

	.bucket-info {
		background: var(--md-sys-color-secondary-container);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		padding: 10px 17px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-flow: column;
		height: 60px;
		width: 100%;
		min-width: 150px;
	}

	.bucket-info md-icon {
		margin: 0;
		width: 48px;
		height: 48px;
		font-size: 48px;
	}

	.bucket-info p {
		margin: 0;
		opacity: 0.8;
		font-size: 10px;
		text-align: center;
	}

	.twofactor {
		grid-area: twofactor;
		flex-flow: row wrap;
		align-items: center;
		justify-content: flex-start;
	}

	.twofactor-info {
		background: var(--md-sys-color-secondary-container);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		padding: 10px 17px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-flow: column;
		height: 60px;
		width: 100%;
		min-width: 150px;
	}

	.twofactor-info md-icon {
		margin: 0;
		width: 48px;
		height: 48px;
		font-size: 48px;
	}

	.twofactor-info p {
		margin: 0;
		opacity: 0.8;
		font-size: 10px;
		text-align: center;
	}

	.data {
		grid-area: data;
	}
</style>
