import type React from "react";
import { useEffect, useState } from "react";
import { useSearchParams, Link, useNavigate } from "react-router-dom";
import { Film, User, Clapperboard, Search, Popcorn, Star } from "lucide-react";
import type { Movie } from "../types";

const SearchResults: React.FC = () => {
	const [searchParams] = useSearchParams();
	const navigate = useNavigate();
	const query = searchParams.get("q") || "";
	const [results, setResults] = useState<Movie[]>([]);
	const [isLoading, setIsLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [activeNav, setActiveNav] = useState("Films");

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

	const navItems = [
		{ name: "Films", icon: <Film className="h-5 w-5" />, to: "/" },
		{ name: "Actors", icon: <User className="h-5 w-5" />, to: "/actors" },
		{
			name: "Directors",
			icon: <Clapperboard className="h-5 w-5" />,
			to: "/directors",
		},
	];

	const handleMovieClick = (movieId: number) => {
		navigate(`/movie/${movieId}`);
	};

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
										placeholder="Search Cinescore..."
										type="text"
										defaultValue={query}
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

			<main className="mx-auto w-full max-w-7xl flex-grow px-6 pb-6 pt-24">
				<div className="mb-8">
					<h1 className="text-2xl font-bold text-white">
						Showing Matches for "{query}"
					</h1>
				</div>

				<div className="flex gap-8">
					{/* Main content - Vertical list of movies */}
					<div className="flex-grow">
						{isLoading ? (
							<div className="flex items-center justify-center pt-20">
								<div className="text-lg text-neutral-400">
									Loading results...
								</div>
							</div>
						) : error ? (
							<div className="flex items-center justify-center pt-20">
								<div className="text-lg text-red-400">{error}</div>
							</div>
						) : results.length === 0 ? (
							<div className="flex items-center justify-center pt-20">
								<div className="text-lg text-neutral-400">No results found</div>
							</div>
						) : (
							<div className="space-y-6">
								{results.map((movie) => (
									<div
										key={movie.id}
										onClick={() => handleMovieClick(movie.id)}
										className="group flex cursor-pointer gap-6 rounded-lg bg-neutral-800/50 p-4 transition-colors hover:bg-neutral-800"
									>
										<div className="relative h-48 w-32 flex-shrink-0">
											{movie.poster ? (
												<img
													src={movie.poster}
													alt={movie.title}
													className="h-full w-full rounded-md object-cover"
												/>
											) : (
												<div className="flex h-full w-full items-center justify-center rounded-md bg-neutral-700">
													<Film className="h-12 w-12 text-neutral-600" />
												</div>
											)}
											<div className="absolute right-2 top-2 flex items-center rounded-full bg-black/70 px-2 py-1">
												<Star className="mr-1 h-4 w-4 text-yellow-400" />
												<span className="font-bold">{movie.overallScore.toFixed(1)}</span>
											</div>
										</div>
										<div className="flex flex-grow flex-col">
											<h2 className="text-xl font-semibold text-white group-hover:text-purple-400">
												{movie.title}
											</h2>
											<p className="mt-2 text-sm text-neutral-400">
												{movie.description}
											</p>
										</div>
									</div>
								))}
							</div>
						)}
					</div>

					{/* Sidebar filters */}
					<div className="w-64 flex-shrink-0">
						<div className="rounded-lg bg-neutral-800/50 p-4">
							<h2 className="mb-4 font-semibold text-white">
								SHOW RESULTS FOR
							</h2>
							<div className="space-y-2">
								<button type="button" className="w-full rounded-md bg-purple-400 px-4 py-2 text-left text-sm font-medium text-white">
									All
								</button>
								<button type="button" className="w-full rounded-md px-4 py-2 text-left text-sm font-medium text-neutral-400 transition-colors hover:bg-neutral-700 hover:text-white">
									Films
								</button>
								<button type="button" className="w-full rounded-md px-4 py-2 text-left text-sm font-medium text-neutral-400 transition-colors hover:bg-neutral-700 hover:text-white">
									Reviews
								</button>
								<button type="button" className="w-full rounded-md px-4 py-2 text-left text-sm font-medium text-neutral-400 transition-colors hover:bg-neutral-700 hover:text-white">
									Lists
								</button>
								<button type="button" className="w-full rounded-md px-4 py-2 text-left text-sm font-medium text-neutral-400 transition-colors hover:bg-neutral-700 hover:text-white">
									Members
								</button>
							</div>
						</div>
					</div>
				</div>
			</main>

			<footer className="mt-auto bg-neutral-900 py-6 text-center text-neutral-400">
				<p>
					Created and Copyrighted by Owen Perry and Connor Sample. All Rights
					Reserved © 2025.
				</p>
			</footer>
		</div>
	);
};

export default SearchResults;
