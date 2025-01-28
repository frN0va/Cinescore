import React, { useState } from "react";
import { Star, Heart, Plus, Check } from "lucide-react";
import type { Movie } from "../types";

interface MoviePosterProps {
	movie: Movie;
	onRank: (id: number, rank: number) => void;
	onLike: (id: number) => void;
	onAddToWatchlist: (id: number) => void;
}

const MoviePoster: React.FC<MoviePosterProps> = ({
	movie,
	onRank,
	onLike,
	onAddToWatchlist,
}) => {
	const [showDetails, setShowDetails] = useState(false);
	const [hoveredRank, setHoveredRank] = useState(0);

	return (
		<div className="flex flex-col cursor-pointer">
			<div
				className="relative group overflow-hidden rounded-lg shadow-xl transition-all duration-300 ease-in-out"
				onMouseEnter={() => setShowDetails(true)}
				onMouseLeave={() => setShowDetails(false)}
			>
				<div className="relative">
					<img
						src={movie.poster}
						alt={movie.title}
						className="h-[300px] w-full object-cover transition-transform duration-300 group-hover:scale-105"
					/>
					{movie.overallScore !== undefined && (
						<div className="absolute right-3 top-3 flex items-center rounded-full bg-black/70 px-3 py-1 text-yellow-400">
							<Star className="mr-1 h-5 w-5" />
							<span className="font-bold">{movie.overallScore.toFixed(1)}</span>
						</div>
					)}
				</div>
				{showDetails && (
					<div className="absolute inset-0 flex flex-col justify-between bg-black/80 p-4 text-white opacity-0 transition-opacity duration-300 group-hover:opacity-100">
						<div>
							<h3 className="mb-2 text-2xl font-bold">{movie.title}</h3>
							<p className="text-sm">{movie.description}</p>
						</div>
						<div className="mt-4 flex justify-center space-x-2">
							{[1, 2, 3, 4, 5].map((rank) => (
								<Star
									key={rank}
									className={`h-8 w-8 cursor-pointer transition-colors ${
										rank <= hoveredRank ? "text-yellow-400" : "text-gray-500"
									} hover:text-yellow-300`}
									onMouseEnter={() => setHoveredRank(rank)}
									onMouseLeave={() => setHoveredRank(movie.rank)}
									onClick={() => onRank(movie.id, rank)}
								/>
							))}
						</div>
					</div>
				)}
			</div>
			<div className="mt-2 flex justify-center space-x-2">
				<button
					type="button"
					onClick={(e) => {
						e.stopPropagation();
						onLike(movie.id);
					}}
					className={`rounded-full p-2 transition-colors duration-300 ${
						movie.isLiked
							? "bg-red-100 text-red-500"
							: "bg-neutral-800 text-neutral-300 hover:bg-neutral-700"
					}`}
					title={movie.isLiked ? "Remove from favorites" : "Add to favorites"}
				>
					<Heart
						className={`h-5 w-5 transition-transform duration-300 ${
							movie.isLiked
								? "fill-red-400 text-red-500"
								: "fill-none stroke-white"
						}`}
						style={{
							strokeWidth: movie.isLiked ? 0 : 2,
						}}
					/>
				</button>
				<button
					type="button"
					onClick={(e) => {
						e.stopPropagation();
						onAddToWatchlist(movie.id);
					}}
					className={`rounded-full p-2 transition-colors duration-300 ${
						movie.inWatchlist
							? "bg-green-100 text-green-500"
							: "bg-neutral-800 text-neutral-300 hover:bg-neutral-700"
					}`}
					title={
						movie.inWatchlist ? "Remove from watchlist" : "Add to watchlist"
					}
				>
					{movie.inWatchlist ? (
						<Check className="h-5 w-5" />
					) : (
						<Plus className="h-5 w-5" />
					)}
				</button>
			</div>
		</div>
	);
};

export default MoviePoster;
