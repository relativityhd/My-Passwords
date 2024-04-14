<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { MdOutlinedTextField } from '@material/web/textfield/outlined-text-field';
	import SearchResultContainer from '$lib/container/SearchResult.svelte';

	import { search, getSecurePassword } from '$lib/bindings';
	import type { SearchResult } from '$lib/bindings';

	const dispatch = createEventDispatcher();

	let search_element: MdOutlinedTextField;
	let searchterm = '';
	let delay = 500;
	let timeout: NodeJS.Timeout;

	let search_results: SearchResult[] = [];

	let search_error = '';
	$: search_results_visible = search_results.length > 0;

	function handleSearchInput() {
		searchterm = search_element.value;
		search_error = '';
		if (searchterm.length < 2) {
			search_results = [];
			search_results_visible = false;
			clearTimeout(timeout);
			return;
		}
		search_results_visible = true;

		clearTimeout(timeout);

		timeout = setTimeout(() => {
			console.log('searching for', searchterm);
			search(searchterm)
				.then((results) => {
					if (results.length == 0) {
						search_error = 'No results found.';
					}
					search_results = results;
					console.log(search_results);
				})
				.catch((error) => {
					console.error(error);
					search_error = 'An error occurred while searching. Please try again later.';
				});
		}, delay);
	}

	function selectPassword(event: CustomEvent<SearchResult>) {
		search_results = [];
		getSecurePassword(event.detail.id)
			.then((password) => {
				dispatch('password', password);
			})
			.catch((error) => {
				console.error(error);
			});
	}
</script>

<div class="super">
	<md-outlined-text-field
		label="Search"
		bind:this={search_element}
		on:input={handleSearchInput}
		error={search_error != ''}
		error-text={search_error}
	>
		<md-icon slot="leading-icon">search</md-icon>
	</md-outlined-text-field>

	<div class="result">
		<SearchResultContainer {search_results} on:select={selectPassword} />
	</div>
</div>

<style>
	.super {
		display: flex;
		justify-content: center;
		width: 100%;
	}
	.super md-outlined-text-field {
		width: 100%;
	}
	.result {
		position: absolute;
		width: calc(100% - 48px);
		margin-top: 56px;
		margin-left: 24px;
		margin-right: 24px;
		max-height: 400px;
		overflow-y: auto;
		z-index: 1;
	}
</style>
