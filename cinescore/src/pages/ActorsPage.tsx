import type React from "react";
import { useState, useEffect } from "react";
import { useParams, Link } from "react-router-dom";
import { addDays, format } from "date-fns";
import {
	Film,
	Heart,
	ChevronLeft,
	Calendar,
	Globe,
	Facebook,
	Instagram,
	Twitter,
} from "lucide-react";
import type { Movie, Credits } from "../types";

interface FrontendSocials {
	imdb?: string;
	facebook?: string;
	instagram?: string;
	tiktok?: string;
	twitter?: string;
}

interface FrontendPersonDetails {
	biography: string;
	birthday: string;
	deathday?: string;
	gender: number;
	id: number;
	knownForDepartment: string;
	name: string;
	placeOfBirth: string;
	iconUrl: string;
	credits?: Credits;
	socials?: FrontendSocials;
}

const ActorPage: React.FC = () => {
	const { id } = useParams();
	const [actor, setActor] = useState<FrontendPersonDetails | null>(null);
	const [isLoading, setIsLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);
	const [isLiked, setIsLiked] = useState(false);

	useEffect(() => {
		const fetchActorDetails = async () => {
			try {
				setIsLoading(true);
				const response = await fetch(`/api/v1/people/${id}`);
				if (!response.ok) {
					throw new Error("Failed to fetch actor details");
				}
				const data = await response.json();
				setActor(data);
			} catch (err) {
				setError(
					err instanceof Error ? err.message : "Failed to fetch actor details",
				);
			} finally {
				setIsLoading(false);
			}
		};

		fetchActorDetails();
	}, [id]);

	const getImageUrl = (path?: string) => {
		if (!path) return "/placeholder-person.jpg";
		return path;
	};

	const getFontSizeClass = (name: string) => {
		if (name.length > 25) return "text-lg";
		if (name.length > 20) return "text-xl";
		return "text-2xl";
	};

	if (isLoading) {
		return (
			<div className="flex min-h-screen items-center justify-center bg-neutral-950">
				<div className="text-lg text-neutral-400">Loading actor details...</div>
			</div>
		);
	}

	if (error || !actor) {
		return (
			<div className="flex min-h-screen items-center justify-center bg-neutral-950">
				<div className="text-lg text-red-400">{error || "Actor not found"}</div>
			</div>
		);
	}

	return (
		<div className="flex min-h-screen flex-col bg-neutral-950 text-white">
			<div className="mx-auto max-w-7xl px-8">
				<Link
					to="/actors"
					className="absolute left-8 top-8 flex items-center space-x-2 rounded-full bg-neutral-900/50 px-4 py-2 backdrop-blur-sm transition hover:bg-neutral-800"
				>
					<ChevronLeft className="h-5 w-5" />
					<span>Back to Actors</span>
				</Link>
			</div>

			<div className="relative mx-auto w-full max-w-8xl flex-grow px-8 py-8">
				<div className="grid grid-cols-1 gap-12 lg:grid-cols-5">
					<div className="space-y-8 lg:col-span-2">
						<div className="rounded-lg bg-neutral-900 p-6 shadow-xl">
							{/* Profile Image and Basic Info */}
							<div className="mb-6 flex items-start gap-4">
								<img
									src={getImageUrl(actor.iconUrl)}
									alt={actor.name}
									className="w-32 shrink-0 rounded-lg object-cover shadow-lg aspect-[2/3]"
								/>
								<div className="flex-1">
									<h1 className="text-2xl font-bold break-normal">
										{actor.name}
									</h1>
									<p className="mt-1 text-lg text-neutral-400">
										{actor.knownForDepartment}
									</p>

									{/* Social Media Links */}
									<div className="mt-4 flex space-x-3">
										{actor.socials?.facebook && (
											<a
												href={`https://facebook.com/${actor.socials.facebook.replace("@", "")}`}
												target="_blank"
												rel="noopener noreferrer"
												className="rounded-full bg-neutral-800 p-2 text-neutral-300 hover:bg-neutral-700 transition-colors"
											>
												<Facebook className="h-5 w-5" />
											</a>
										)}

										{actor.socials?.instagram && (
											<a
												href={`https://instagram.com/${actor.socials.instagram.replace("@", "")}`}
												target="_blank"
												rel="noopener noreferrer"
												className="rounded-full bg-neutral-800 p-2 text-neutral-300 hover:bg-neutral-700 transition-colors"
											>
												<Instagram className="h-5 w-5" />
											</a>
										)}
										{actor.socials?.twitter && (
											<a
												href={`https://twitter.com/${actor.socials.twitter.replace("@", "")}`}
												target="_blank"
												rel="noopener noreferrer"
												className="rounded-full bg-neutral-800 p-2 text-neutral-300 hover:bg-neutral-700 transition-colors"
											>
												<Twitter className="h-5 w-5" />
											</a>
										)}
									</div>
								</div>
							</div>

							{/* Like Button */}
							<div className="mb-6 flex space-x-2">
								<button
									type="button"
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
							</div>

							{/* Biography */}
							<div className="mb-6">
								<h2 className="mb-2 text-xl font-semibold">Biography</h2>
								<div className="max-h-72 overflow-y-auto">
									<p className="text-neutral-300">{actor.biography}</p>
								</div>
							</div>

							{/* Personal Info */}
							<div className="space-y-4">
								<div className="flex items-center space-x-3 text-neutral-300">
									<Calendar className="h-5 w-5" />
									<span>
										Born:{" "}
										{actor.birthday
											? format(
													addDays(new Date(actor.birthday), 1),
													"MMM d, yyyy",
												)
											: "Unknown"}
									</span>
								</div>
								{actor.deathday && (
									<div className="flex items-center space-x-3 text-neutral-300">
										<Calendar className="h-5 w-5" />
										<span>Died: {actor.deathday}</span>
									</div>
								)}
								<div className="flex items-center space-x-3 text-neutral-300">
									<Globe className="h-5 w-5" />
									<span>Place of Birth: {actor.placeOfBirth}</span>
								</div>
								{actor.socials?.imdb && (
									<div className="flex items-center space-x-3 text-neutral-300">
										<Film className="h-5 w-5" />
										<a
											href={`https://www.imdb.com/name/${actor.socials.imdb}`}
											target="_blank"
											rel="noopener noreferrer"
											className="!text-purple-400 hover:!text-purple-300"
										>
											View on IMDb
										</a>
									</div>
								)}
							</div>
						</div>
					</div>

					{/* Right Column - Filmography */}
					<div className="lg:col-span-3">
						{actor.credits && (
							<div className="rounded-lg bg-neutral-900 p-6 shadow-xl">
								<h2 className="mb-4 text-2xl font-bold">Filmography</h2>
								<div className="max-h-[800px] overflow-y-auto pr-4">
									<div className="grid grid-cols-2 gap-6 sm:grid-cols-3">
										{actor.credits.cast.map((movie) => (
											<Link
												key={movie.id || movie.title}
												to={`/movie/${movie.id}`}
												className="group relative overflow-hidden rounded-lg"
											>
												<img
													src={getImageUrl(movie.posterUrl)}
													alt={movie.title || "Movie Poster"}
													className="aspect-[2/3] w-full object-cover transition-transform duration-300 group-hover:scale-105"
												/>
												<div className="absolute inset-0 flex flex-col justify-end bg-gradient-to-t from-black/80 to-transparent p-4">
													<span className="font-medium text-white">
														{movie.title}
													</span>
													<span className="text-sm text-neutral-300">
														• {movie.character}
													</span>
												</div>
											</Link>
										))}
									</div>
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

export default ActorPage;
