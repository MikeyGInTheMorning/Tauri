<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import Sidebar from './components/sidebar.svelte';
	
	let sidebar_show = false;
	
	export let name;
	let a = async ()=>{
		await invoke('write_report')
	}
	
</script>

<main>
	<Sidebar bind:show={sidebar_show} />
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	<button on:click={() => sidebar_show = !sidebar_show}>Toggle Sidebar</button>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>