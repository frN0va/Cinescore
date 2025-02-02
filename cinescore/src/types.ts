export type Movie = {
	id: number;
	title: string;
	poster: string;
	description: string;
	rank: number;
	overallScore?: number;
	isLiked: boolean;
	inWatchlist: boolean;
	releaseDate: string;
};

export type Cast = {
	name?: string;
	iconUrl?: string;
	character: string;
	posterUrl?: string;
	title?: string;
};

export type Crew = {
	name?: string;
	iconUrl?: string;
	department: string;
	posterUrl?: string;
	title?: string;
	id: number;
};

export type Credits = {
	cast: Array<Cast>;
	crew: Array<Crew>;
};
