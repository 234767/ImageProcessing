<script lang="ts">
    import Greet from './lib/Greet.svelte'
    import {tauri} from "@tauri-apps/api";
    import {listen} from "@tauri-apps/api/event";
    let img_src = "";
    listen("file-open", (e) => {
        let path: string = e.payload.path
        loadImage(path)
    })

    function loadImage(path: string) {
        img_src = tauri.convertFileSrc(path)
    }
</script>

<main class="container">

	<div class="image-view">
		<!--suppress HtmlRequiredAltAttribute -->
		<img id="active-image" src={img_src} alt="No image loaded. Use File->Open" />
	</div>
	<div class="column" id="adjustments-view">
		<div class="histogram">Histogram</div>
		<div class:Greet></div>
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