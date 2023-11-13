export type DatasetImage = {
	name: string;
	path: string;
	tags: string[];
};

export type Dataset = {
	name: string;
	path: string;
	data: DatasetImage[];
};
