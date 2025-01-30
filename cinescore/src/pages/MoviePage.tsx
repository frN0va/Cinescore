import type React from "react";
import { useState, useEffect } from "react";
import { useParams, Link } from "react-router-dom";
import {
	Film,
	Star,
	Heart,
	Plus,
	Check,
	ChevronLeft,
	Clock,
	Calendar,
	Users,
} from "lucide-react";
import type { Movie } from "../types";

interface MovieDetails extends Movie {
	poster: any;
	releaseDate: string;
	runtime: string;
	director: string;
	cast: string[];
	genres: string[];
	similarMovies: Movie[];
}

const MoviePage: React.FC = () => {
	const { id } = useParams();
	const [movie, setMovie] = useState<MovieDetails | null>(null);
	const [isLoading, setIsLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);

	useEffect(() => {
		const fetchMovieDetails = async () => {
			try {
				setIsLoading(true);
				const response = await fetch(`/api/v1/movies/${id}`);
				if (!response.ok) {
					throw new Error("Failed to fetch movie details");
				}
				const data = await response.json();
				setMovie(data);
			} catch (err) {
				setError(
					err instanceof Error ? err.message : "Failed to fetch movie details",
				);
			} finally {
				setIsLoading(false);
			}
		};

		fetchMovieDetails();
	}, [id]);

	if (isLoading) {
		return (
			<div className="flex min-h-screen items-center justify-center bg-neutral-950">
				<div className="text-lg text-neutral-400">Loading movie details...</div>
			</div>
		);
	}

	if (error || !movie) {
		return (
			<div className="flex min-h-screen items-center justify-center bg-neutral-950">
				<div className="text-lg text-red-400">{error || "Movie not found"}</div>
			</div>
		);
	}

	return (
		<div className="min-h-screen bg-neutral-950 text-white">
			{/* Hero Section with Backdrop */}
			<div
				className="relative h-[70vh] w-full bg-cover bg-center"
				style={{ backgroundImage: `url(${movie.poster})` }}
			>
				<div className="absolute inset-0 bg-gradient-to-t from-neutral-950 to-transparent" />
				<div className="absolute inset-0 bg-gradient-to-r from-neutral-950/90 to-transparent" />

				{/* Back Button */}
				<Link
					to="/"
					className="absolute left-8 top-8 flex items-center space-x-2 rounded-full bg-neutral-900/50 px-4 py-2 backdrop-blur-sm transition hover:bg-neutral-800"
				>
					<ChevronLeft className="h-5 w-5" />
					<span>Back to Browse</span>
				</Link>
			</div>

			{/* Movie Details */}
			<div className="mx-auto max-w-7xl px-8 py-12">
				<div className="grid grid-cols-1 gap-12 lg:grid-cols-3">
					{/* Left Column */}
					<div className="space-y-8">
						<div className="rounded-lg bg-neutral-900 p-6">
							<h1 className="mb-4 text-4xl font-bold">{movie.title}</h1>

							<div className="mb-6 flex items-center space-x-4">
								<div className="flex items-center text-yellow-400">
									<Star className="mr-1 h-6 w-6 fill-current" />
									<span className="text-2xl font-bold">
										{movie.overallScore.toFixed(1)}
									</span>
								</div>

								<div className="flex space-x-2">
									<button
										className={`rounded-full p-3 transition-colors duration-300 ${
											movie.isLiked
												? "bg-red-100 text-red-500"
												: "bg-neutral-800 text-neutral-300 hover:bg-neutral-700"
										}`}
									>
										<Heart
											className={`h-6 w-6 ${movie.isLiked ? "fill-current" : ""}`}
										/>
									</button>
									<button
										className={`rounded-full p-3 transition-colors duration-300 ${
											movie.inWatchlist
												? "bg-green-100 text-green-500"
												: "bg-neutral-800 text-neutral-300 hover:bg-neutral-700"
										}`}
									>
										{movie.inWatchlist ? (
											<Check className="h-6 w-6" />
										) : (
											<Plus className="h-6 w-6" />
										)}
									</button>
								</div>
							</div>

							<p className="text-lg text-neutral-300">{movie.description}</p>
						</div>

						<div className="space-y-4 rounded-lg bg-neutral-900 p-6">
							<div className="flex items-center space-x-3 text-neutral-300">
								<Calendar className="h-5 w-5" />
								<span>Released: {movie.releaseDate}</span>
							</div>
							<div className="flex items-center space-x-3 text-neutral-300">
								<Clock className="h-5 w-5" />
								<span>Runtime: {movie.runtime}</span>
							</div>
							<div className="flex items-center space-x-3 text-neutral-300">
								<Film className="h-5 w-5" />
								<span>Director: {movie.director}</span>
							</div>
							<div className="flex items-center space-x-3 text-neutral-300">
								<Users className="h-5 w-5" />
								<span>Cast: {movie.cast.join(", ")}</span>
							</div>
						</div>
					</div>

					{/* Right Column */}
					<div className="lg:col-span-2">
						<div className="rounded-lg bg-neutral-900 p-6">
							<h2 className="mb-4 text-2xl font-bold">Similar Movies</h2>
							<div className="grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4">
								{movie.similarMovies.slice(0, 8).map((similar) => (
									<Link
										key={similar.id}
										to={`/movie/${similar.id}`}
										className="group relative overflow-hidden rounded-lg"
									>
										<img
											src={similar.poster}
											alt={similar.title}
											className="w-full transition-transform duration-300 group-hover:scale-105"
										/>
										<div className="absolute inset-0 flex items-end bg-gradient-to-t from-black/80 to-transparent p-4 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
											<span className="text-sm font-medium">
												{similar.title}
											</span>
										</div>
									</Link>
								))}
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	);
};

export default MoviePage;
