import type { DatasetImage } from '$lib/types';
import { writable } from 'svelte/store';

const activeDatasetItemStore = writable<DatasetImage | null>(null);

export default activeDatasetItemStore;
