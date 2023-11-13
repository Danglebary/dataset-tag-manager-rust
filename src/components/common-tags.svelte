<script lang="ts">
	import { datasetTagsStore } from '$lib/stores/dataset.store';
</script>

<div
	class="w-full h-full flex flex-col justify-start items-center gap-2 text-white outline outline-1 outline-white rounded-br-lg rounded-tr-lg"
>
	<h1 class="">All Tags in Dataset:</h1>
	<div class="w-full h-full flex flex-col gap-2">
		{#if $datasetTagsStore !== null}
			{#each $datasetTagsStore as tag}
				<div class="w-full h-auto bg-zinc-600 p-1">"{tag}"</div>
			{/each}
		{/if}
	</div>
</div>
