<script lang="ts">
    import Greet from './lib/Greet.svelte'
</script>

<main class="container">
	<h1>Hello!</h1>

	<div class="row">
		<Greet />
	</div>


</main>

<style>
	.logo.vite:hover {
		filter: drop-shadow(0 0 2em #747BFF);
	}

	.logo.svelte:hover {
		filter: drop-shadow(0 0 2em #FF3E00);
	}
</style>