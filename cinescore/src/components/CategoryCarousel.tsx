import { ChevronLeft, ChevronRight } from "lucide-react";
import MoviePoster from "./MoviePoster";
import type { Movie } from "../types";

interface CarouselProps {
	category: string;
	categoryMovies: Array<Movie>;
	currentPage: Record<string, number>;
	onRank: (id: number, rank: number) => void;
	onLike: (id: number) => void;
	onAddToWatchlist: (id: number) => void;
	onPrevPage: (category: string) => void;
	onNextPage: (category: string) => void;
}

export const MOVIES_PER_PAGE = 5;

const CategoryCarousel: React.FC<CarouselProps> = ({
	category,
	categoryMovies,
	currentPage,
	onRank,
	onLike,
	onAddToWatchlist,
	onPrevPage,
	onNextPage,
}) => {
	return (
		<section key={category} className="mb-12">
			<div className="mb-6 flex items-center justify-between border-b border-neutral-800 pb-2">
				<h2 className="text-2xl font-semibold text-blue-300">{category}</h2>
				<div className="flex space-x-2">
					<button
						type="button"
						className="rounded-full p-1 text-neutral-400 transition-colors hover:bg-neutral-800 hover:text-white"
						onClick={() => onPrevPage(category)}
					>
						<ChevronLeft className="h-6 w-6" />
					</button>
					<button
						type="button"
						className="rounded-full p-1 text-neutral-400 transition-colors hover:bg-neutral-800 hover:text-white"
						onClick={() => onNextPage(category)}
					>
						<ChevronRight className="h-6 w-6" />
					</button>
				</div>
			</div>
			<div className="grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
				{categoryMovies
					.slice(
						currentPage[category] * MOVIES_PER_PAGE,
						(currentPage[category] + 1) * MOVIES_PER_PAGE,
					)
					.map((movie) => (
						<MoviePoster
							key={movie.id}
							movie={movie}
							onAddToWatchlist={onAddToWatchlist}
							onLike={onLike}
							onRank={onRank}
						/>
					))}
			</div>
		</section>
	);
};

export default CategoryCarousel;
