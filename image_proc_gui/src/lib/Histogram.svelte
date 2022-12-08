<script lang="ts">
    import {listen} from "@tauri-apps/api/event";
    import {tauri} from "@tauri-apps/api";
    import {activeHistogramPath} from "./stores";

    listen(
        "active-histogram-update", async (e) => {
            let path = e.payload.path;
            activeHistogramPath.set(tauri.convertFileSrc(path));
        }
    )
</script>

<div class="histogram">
	<img src={$activeHistogramPath} alt="image_histogram">
</div>