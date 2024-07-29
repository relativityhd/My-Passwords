<script>
	import {
		argbFromHex,
		themeFromSourceColor,
		applyTheme
	} from '@material/material-color-utilities';

	// Get the theme from a hex color
	let sourcecolor = argbFromHex('#825f6b');
	const theme = themeFromSourceColor(sourcecolor);

	// Print out the theme as JSON
	console.log(JSON.stringify(theme, null, 2));

	// Check if the user has dark mode turned on
	const systemDark = false; //window.matchMedia('(prefers-color-scheme: dark)').matches;

	console.log({ systemDark });
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

<img src="/noise.svg" alt="Noise overlay" class="overlay" />

<style global>
	:root {
		--md-ref-typeface-brand: 'DM Serif Display';
		--md-ref-typeface-plain: 'DM Serif Display';
	}
	body {
		font-family: var(--md-ref-typeface-brand);
		background: var(--md-sys-color-surface);
		color: var(--md-sys-color-on-background);
	}

	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: 9999;
		object-fit: cover;
		pointer-events: none;
	}
</style>
