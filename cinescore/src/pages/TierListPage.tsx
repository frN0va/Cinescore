import type React from "react";
import { useState, useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import {
	Film,
	User,
	Clapperboard,
	Search,
	Popcorn,
	Trophy,
	ListChecks,
	Plus,
	X,
	Download,
} from "lucide-react";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import TierRow from "../components/TierRow";
import MovieDraggable from "../components/MovieDraggable";
import ProfileDropdown from "../components/ProfileDropdown";
import type { Movie } from "../types";

const tierLevels = [
	{ name: "S", color: "bg-red-500" },
	{ name: "A", color: "bg-orange-500" },
	{ name: "B", color: "bg-yellow-500" },
	{ name: "C", color: "bg-green-500" },
	{ name: "D", color: "bg-blue-500" },
	{ name: "F", color: "bg-purple-500" },
];

const TierListPage: React.FC = () => {
	const [activeNav, setActiveNav] = useState("Tier Lists");
	const [searchQuery, setSearchQuery] = useState("");
	const [tierListTitle, setTierListTitle] = useState("My Custom Tier List");
	const [isEditingTitle, setIsEditingTitle] = useState(false);
	const [movieSearchQuery, setMovieSearchQuery] = useState("");
	const [searchResults, setSearchResults] = useState<Movie[]>([]);
	const [isSearching, setIsSearching] = useState(false);
	const [movies, setMovies] = useState<Movie[]>([]);
	const [tierListMovies, setTierListMovies] = useState<Record<string, Movie[]>>({
		S: [],
		A: [],
		B: [],
		C: [],
		D: [],
		F: [],
		unassigned: [],
	});
	const [isLoading, setIsLoading] = useState(false);
	const [isExporting, setIsExporting] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const dropdownRef = useRef<HTMLDivElement>(null);
	const tierListRef = useRef<HTMLDivElement>(null);

	const navItems = [
		{ name: "Films", icon: <Film className="h-5 w-5" />, to: "/" },
		{ name: "Actors", icon: <User className="h-5 w-5" />, to: "/actors" },
		{
			name: "Directors",
			icon: <Clapperboard className="h-5 w-5" />,
			to: "/directors",
		},
		{
			name: "Tier Lists",
			icon: <Trophy className="h-5 w-5" />,
			to: "/tierlists",
		},
	];

	const handleMoveDrop = (
		movieId: number,
		sourceTier: string,
		targetTier: string,
	) => {
		if (sourceTier === targetTier) return;

		const newTierList = { ...tierListMovies };

		// Find the movie in the source tier
		const movieIndex = newTierList[sourceTier].findIndex(
			(m) => m.id === movieId,
		);
		if (movieIndex === -1) return;

		// Get the movie and remove it from the source tier
		const movie = newTierList[sourceTier][movieIndex];
		newTierList[sourceTier] = newTierList[sourceTier].filter(
			(m) => m.id !== movieId,
		);

		// Add the movie to the target tier
		newTierList[targetTier] = [...newTierList[targetTier], movie];

		setTierListMovies(newTierList);
	};

	const handleAddMovie = (movie: Movie) => {
		// Check if movie already exists in any tier
		const exists = Object.values(tierListMovies).some((tierMovies) =>
			tierMovies.some((m) => m.id === movie.id)
		);

		if (exists) {
			setError("This movie is already in your tier list");
			return;
		}

		// Add to unassigned movies
		setTierListMovies((prev) => ({
			...prev,
			unassigned: [...prev.unassigned, movie],
		}));

		// Add to overall movies list
		setMovies((prev) => [...prev, movie]);

		// Clear search query and results
		setMovieSearchQuery("");
		setSearchResults([]);
		setError(null);
	};

	const handleRemoveMovie = (movieId: number) => {
		// Remove from all tiers
		const newTierList = { ...tierListMovies };

		Object.keys(newTierList).forEach((tier) => {
			newTierList[tier] = newTierList[tier].filter((m) => m.id !== movieId);
		});

		setTierListMovies(newTierList);

		// Remove from overall movies list
		setMovies((prev) => prev.filter((m) => m.id !== movieId));
	};

	// Simplified download function that doesn't rely on HTML2Canvas
	const handleSaveTierList = async () => {
		setIsExporting(true);
		try {
			// Create a structured representation of the tier list data
			const tierListData = {
				title: tierListTitle,
				tiers: Object.entries(tierListMovies)
					.filter(([tier]) => tier !== "unassigned") // Exclude unassigned
					.map(([tier, movies]) => ({
						tier,
						movies: movies.map(movie => ({
							id: movie.id,
							title: movie.title,
						}))
					}))
			};

			// Convert to JSON string
			const jsonStr = JSON.stringify(tierListData, null, 2);
			
			// Create a blob and download
			const blob = new Blob([jsonStr], { type: "application/json" });
			const url = URL.createObjectURL(blob);
			const link = document.createElement("a");
			link.href = url;
			link.download = `${tierListTitle.replace(/\s+/g, "_")}_tierlist.json`;
			link.click();
			URL.revokeObjectURL(url);

			// Show success message
			setError(null);
		} catch (err) {
			console.error("Error exporting tier list:", err);
			setError("Failed to export tier list");
		} finally {
			setIsExporting(false);
		}
	};

	const handleResetTierList = () => {
		setTierListMovies({
			S: [],
			A: [],
			B: [],
			C: [],
			D: [],
			F: [],
			unassigned: [...movies],
		});
	};

	// For searching movies to add to the tier list
	useEffect(() => {
		const searchMovies = async () => {
			if (movieSearchQuery.length < 2) {
				setSearchResults([]);
				return;
			}

			setIsSearching(true);
			try {
				console.log(`Searching for: ${movieSearchQuery}`);
				const response = await fetch(
					`/api/v1/search/movies?query=${encodeURIComponent(movieSearchQuery)}`
				);
				
				if (!response.ok) {
					throw new Error("Search failed");
				}
				
				const data = await response.json();
				console.log("Search results:", data);
				setSearchResults(data.movies || []);
			} catch (error) {
				console.error("Search error:", error);
				setSearchResults([]);
			} finally {
				setIsSearching(false);
			}
		};

		const debounceTimeout = setTimeout(searchMovies, 300);
		return () => clearTimeout(debounceTimeout);
	}, [movieSearchQuery]);

	// Click outside handler for search dropdown
	useEffect(() => {
		const handleClickOutside = (event: MouseEvent) => {
			if (
				dropdownRef.current &&
				!dropdownRef.current.contains(event.target as Node)
			) {
				setMovieSearchQuery("");
			}
		};

		document.addEventListener("mousedown", handleClickOutside);

		return () => {
			document.removeEventListener("mousedown", handleClickOutside);
		};
	}, []);

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
							</div>
							<ProfileDropdown
								isAuthenticated={false}
								onLogout={() => {
									// Handle logout logic here
								}}
							/>
						</div>
					</div>
				</div>
			</nav>

			<main className="mx-auto w-full max-w-7xl flex-grow px-6 pb-8 pt-24">
				<DndProvider backend={HTML5Backend}>
					<div className="mb-6 flex flex-col">
						{/* Title with edit functionality */}
						<div className="mb-4 flex items-center">
							{isEditingTitle ? (
								<div className="flex items-center gap-2">
									<input
										type="text"
										value={tierListTitle}
										onChange={(e) => setTierListTitle(e.target.value)}
										className="rounded bg-neutral-800 px-2 py-1 text-2xl font-bold text-white"
										onBlur={() => setIsEditingTitle(false)}
										onKeyDown={(e) => {
											if (e.key === "Enter") setIsEditingTitle(false);
										}}
									/>
									<button
										type="button"
										onClick={() => setIsEditingTitle(false)}
										className="rounded bg-purple-600 px-2 py-1 text-sm font-medium text-white hover:bg-purple-500"
									>
										Save
									</button>
								</div>
							) : (
								<h2
									className="cursor-pointer text-2xl font-bold text-white"
									onClick={() => setIsEditingTitle(true)}
								>
									{tierListTitle}{" "}
									<span className="text-sm text-neutral-400">(click to edit)</span>
								</h2>
							)}
						</div>

						{/* Add movie by search */}
						<div className="relative mb-6 w-full max-w-md">
							<div className="rounded-lg bg-neutral-800 p-4">
								<label
									htmlFor="movie-search"
									className="block text-sm font-medium text-neutral-300"
								>
									Search for Movies to Add
								</label>
								<div className="mt-1 flex items-center rounded-md bg-neutral-700 px-3 py-1.5">
									<Search className="mr-2 h-5 w-5 text-neutral-400" />
									<input
										type="text"
										id="movie-search"
										value={movieSearchQuery}
										onChange={(e) => setMovieSearchQuery(e.target.value)}
										className="w-full bg-transparent text-white placeholder-neutral-400 focus:outline-none"
										placeholder="Search by movie title..."
									/>
									{movieSearchQuery && (
										<button
											type="button"
											className="text-neutral-400 hover:text-white"
											onClick={() => setMovieSearchQuery("")}
										>
											<X className="h-5 w-5" />
										</button>
									)}
								</div>
								{error && <p className="mt-1 text-sm text-red-500">{error}</p>}
							</div>

							{/* Search results dropdown */}
							{movieSearchQuery.length > 0 && (
								<div
									ref={dropdownRef}
									className="absolute left-0 right-0 z-10 mt-1 max-h-60 overflow-y-auto rounded-lg bg-neutral-800 shadow-xl"
								>
									{isSearching ? (
										<div className="p-4 text-center text-neutral-400">
											Searching...
										</div>
									) : searchResults.length === 0 ? (
										<div className="p-4 text-center text-neutral-400">
											No results found
										</div>
									) : (
										<div className="p-2">
											{searchResults.map((movie) => (
												<div
													key={movie.id}
													className="flex cursor-pointer items-center gap-3 rounded-md p-2 hover:bg-neutral-700"
													onClick={() => handleAddMovie(movie)}
												>
													{movie.poster ? (
														<img
															src={movie.poster}
															alt={movie.title}
															className="h-32 w-48 rounded object-cover"
														/>
													) : (
														<div className="flex h-32 w-48 items-center justify-center rounded bg-neutral-700">
															<Film className="h-10 w-10 text-neutral-400" />
														</div>
													)}
													<div className="flex flex-1 flex-col">
														<h4 className="text-sm font-medium text-white">
															{movie.title}
														</h4>
														<span className="text-xs text-neutral-400">
															{movie.releaseDate
																? new Date(movie.releaseDate).getFullYear()
																: "Unknown"}
														</span>
													</div>
													<button
														type="button"
														className="rounded-full bg-purple-500 p-1 text-white hover:bg-purple-400"
													>
														<Plus className="h-4 w-4" />
													</button>
												</div>
											))}
										</div>
									)}
								</div>
							)}
						</div>

						{/* Tier list actions */}
						<div className="mb-6 flex items-center justify-end space-x-4">
							<button
								type="button"
								onClick={handleResetTierList}
								className="rounded-md bg-neutral-700 px-4 py-2 text-sm font-medium text-white hover:bg-neutral-600"
							>
								Reset
							</button>
							<button
								type="button"
								onClick={handleSaveTierList}
								disabled={isExporting}
								className="flex items-center rounded-md bg-purple-600 px-4 py-2 text-sm font-medium text-white hover:bg-purple-500 disabled:opacity-50"
							>
								{isExporting ? (
									<>Exporting...</>
								) : (
									<>
										<Download className="mr-2 h-4 w-4" /> Save Tier List
									</>
								)}
							</button>
						</div>

						{/* Tier rows - this div is what gets captured for the PNG */}
						<div ref={tierListRef} className="mb-8 space-y-4">
							{tierLevels.map((tier) => (
								<TierRow
									key={tier.name}
									tier={tier.name}
									color={tier.color}
									movies={tierListMovies[tier.name]}
									onMovieDrop={(movieId: number, sourceTier: string) =>
										handleMoveDrop(movieId, sourceTier, tier.name)
									}
								/>
							))}
						</div>

						{/* Unassigned movies */}
						<div className="mt-8">
							<div className="mb-4 flex items-center justify-between">
								<div className="flex items-center">
									<ListChecks className="mr-2 h-5 w-5 text-neutral-400" />
									<h3 className="text-lg font-semibold text-white">
										Unassigned Movies
									</h3>
								</div>
							</div>
							<div className="grid grid-cols-2 gap-4 rounded-lg bg-neutral-800 p-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6">
								{tierListMovies.unassigned.map((movie) => (
									<div key={movie.id} className="relative">
										<MovieDraggable
											movie={movie}
											sourceTier="unassigned"
											onMoveDrop={handleMoveDrop}
										/>
										<button
											type="button"
											onClick={() => handleRemoveMovie(movie.id)}
											className="absolute -right-2 -top-2 flex h-6 w-6 items-center justify-center rounded-full bg-red-500 text-white hover:bg-red-600"
											title="Remove movie"
										>
											×
										</button>
									</div>
								))}
								{tierListMovies.unassigned.length === 0 && (
									<div className="col-span-full py-8 text-center text-neutral-400">
										Search for movies using the form above, then drag them to the
										appropriate tier
									</div>
								)}
							</div>
						</div>
					</div>
				</DndProvider>
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

export default TierListPage;
