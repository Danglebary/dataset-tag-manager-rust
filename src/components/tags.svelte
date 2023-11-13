<script lang="ts">
	import activeDatasetItemStore from '$lib/stores/active-item.store';
</script>

<div
	class="w-full h-full flex flex-col justify-start items-center gap-2 text-white outline outline-1 outline-white"
>
	<h1 class="">Tags:</h1>
	<div class="w-full h-full flex flex-col gap-2">
		{#if $activeDatasetItemStore !== null}
			{#each $activeDatasetItemStore.tags as tag}
				<div class="w-full h-auto bg-zinc-600 p-1">"{tag}"</div>
			{/each}
		{/if}
	</div>
</div>
