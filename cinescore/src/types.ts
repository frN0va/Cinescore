export type Movie = {
	id: number;
	title: string;
	poster: string;
	description: string;
	rank: number;
	overallScore?: number;
	isLiked: boolean;
	inWatchlist: boolean;
};
