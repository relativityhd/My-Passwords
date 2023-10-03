<script>
	import {
		argbFromHex,
		themeFromSourceColor,
		applyTheme
	} from '@material/material-color-utilities';

	// Get the theme from a hex color
	const theme = themeFromSourceColor(argbFromHex('#f82506'), [
		{
			name: 'custom-1',
			value: argbFromHex('#ff0000'),
			blend: true
		}
	]);

	// Print out the theme as JSON
	console.log(JSON.stringify(theme, null, 2));

	// Check if the user has dark mode turned on
	const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

	// Apply the theme to the body by updating custom properties for material tokens
	applyTheme(theme, { target: document.body, dark: systemDark });
</script>

<svelte:head>
	<link href="https://fonts.googleapis.com/css?family=Gelasio" rel="stylesheet" />
</svelte:head>
<slot />

<style global>
	:root {
		--md-ref-typeface-brand: 'Montserrat';
		--md-ref-typeface-plain: 'Montserrat';
	}
	body {
		font-family: Montserrat;
	}
</style>
