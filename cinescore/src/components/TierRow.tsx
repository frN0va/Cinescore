import type React from "react";
import { useDrop } from "react-dnd";
import MovieDraggable from "./MovieDraggable";
import type { Movie } from "../types";

interface TierRowProps {
	tier: string;
	color: string;
	movies: Movie[];
	onMovieDrop: (movieId: number, sourceTier: string) => void;
}

const TierRow: React.FC<TierRowProps> = ({
	tier,
	color,
	movies,
	onMovieDrop,
}) => {
	const [{ isOver }, drop] = useDrop({
		accept: "movie",
		drop: (item: { id: number; sourceTier: string }) => {
			onMovieDrop(item.id, item.sourceTier);
		},
		collect: (monitor) => ({
			isOver: !!monitor.isOver(),
		}),
	});

	return (
		<div className="flex w-full">
			{/* Tier label */}
			<div
				className={`flex h-60 w-32 flex-shrink-0 items-center justify-center ${color} font-bold text-3xl text-white`}
			>
				{tier}
			</div>

			{/* Droppable area */}
			<div
				ref={drop}
				className={`flex flex-grow flex-wrap items-center gap-4 rounded-r-lg p-4 transition-colors ${
					isOver ? "bg-neutral-700" : "bg-neutral-800"
				}`}
			>
				{movies.length === 0 ? (
					<div className="flex h-16 w-full items-center justify-center text-neutral-500">
						Drop movies here
					</div>
				) : (
					movies.map((movie) => (
						<MovieDraggable
							key={movie.id}
							movie={movie}
							sourceTier={tier}
							onMoveDrop={onMovieDrop}
						/>
					))
				)}
			</div>
		</div>
	);
};

export default TierRow;
