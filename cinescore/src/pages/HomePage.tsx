import type React from "react";
import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { Film, User, Clapperboard, Search, Popcorn } from "lucide-react";
import CategoryCarosel, {
	MOVIES_PER_PAGE,
} from "../components/CategoryCarousel";
import { SearchDropdown } from "../components/SearchDropdown";
import type { Movie } from "../types";
import ProfileDropdown from "../components/ProfileDropdown";

interface MovieCategories {
	[key: string]: MovieListing[];
}

const staticMovieCategories = {
	"Controversial Ratings": [],
	"Upcoming Movies": [],
};

interface MovieListing {
	id: number;
	title: string;
	poster: string;
	description: string;
	rank: number;
	overallScore: number;
	isLiked: boolean;
	inWatchlist: boolean;
	releaseDate: string;
}

const HomePage: React.FC = () => {
	const [movies, setMovies] = useState<MovieCategories>({
		"Trending Now": [],
		"Now Playing": [],
		...staticMovieCategories,
	});
	const [activeNav, setActiveNav] = useState("Films");
	const [searchQuery, setSearchQuery] = useState("");
	const [searchResults, setSearchResults] = useState<Movie[]>([]);
	const [isSearching, setIsSearching] = useState(false);
	const [isLoading, setIsLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);
	const [currentPage, setCurrentPage] = useState<Record<string, number>>(() => {
		return Object.keys({
			"Trending Now": [],
			"Now Playing": [],
			...staticMovieCategories,
		}).reduce(
			(acc, category) => {
				acc[category] = 0;
				return acc;
			},
			{} as Record<string, number>,
		);
	});

	useEffect(() => {
		const fetchMovies = async () => {
			try {
				const [trendingResponse, nowPlayingResponse, upcomingResponse] =
					await Promise.all([
						fetch("/api/v1/discover/trending"),
						fetch("/api/v1/discover/now_playing"),
						fetch("/api/v1/discover/upcoming"),
					]);

				if (!trendingResponse.ok) {
					throw new Error("Failed to fetch trending movies");
				}
				if (!nowPlayingResponse.ok) {
					throw new Error("Failed to fetch now playing movies");
				}

				const trendingData = await trendingResponse.json();
				const nowPlayingData = await nowPlayingResponse.json();
				const upcomingData = await upcomingResponse.json();

				setMovies((prev) => ({
					...prev,
					"Trending Now": trendingData.movies.map((movie: MovieListing) => ({
						...movie,
						rank: 0,
					})),
					"Now Playing": nowPlayingData.movies.map((movie: MovieListing) => ({
						...movie,
						rank: 0,
					})),
					"Upcoming Movies": upcomingData.movies.map((movie: MovieListing) => ({
						...movie,
						rank: 0,
					})),
				}));
			} catch (err) {
				setError(err instanceof Error ? err.message : "Failed to fetch movies");
			} finally {
				setIsLoading(false);
			}
		};

		fetchMovies();
	}, []);

	// Search functionality
	useEffect(() => {
		const searchMovies = async () => {
			if (searchQuery.length < 2) {
				setSearchResults([]);
				return;
			}

			setIsSearching(true);
			try {
				const response = await fetch(
					`/api/v1/search/movies?query=${encodeURIComponent(searchQuery)}`,
				);
				if (!response.ok) throw new Error("Search failed");
				const data = await response.json();
				setSearchResults(data.movies);
			} catch (error) {
				console.error("Search error:", error);
				setSearchResults([]);
			} finally {
				setIsSearching(false);
			}
		};

		const debounceTimeout = setTimeout(searchMovies, 300);
		return () => clearTimeout(debounceTimeout);
	}, [searchQuery]);

	const handleRankMovie = (movieId: number, rank: number) => {
		const updatedMovies = { ...movies };
		for (const category of Object.keys(updatedMovies)) {
			updatedMovies[category] = updatedMovies[category].map((movie) =>
				movie.id === movieId ? { ...movie, rank } : movie,
			);
		}
		setMovies(updatedMovies);
	};

	const handleLikeMovie = (movieId: number) => {
		const updatedMovies = { ...movies };
		for (const category of Object.keys(updatedMovies)) {
			updatedMovies[category] = updatedMovies[category].map((movie) =>
				movie.id === movieId ? { ...movie, isLiked: !movie.isLiked } : movie,
			);
		}
		setMovies(updatedMovies);
	};

	const handleAddToWatchlist = (movieId: number) => {
		const updatedMovies = { ...movies };
		for (const category of Object.keys(updatedMovies)) {
			updatedMovies[category] = updatedMovies[category].map((movie) =>
				movie.id === movieId
					? { ...movie, inWatchlist: !movie.inWatchlist }
					: movie,
			);
		}
		setMovies(updatedMovies);
	};

	const handleNextPage = (category: string) => {
		const totalPages = Math.ceil(movies[category].length / MOVIES_PER_PAGE);
		setCurrentPage((prev) => ({
			...prev,
			[category]: (prev[category] + 1) % totalPages,
		}));
	};

	const handlePrevPage = (category: string) => {
		const totalPages = Math.ceil(movies[category].length / MOVIES_PER_PAGE);
		setCurrentPage((prev) => ({
			...prev,
			[category]: (prev[category] - 1 + totalPages) % totalPages,
		}));
	};

	const navItems = [
		{ name: "Films", icon: <Film className="h-5 w-5" />, to: "/" },
		{ name: "Actors", icon: <User className="h-5 w-5" />, to: "/actors" },
		{
			name: "Directors",
			icon: <Clapperboard className="h-5 w-5" />,
			to: "/directors",
		},
	];

	return (
		<div className="flex min-h-screen w-full flex-col bg-neutral-950 text-white">
			<nav className="fixed left-0 right-0 top-0 z-50 bg-neutral-900/90 shadow-lg backdrop-blur-sm">
				<div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
					<div className="flex h-16 items-center">
						<div className="flex items-center space-x-8">
							<Link className="flex items-center" to="/">
								<Popcorn className="mr-2 h-6 w-6 text-purple-400" />
								<span className="text-lg font-bold tracking-wider text-white">
									Cinescore
								</span>
							</Link>
							<div className="flex space-x-4">
								{navItems.map((item) => (
									<Link
										key={item.name}
										className={`flex items-center space-x-2 rounded-md px-3 py-2 transition-colors duration-300 ${
											activeNav === item.name
												? "bg-purple-400 text-white"
												: "text-neutral-300 hover:bg-neutral-800 hover:text-white"
										}`}
										onClick={() => setActiveNav(item.name)}
										to={item.to}
									>
										{item.icon}
										<span className="text-sm font-medium">{item.name}</span>
									</Link>
								))}
							</div>
						</div>
						<div className="ml-auto flex items-center space-x-4">
							<div className="relative">
								<div className="flex items-center rounded-full bg-neutral-800/50 pr-4">
									<div className="flex items-center pl-4 pr-2">
										<Search className="h-5 w-5 text-neutral-400" />
									</div>
									<input
										className="w-64 bg-transparent py-2 text-white placeholder-neutral-400 focus:outline-none"
										onChange={(e) => setSearchQuery(e.target.value)}
										placeholder="Search Cinescore..."
										type="text"
										value={searchQuery}
									/>
								</div>
								<SearchDropdown
									results={searchResults}
									isLoading={isSearching}
									searchQuery={searchQuery}
									onClose={() => setSearchQuery("")}
								/>
							</div>
							<ProfileDropdown
								isAuthenticated={false} // Replace with your auth state
								onLogout={() => {
									// Handle logout logic here
								}}
							/>
						</div>
					</div>
				</div>
			</nav>

			<main className="mx-auto flex-grow max-w-7xl px-6 pb-6 pt-20">
				{isLoading ? (
					<div className="flex items-center justify-center pt-20">
						<div className="text-lg text-neutral-400">Loading movies...</div>
					</div>
				) : error ? (
					<div className="flex items-center justify-center pt-20">
						<div className="text-lg text-red-400">{error}</div>
					</div>
				) : (
					Object.entries(movies).map(([category, categoryMovies]) => (
						<CategoryCarosel
							key={category}
							category={category}
							categoryMovies={categoryMovies}
							currentPage={currentPage}
							onRank={handleRankMovie}
							onLike={handleLikeMovie}
							onAddToWatchlist={handleAddToWatchlist}
							onPrevPage={handlePrevPage}
							onNextPage={handleNextPage}
						/>
					))
				)}
			</main>

			<footer className="mt-auto bg-neutral-900 py-6 text-center text-neutral-400">
				<p>
					Created and Copyrighted by Owen Perry and Connor Sample. All Rights
					Reserved &copy; 2025.
				</p>
			</footer>
		</div>
	);
};

export default HomePage;
