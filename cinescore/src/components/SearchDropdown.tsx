import type React from "react";
import { Film } from "lucide-react";
import { Link } from "react-router-dom";
import { useEffect, useRef } from "react";
import type { Credits, Movie } from "../types";
import { format } from "date-fns";

interface SearchDropdownProps {
	results: Movie[];
	isLoading: boolean;
	searchQuery: string;
	onClose: () => void;
}

interface FrontendMovieDetails {
	backdropUrl: string;
	budget: number;
	id: number;
	imdbId: string;
	originalLanguage: string;
	overview: string;
	posterUrl: string;
	title: string;
	releaseDate: string;
	revenue: number;
	runtime: number;
	spokenLanguages: Array<{ name: string }>;
	tagline: string;
	rank?: number;
	overallScore?: number;
	credits?: Credits;
}

export const SearchDropdown: React.FC<SearchDropdownProps> = ({
	results,
	isLoading,
	searchQuery,
	onClose,
}) => {
	const dropdownRef = useRef<HTMLDivElement>(null);

	useEffect(() => {
		const handleClickOutside = (event: MouseEvent) => {
			if (
				dropdownRef.current &&
				!dropdownRef.current.contains(event.target as Node)
			) {
				onClose();
			}
		};

		document.addEventListener("mousedown", handleClickOutside);

		return () => {
			document.removeEventListener("mousedown", handleClickOutside);
		};
	}, [onClose]);

	if (searchQuery.length === 0) return null;

	return (
		<div
			ref={dropdownRef}
			className="absolute left-0 right-0 top-full mt-2 max-h-96 overflow-y-auto rounded-lg bg-neutral-800 shadow-xl"
		>
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
										{movie.releaseDate
											? format(new Date(movie.releaseDate), "yyyy")
											: "Unknown"}
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
