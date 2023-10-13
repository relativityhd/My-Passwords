<script>
	import {
		argbFromHex,
		themeFromSourceColor,
		applyTheme
	} from '@material/material-color-utilities';

	// Get the theme from a hex color
	const theme = themeFromSourceColor(argbFromHex('#0054a4'));

	// Print out the theme as JSON
	console.log(JSON.stringify(theme, null, 2));

	// Check if the user has dark mode turned on
	const systemDark = false; //window.matchMedia('(prefers-color-scheme: dark)').matches;

	console.log(systemDark);
	// Apply the theme to the body by updating custom properties for material tokens
	applyTheme(theme, { target: document.body, dark: systemDark });
</script>

<svelte:head>
	<link href="https://fonts.googleapis.com/css?family=DM+Serif+Display" rel="stylesheet" />
	<link
		href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined"
		rel="stylesheet"
	/>
</svelte:head>

<slot />

<style global>
	:root {
		--md-ref-typeface-brand: 'DM Serif Display';
		--md-ref-typeface-plain: 'DM Serif Display';
	}
	body {
		font-family: var(--md-ref-typeface-brand);
		background: var(--md-sys-color-surface-variant);
		color: var(--md-sys-color-on-background);
	}
</style>
