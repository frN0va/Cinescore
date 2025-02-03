import type React from "react";
import { Film } from "lucide-react";
import { Link } from "react-router-dom";
import type { Movie } from "../types";

interface SearchDropdownProps {
	results: Movie[];
	isLoading: boolean;
	searchQuery: string;
	onClose: () => void;
}

export const SearchDropdown: React.FC<SearchDropdownProps> = ({
	results,
	isLoading,
	searchQuery,
	onClose,
}) => {
	if (searchQuery.length === 0) return null;

	return (
		<div className="absolute left-0 right-0 top-full mt-2 max-h-96 overflow-y-auto rounded-lg bg-neutral-800 shadow-xl">
			{isLoading ? (
				<div className="p-4 text-center text-neutral-400">Searching...</div>
			) : results.length === 0 ? (
				<div className="p-4 text-center text-neutral-400">No results found</div>
			) : (
				<>
					<div className="p-2">
						{results.slice(0, 5).map((movie) => (
							<Link
								key={movie.id}
								to={`/movie/${movie.id}`}
								className="flex items-center gap-3 rounded-md p-2 hover:bg-neutral-700"
								onClick={onClose}
							>
								{movie.poster ? (
									<img
										src={movie.poster}
										alt={movie.title}
										className="h-12 w-8 rounded object-cover"
									/>
								) : (
									<div className="flex h-12 w-8 items-center justify-center rounded bg-neutral-700">
										<Film className="h-4 w-4 text-neutral-400" />
									</div>
								)}
								<div className="flex flex-1 flex-col">
									<h4 className="text-sm font-medium text-white">
										{movie.title}
									</h4>
									<span className="text-xs text-neutral-400">
										{new Date(movie.releaseDate).getFullYear()}
									</span>
								</div>
							</Link>
						))}
					</div>
					{results.length > 5 && (
						<Link
							to={`/search/?q=${encodeURIComponent(searchQuery)}`}
							className="block border-t border-neutral-700 p-3 text-center text-sm text-purple-400 hover:bg-neutral-700"
							onClick={onClose}
						>
							View all {results.length} results
						</Link>
					)}
				</>
			)}
		</div>
	);
};

export default SearchDropdown;
