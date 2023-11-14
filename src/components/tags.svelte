<script lang="ts">
	import datasetStore, {
		activeDatasetImageStore,
		activeDatasetTagsStore
	} from '$lib/stores/dataset.store';
	import { invoke } from '@tauri-apps/api/tauri';

	// TODO: need to figure out why the view doesn't automatically update when the store changes here
	// Doesn't make much sense, since it's supposed to be a reactive store
	// You have to click on a different image and then click back to the image you were on to see the changes, which sucks

	async function handleAddNewTag() {
		const newTag = (document.getElementById('new_tag_input') as HTMLInputElement | undefined)
			?.value;
		if (newTag && newTag !== '') {
			datasetStore.update((dataset) => {
				if (!dataset) return null;
				const image = dataset.data.find((image) => image.name === $activeDatasetImageStore);
				if (!image) return dataset;
				dataset.data.forEach(async (image, idx) => {
					if (image.name === $activeDatasetImageStore) {
						console.log(`invoking save_dataset_image_tags with new tag '${newTag}'`);
						const dataToSend = {
							name: image.name,
							path: image.path,
							tags: [...image.tags, newTag]
						};
						invoke('save_dataset_image_tags', { image: dataToSend }).then((res) => {
							if (res === true) {
								console.log(`backend says that the new tag '${newTag}' was saved`);
								dataset.data[idx].tags.push(newTag);
								console.log(`alright, we added the new tag '${newTag} to the dataset store'`);
							} else {
								console.log(`backend says that the new tag '${newTag}' was NOT saved`);
							}
						});
					}
				});
				return dataset;
			});
		}
	}

	async function handleDeleteTag(index: number) {
		datasetStore.update((dataset) => {
			if (!dataset) return null;
			const image = dataset.data.find((image) => image.name === $activeDatasetImageStore);
			if (!image) return dataset;
			const tag = image.tags[index];
			console.log(`invoking delete_dataset_image_tag for tag '${tag}'`);
			invoke('delete_dataset_image_tag', { tag, imageName: image.name }).then((res) => {
				if (res === true) {
					console.log(`backend says that the tag '${tag}' was deleted`);
					dataset.data.forEach((image, idx) => {
						if (image.name === $activeDatasetImageStore) {
							dataset.data[idx].tags.splice(index, 1);
						}
					});
					console.log(`alright, we deleted the tag '${tag}' from the dataset store'`);
				} else {
					console.log(`backend says that the tag '${tag}' was NOT deleted`);
				}
			});
			return dataset;
		});
	}
</script>

<div
	class="w-full h-full flex flex-col justify-start items-center gap-2 text-white outline outline-1 outline-white"
>
	<h1 class="">Tags:</h1>
	<div class="w-full h-full flex flex-col gap-2">
		{#if $activeDatasetTagsStore !== undefined}
			{#each $activeDatasetTagsStore as tag, index}
				<div class="w-full h-auto bg-zinc-600 p-1 flex flex-row justify-between items-center">
					<div>"{tag}"</div>
					<button on:click={() => handleDeleteTag(index)}>x</button>
				</div>
			{/each}
		{/if}
	</div>
	<div class="w-full h-fit py-2 flex flex-row justify-between items-center">
		<input id="new_tag_input" type="text" class="text-black" />
		<button on:click={handleAddNewTag}>add</button>
	</div>
</div>
