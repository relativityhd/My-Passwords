<script lang="ts">
	import { goto } from '$app/navigation';
	import '@material/web/icon/icon';

	const wallpapers = [
		'/desert-ribbons.jpg',
		'/facing-the-tide.jpg',
		'/painting-the-desert.jpg',
		'/rapid-ice-movement.jpg',
		'/re-entry.jpg'
	];

	const wallpaper_describtions = [
		'Rock folding on a tectonic scale occurred in northwestern Africa. These motley ribbons dancing across the desert in Morocco are folds caused by the prolonged collision of tectonic plates. The long continuous line is Jbel Ouarkziz, a ridge that rises 200–300 meters above the valley floors.',
		'Rupert Bay, an arm of James Bay, extends into Quebec, Canada. Many rivers carry sediment into the bay and combine with seawater coming in from the tide. A prominent sediment stream extends past Stag Island and a vortex curls off Stag Rock in the middle of the bay. Sediment trails off the islands toward the mainland, indicating the tide was coming in at the time of image acquisition.',
		'The Lake Eyre Basin is one of the driest places in Australia. But this image features a rare green flush to this otherwise parched landscape. Streams and creeks that drain into the basin are usually dry, but storms in March 2018 delivered water to these braided channels. By April, the floodwater had receded and left a green expanse behind. Scientists use satellites to track such flooding and greening events.',
		'One glacier on Russian islands in the Arctic Ocean surprised scientists with its rapid change. After decades of normal, slow movement, a glacier draining Vavilov Ice Cap sprang forward, accelerating rapidly after 2013. This fast movement is extremely rare for cold-based glaciers. In 5 years, the ice tongue doubled in size. In this inverted rendition, land is blue and fractured sea ice appears tan across the top of the image.',
		'Jebel Kissu, in northwestern Sudan, emerges abruptly like an island in the vast Sahara Desert. The plateau is the eroded remnant of a granite dome. The bright linear features are truck tracks, common in the Sahara where there are no paved roads. Resembling graphic novel art style, this image could be an asteroid hurtling toward Earth, burning across a twilight sky.'
	];

	function getIdxFromColor(color: string): number {
		let charcodes = color.split('').map((c) => c.charCodeAt(0));
		return charcodes.reduce((acc, cur) => acc + cur, 0) % wallpapers.length;
	}

	$: wallpaper = wallpapers[getIdxFromColor(color)];
	$: describtion = wallpaper_describtions[getIdxFromColor(color)];

	export let name: string;
	export let n: number;
	export let color: string;
	export let id: string;
</script>

<div
	class="grid-item"
	on:click={() => goto('/list')}
	on:keypress={() => goto('/list')}
	role="button"
	tabindex="0"
>
	<div class="card-image-wrapper">
		<img class="card-image" src={`/wallpaper${wallpaper}`} alt="Artisitc Satellite Imagery" />
	</div>
	<div class="card-content">
		<div class="content-header">
			<h3>{name} ({n})</h3>

			<div class="icon">
				<md-icon style="color: {color};">category</md-icon>
			</div>
		</div>

		<div class="content">
			<p>
				Imagery taken from Earth Resources Observation and Science (EROS) Center "Earth as Art 6"
				collection.
				<a href="https://eros.usgs.gov/media-gallery/earth-as-art/earth-as-art-6">Online</a>
			</p>
			<p>
				{describtion}
			</p>
		</div>
	</div>
</div>

<style scoped>
	.grid-item {
		background: var(--md-sys-color-tertiary-container);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;

		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: box-shadow 0.1s ease-in-out, transform 0.1s ease-in-out;
		width: 300px;
	}

	.grid-item:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
		transform: scale(1.02);
		cursor: pointer;
	}

	.grid-item .content-header {
		flex-grow: 1;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.grid-item h3 {
		margin: 0;
	}

	.grid-item p {
		font-size: xx-small;
		text-align: justify;
	}

	.card-image-wrapper {
		width: 100%;
		height: 150px;
		overflow: hidden;
		border-radius: 12px;
	}
	.card-image {
		width: 100%;
		height: 300px;
		object-fit: cover;
		border-radius: 12px;
		transform: scale(1.1);
		transition: transform 0.5s;
	}
	/* Add animation to zoom the image slighlty on hover of the card */
	.grid-item:hover .card-image,
	.grid-item:focus .card-image {
		transform: scale(1.15);
	}

	.card-content {
		padding: 4px 16px 16px 16px;
	}
</style>
