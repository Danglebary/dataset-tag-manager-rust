import type { Dataset } from '$lib/types';
import { writable, derived } from 'svelte/store';

// Main store
const datasetStore = writable<Dataset | null>(null);

// Active dataset image, used for the tags viewer
export const activeDatasetImageStore = writable<string | null>(null);

// Active dataset tags, derived from the active dataset image and the dataset stores
export const activeDatasetTagsStore = derived(
	[datasetStore, activeDatasetImageStore],
	([$dataset, $imageName]) => {
		if (!$dataset || !$imageName) return [] as string[];
		const datasetImage = $dataset.data.find((img) => img.name === $imageName);
		if (!datasetImage) {
			console.log('literally how the hell did this happen????');
			return [] as string[];
		}
		return datasetImage.tags;
	}
);

export const datasetTagsStore = derived(datasetStore, ($dataset) => {
	const tags = new Set<string>();
	$dataset?.data.forEach((image) => image.tags.forEach((tag) => tags.add(tag)));
	return Array.from(tags);
});

export default datasetStore;
