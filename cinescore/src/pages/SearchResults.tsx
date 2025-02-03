import type React from "react";
import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { Film } from "lucide-react";
import type { Movie } from "../types";

const SearchResults: React.FC = () => {
	const [searchParams] = useSearchParams();
	const query = searchParams.get("q") || "";
	const [results, setResults] = useState<Movie[]>([]);
	const [isLoading, setIsLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);

	useEffect(() => {
		const fetchResults = async () => {
			if (!query) return;

			setIsLoading(true);
			setError(null);

			try {
				const response = await fetch(
					`/api/v1/search/movies?query=${encodeURIComponent(query)}`,
				);

				if (!response.ok) {
					throw new Error("Failed to fetch search results");
				}

				const data = await response.json();
				setResults(data.movies);
			} catch (err) {
				setError(err instanceof Error ? err.message : "An error occurred");
			} finally {
				setIsLoading(false);
			}
		};

		fetchResults();
	}, [query]);

	return (
		<div className="mx-auto max-w-7xl px-6 py-8">
			<h1 className="mb-6 text-2xl font-bold text-white">
				Search Results for "{query}"
			</h1>

			{isLoading ? (
				<div className="text-center text-neutral-400">Loading results...</div>
			) : error ? (
				<div className="text-center text-red-400">{error}</div>
			) : results.length === 0 ? (
				<div className="text-center text-neutral-400">No results found</div>
			) : (
				<div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
					{results.map((movie) => (
						<div
							key={movie.id}
							className="flex flex-col overflow-hidden rounded-lg bg-neutral-800"
						>
							{movie.poster ? (
								<img
									src={movie.poster}
									alt={movie.title}
									className="h-64 w-full object-cover"
								/>
							) : (
								<div className="flex h-64 items-center justify-center bg-neutral-700">
									<Film className="h-12 w-12 text-neutral-600" />
								</div>
							)}
							<div className="flex flex-1 flex-col p-4">
								<h2 className="mb-2 text-lg font-semibold text-white">
									{movie.title}
								</h2>
								<p className="text-sm text-neutral-400 line-clamp-3">
									{movie.description}
								</p>
								<div className="mt-4 flex items-center justify-between">
									<span className="text-sm text-purple-400">
										Score: {movie.overallScore}
									</span>
								</div>
							</div>
						</div>
					))}
				</div>
			)}
		</div>
	);
};

export default SearchResults;
