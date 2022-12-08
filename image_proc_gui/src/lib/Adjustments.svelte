<script lang="ts">
    import Slider from '@bulatdashiev/svelte-slider';

    let brightnessSlider = [0, 0];
    $: brightness = brightnessSlider[0]

    let contrastSlider = [1.0,0];
		$: contrast = contrastSlider[0]
</script>

<div class="column adjustments">
	<div class="group brightness">
		<span class="title">
			<label>Brightness:</label>
			<input value={brightnessSlider[0]}>
		</span>
		<Slider min={-255} max={255} bind:value={brightnessSlider}/>
	</div>
	<div class="group contrast">
		<span class="title">
			<label>Contrast:</label>
			<input value={contrastSlider[0]}>
		</span>
		<Slider min={0} max={10} bind:value={contrastSlider} step={0.1} />
	</div>
</div>

<style>
	.adjustments {
		margin-inline: auto;
		width: 90%;
		display: flex;
		flex-direction: column;
	}

	.group {
		display: block;
		text-align: left;
	}

	.title {
		display: flex;
		align-items: center;
		justify-content: left;
		margin-inline: 6px;
	}

	.title label {
		display: inline;
		flex-grow: 1;
	}

	.title input {
		margin-left: fill;
		display: inline;
		width: 2em;
		height: .7em;
	}
</style>

