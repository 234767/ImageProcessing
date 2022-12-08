<script lang="ts">
    import Histogram from "./lib/Histogram.svelte";
    import {tauri} from "@tauri-apps/api";
    import {listen} from "@tauri-apps/api/event";
    import {activeImagePath} from "./lib/stores";
    import Adjustments from "./lib/Adjustments.svelte";

    listen("file-open", (e) => {
        let path: string = e.payload.path
        activeImagePath.set(tauri.convertFileSrc(path))
    })

</script>

<main class="container">

	<div class="image-view">
		<img id="active-image" src={$activeImagePath} alt="No image loaded. Use File->Open" />
	</div>
	<div class="column" id="adjustments-view">
		<p>Histogram</p>
		<Histogram />
		<Adjustments/>
	</div>


</main>

<style>
	main {
		flex-grow: 1;
		padding: 0;
		height: 98vh;
		width: 99vw;
		display: grid;
		grid-template-columns: 3fr 1fr;
		border: dashed gray 1px;
	}

	#adjustments-view {
		border: solid red 1px;
		grid-column: 2;
	}

	.image-view {
		border: solid 2px white;
		grid-column: 1;
		overflow: hidden;
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.image-view > img {
		position: relative;
		max-height: 100%;
		width: 100%;
		object-fit: contain;
		text-align: center;
	}
</style>