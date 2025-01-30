import type React from "react";
import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { Film, User, Clapperboard, Search, Popcorn } from "lucide-react";
import CategoryCarosel, {
	MOVIES_PER_PAGE,
} from "../components/CategoryCarousel";

interface MovieListing {
	id: number;
	title: string;
	poster: string;
	description: string;
	rank: number;
	overallScore: number;
	isLiked: boolean;
	inWatchlist: boolean;
}

interface MovieCategories {
	[key: string]: MovieListing[];
}

const staticMovieCategories = {
	"Top Rated": [],
	"New Releases": [],
};

const HomePage: React.FC = () => {
	const [movies, setMovies] = useState<MovieCategories>({
		"Trending Now": [],
		...staticMovieCategories,
	});
	const [activeNav, setActiveNav] = useState("Films");
	const [searchQuery, setSearchQuery] = useState("");
	const [isLoading, setIsLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);
	const [currentPage, setCurrentPage] = useState<Record<string, number>>(() =>
		Object.keys({ "Trending Now": [], ...staticMovieCategories }).reduce(
			// biome-ignore lint/performance/noAccumulatingSpread: <explanation>
			(acc, category) => ({ ...acc, [category]: 0 }),
			{},
		),
	);

	useEffect(() => {
		const fetchTrendingMovies = async () => {
			try {
				const response = await fetch("/api/v1/discover/trending");
				if (!response.ok) {
					throw new Error("Failed to fetch trending movies");
				}
				const data = await response.json();

				setMovies((prev) => ({
					...prev,
					"Trending Now": data.movies.map((movie: MovieListing) => ({
						...movie,
						rank: 0,
					})),
				}));
			} catch (err) {
				setError(
					err instanceof Error
						? err.message
						: "Failed to fetch trending movies",
				);
			} finally {
				setIsLoading(false);
			}
		};

		fetchTrendingMovies();
	}, []);

	const handleRankMovie = (movieId: number, rank: number) => {
		const updatedMovies = { ...movies };
		Object.keys(updatedMovies).forEach((category) => {
			updatedMovies[category] = updatedMovies[category].map((movie) =>
				movie.id === movieId ? { ...movie, rank } : movie,
			);
		});
		setMovies(updatedMovies);
	};

	const handleLikeMovie = (movieId: number) => {
		const updatedMovies = { ...movies };
		Object.keys(updatedMovies).forEach((category) => {
			updatedMovies[category] = updatedMovies[category].map((movie) =>
				movie.id === movieId ? { ...movie, isLiked: !movie.isLiked } : movie,
			);
		});
		setMovies(updatedMovies);
	};

	const handleAddToWatchlist = (movieId: number) => {
		const updatedMovies = { ...movies };
		Object.keys(updatedMovies).forEach((category) => {
			updatedMovies[category] = updatedMovies[category].map((movie) =>
				movie.id === movieId
					? { ...movie, inWatchlist: !movie.inWatchlist }
					: movie,
			);
		});
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
		{ name: "Films", icon: <Film className="h-5 w-5" /> },
		{ name: "Actors", icon: <User className="h-5 w-5" /> },
		{ name: "Directors", icon: <Clapperboard className="h-5 w-5" /> },
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
										to={`/${item.name.toLowerCase()}`}
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
							</div>
							<Link
								className="rounded-full p-2 transition duration-200 hover:bg-neutral-800"
								to="/profile"
							>
								<User className="h-6 w-6 text-purple-400 transition hover:text-purple-300" />
							</Link>
						</div>
					</div>
				</div>
			</nav>

			<main className="mx-auto flex-grow max-w-7xl px-6 pb-6 pt-20">
				{isLoading ? (
					<div className="flex items-center justify-center pt-20">
						<div className="text-lg text-neutral-400">
							Loading trending movies...
						</div>
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
