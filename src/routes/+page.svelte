<script lang="ts">
	import { VITE_API_URL } from '$env/static/private';
	import { Avatar } from '@skeletonlabs/skeleton-svelte';
	import { onMount, onDestroy } from 'svelte';
	import { fade, fly } from 'svelte/transition';

	let socket: WebSocket;
	type Squeak = {
		id: string;
		content: string;
		author: {
			name: string;
			avatar_url: string;
		};
	};
	let squeaks: Squeak[] = $state([]);

	onMount(() => {
		// Connect to the WebSocket server
		socket = new WebSocket(`${VITE_API_URL}/ws`);

		socket.onopen = () => {
			console.log('Connected to WebSocket server');
		};

		socket.onmessage = (event) => {
			try {
				const squeak: Squeak = JSON.parse(event.data);
				squeaks = [squeak, ...squeaks];
			} catch (error) {
				console.error('Error parsing squeak:', error);
			}
		};

		socket.onerror = (error) => {
			console.error('WebSocket error:', error);
		};

		socket.onclose = () => {
			console.log('Disconnected from WebSocket server');
		};
	});

	onDestroy(() => {
		if (socket) {
			socket.close();
		}
	});
</script>

<main class="flex flex-col items-center justify-center space-y-10">
	<p class="text-8xl text-white">its squeakin time</p>
	<div class="items-left flex-col space-y-4">
		{#each squeaks as squeak, i (squeak.id)}
			<div
				class="bg-primary-600 border-primary-700 w-[300px] space-y-4 rounded-xs border p-4 shadow-lg"
				in:fly={{ y: -20, duration: 400, delay: i * 50 }}
				out:fade={{ duration: 300 }}
			>
				<p class="text-sans text-lg font-semibold">{squeak.content}</p>
				<div class="flex items-center justify-end space-x-2">
					<Avatar src={squeak.author.avatar_url} name={squeak.author.name} size="w-10" />
					<p class="text-right font-sans">{squeak.author.name}</p>
				</div>
			</div>
		{/each}
	</div>
</main>
