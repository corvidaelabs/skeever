<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<main class="bg-primary-500 min-h-screen py-4">
	{@render children()}
</main>
