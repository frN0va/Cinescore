import type React from "react";
import { useDrag } from "react-dnd";
import { Film } from "lucide-react";
import type { Movie } from "../types";

interface MovieDraggableProps {
	movie: Movie;
	sourceTier: string;
	onMoveDrop: (movieId: number, sourceTier: string, targetTier: string) => void;
}

const MovieDraggable: React.FC<MovieDraggableProps> = ({
	movie,
	sourceTier,
}) => {
	const [{ isDragging }, drag] = useDrag({
		type: "movie",
		item: { id: movie.id, sourceTier },
		collect: (monitor) => ({
			isDragging: !!monitor.isDragging(),
		}),
	});

	return (
		<div
			ref={drag}
			className={`h-48 w-32 cursor-move rounded-md border-2 border-transparent transition-all duration-200 hover:border-purple-500 ${
				isDragging ? "opacity-50" : "opacity-100"
			}`}
		>
			{movie.poster ? (
				<img
					src={movie.poster}
					alt={movie.title}
					className="h-full w-full rounded-md object-cover"
					title={movie.title}
				/>
			) : (
				<div className="flex h-full w-full items-center justify-center rounded-md bg-neutral-700">
					<Film className="h-8 w-8 text-neutral-600" />
				</div>
			)}
		</div>
	);
};

export default MovieDraggable;
