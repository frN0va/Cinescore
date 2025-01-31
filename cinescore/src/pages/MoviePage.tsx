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
	DollarSign,
	Globe,
} from "lucide-react";

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
	credits?: {
		cast: Array<{
			name: string;
			iconUrl: string;
			character: string;
			id: number;
		}>;
		crew: Array<{
			name: string;
			iconUrl: string;
			department: string;
			id: number;
		}>;
	};
}

const MoviePage: React.FC = () => {
	const { id } = useParams();
	const [movie, setMovie] = useState<FrontendMovieDetails | null>(null);
	const [isLoading, setIsLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);
	const [isLiked, setIsLiked] = useState(false);
	const [inWatchlist, setInWatchlist] = useState(false);

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

	const director = movie.credits?.crew.find(
		(member) => member.department === "Directing",
	);

	return (
		<div className="flex min-h-screen flex-col bg-neutral-950 text-white">
			{/* Hero Section with Backdrop */}
			<div
				className="relative h-[60vh] w-full bg-cover bg-center"
				style={{ backgroundImage: `url(${movie.backdropUrl})` }}
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

			{/* Movie Details - Moved up and overlapping with backdrop */}
			<div className="relative mx-auto max-w-7xl flex-grow px-8 pb-24">
				<div
					className="grid grid-cols-1 gap-12 lg:grid-cols-3"
					style={{ marginTop: "-120px" }}
				>
					{/* Left Column */}
					<div className="space-y-8">
						<div className="rounded-lg bg-neutral-900 p-6 shadow-xl">
							<h1 className="mb-4 text-4xl font-bold">{movie.title}</h1>
							{movie.tagline && (
								<p className="mb-4 text-lg italic text-neutral-400">
									{movie.tagline}
								</p>
							)}

							<div className="mb-6 flex items-center space-x-4">
								<div className="flex space-x-2">
									<button
										onClick={() => setIsLiked(!isLiked)}
										className={`rounded-full p-3 transition-colors duration-300 ${
											isLiked
												? "bg-red-100 text-red-500"
												: "bg-neutral-800 text-neutral-300 hover:bg-neutral-700"
										}`}
									>
										<Heart
											className={`h-6 w-6 ${isLiked ? "fill-current" : ""}`}
										/>
									</button>
									<button
										onClick={() => setInWatchlist(!inWatchlist)}
										className={`rounded-full p-3 transition-colors duration-300 ${
											inWatchlist
												? "bg-green-100 text-green-500"
												: "bg-neutral-800 text-neutral-300 hover:bg-neutral-700"
										}`}
									>
										{inWatchlist ? (
											<Check className="h-6 w-6" />
										) : (
											<Plus className="h-6 w-6" />
										)}
									</button>
								</div>
							</div>

							<p className="text-lg text-neutral-300">{movie.overview}</p>
						</div>

						<div className="space-y-4 rounded-lg bg-neutral-900 p-6 shadow-xl">
							<div className="flex items-center space-x-3 text-neutral-300">
								<Calendar className="h-5 w-5" />
								<span>Released: {movie.releaseDate}</span>
							</div>
							<div className="flex items-center space-x-3 text-neutral-300">
								<Clock className="h-5 w-5" />
								<span>Runtime: {movie.runtime} minutes</span>
							</div>
							{director && (
								<div className="flex items-center space-x-3 text-neutral-300">
									<Film className="h-5 w-5" />
									<span>Director: {director.name}</span>
								</div>
							)}
							<div className="flex items-center space-x-3 text-neutral-300">
								<Globe className="h-5 w-5" />
								<span>
									Languages:{" "}
									{movie.spokenLanguages.map((lang) => lang.name).join(", ")}
								</span>
							</div>
							<div className="flex items-center space-x-3 text-neutral-300">
								<DollarSign className="h-5 w-5" />
								<span>Budget: ${movie.budget.toLocaleString()}</span>
							</div>
							<div className="flex items-center space-x-3 text-neutral-300">
								<DollarSign className="h-5 w-5" />
								<span>Revenue: ${movie.revenue.toLocaleString()}</span>
							</div>
						</div>
					</div>

					{/* Right Column */}
					<div className="lg:col-span-2">
						{movie.credits && (
							<div className="rounded-lg bg-neutral-900 p-6 shadow-xl">
								<h2 className="mb-4 text-2xl font-bold">Cast</h2>
								<div className="grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4">
									{movie.credits.cast.slice(0, 8).map((actor) => (
										<div
											key={actor.id}
											className="group relative overflow-hidden rounded-lg cursor-pointer"
										>
											<img
												src={actor.iconUrl}
												alt={actor.name}
												className="w-full transition-transform duration-300 group-hover:scale-105"
											/>
											<div className="absolute inset-0 flex flex-col justify-end bg-gradient-to-t from-black/80 to-transparent p-4">
												<span className="font-medium">{actor.name}</span>
												<span className="text-sm text-neutral-300">
													{actor.character}
												</span>
											</div>
										</div>
									))}
								</div>
							</div>
						)}
					</div>
				</div>
			</div>

			{/* Footer */}
			<footer className="bg-neutral-900 py-6 text-center text-neutral-400">
				<p>
					Created and Copyrighted by Owen Perry and Connor Sample. All Rights
					Reserved © 2025.
				</p>
			</footer>
		</div>
	);
};

export default MoviePage;
