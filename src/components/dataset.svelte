<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import { listen } from '@tauri-apps/api/event';
	import type { Dataset } from '$lib/types';
	import datasetStore from '$lib/stores/dataset.store';
	import activeDatasetItemStore from '$lib/stores/active-item.store';

	let unlisten: UnlistenFn | null = null;

	onMount(async () => {
		unlisten = await listen('dataset_loaded', (event) => {
			console.log(event.payload);

			datasetStore.set(event.payload as Dataset);
			activeDatasetItemStore.set((event.payload as Dataset).data[0]);
			console.log($activeDatasetItemStore);
		});
	});

	onDestroy(() => {
		if (unlisten) unlisten();
	});

	function handleDatasetItemClick(idx: number) {
		if ($datasetStore !== null) {
			activeDatasetItemStore.set($datasetStore.data[idx]);
		}
	}
</script>

<div class="w-full h-full flex flex-col justify-start items-center gap-2">
	<div class="w-full text-center">toolbar</div>
	<div class="w-full h-full flex flex-col justify-start items-center gap-2">
		{#if $datasetStore !== null}
			{#each $datasetStore.data as image, index}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div
					class={`w-full h-40 grid grid-cols-2 justify-start items-start bg-zinc-600 cursor-pointer p-2 ${
						$activeDatasetItemStore &&
						$activeDatasetItemStore.name === image.name &&
						'outline outline-2 outline-blue-400'
					}`}
					on:click={() => handleDatasetItemClick(index)}
				>
					<div class="w-full h-40 flex flex-col justify-center">
						<h2 class="select-none">{image.name}</h2>
					</div>
					<div class="w-full h-36">
						<img
							src={convertFileSrc(image.path)}
							alt={image.name}
							class="h-full object-contain aspect-auto select-none"
						/>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>
