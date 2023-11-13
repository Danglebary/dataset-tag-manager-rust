import type { Dataset } from '$lib/types';
import { writable, derived } from 'svelte/store';

const datasetStore = writable<Dataset | null>(null);

export const datasetTagsStore = derived(datasetStore, ($dataset) => {
	const tags = new Set<string>();
	$dataset?.data.forEach((image) => image.tags.forEach((tag) => tags.add(tag)));
	return Array.from(tags);
});

export default datasetStore;
