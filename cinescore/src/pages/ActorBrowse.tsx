import type React from "react";
import { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import { User, Search, Popcorn, Film, Clapperboard, Trophy } from "lucide-react";
import type { Crew } from "../types";

const ACTORS_PER_PAGE = 10;
const FEATURED_ACTORS_IDS = [206, 224513, 3223, 30614, 115440]; // Replace with actual actor IDs you want to feature

const ActorBrowse: React.FC = () => {
	const navigate = useNavigate();
	const [actors, setActors] = useState<{
		[key: string]: Crew[];
	}>({
		"Featured Actors": [],
		"Trending Now": [],
		"Search Results": [],
	});
	const [activeNav, setActiveNav] = useState("Actors");
	const [searchQuery, setSearchQuery] = useState("");
	const [searchResults, setSearchResults] = useState<Crew[]>([]);
	const [isLoading, setIsLoading] = useState(true);
	const [isSearching, setIsSearching] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [currentPage, setCurrentPage] = useState<Record<string, number>>({
		"Featured Actors": 0,
		"Trending Now": 0,
		"Search Results": 0,
	});

	useEffect(() => {
		const fetchActors = async () => {
			try {
				// Fetch trending actors
				const trendingResponse = await fetch(
					"/api/v1/discover/trending_people",
				);
				if (!trendingResponse.ok) {
					throw new Error("Failed to fetch trending actors");
				}
				const trendingData = await trendingResponse.json();

				// Fetch featured actors
				const featuredActorsPromises = FEATURED_ACTORS_IDS.map((id) =>
					fetch(`/api/v1/people/${id}`)
						.then((response) => {
							if (!response.ok) throw new Error(`Failed to fetch actor ${id}`);
							return response.json();
						})
						.catch((error) => {
							console.error(`Error fetching actor ${id}:`, error);
							return null;
						}),
				);

				const featuredActorsResults = await Promise.all(featuredActorsPromises);
				const featuredActors = featuredActorsResults.filter(
					(actor): actor is Crew => actor !== null,
				);

				setActors({
					"Featured Actors": featuredActors,
					"Trending Now": trendingData.people,
					"Search Results": [],
				});
			} catch (err) {
				setError(err instanceof Error ? err.message : "Failed to fetch actors");
			} finally {
				setIsLoading(false);
			}
		};

		fetchActors();
	}, []);

	// Search functionality
	useEffect(() => {
		const searchPeople = async () => {
			if (searchQuery.length < 2) {
				setSearchResults([]);
				return;
			}

			setIsSearching(true);
			try {
				const response = await fetch(
					`/api/v1/search/people?query=${encodeURIComponent(searchQuery)}`,
				);
				if (!response.ok) throw new Error("Search failed");
				const data = await response.json();

				// Update actors state to include search results
				setActors((prev) => ({
					...prev,
					"Search Results": data.people || [],
				}));

				setSearchResults(data.people || []);
			} catch (error) {
				console.error("Search error:", error);
				setSearchResults([]);
			} finally {
				setIsSearching(false);
			}
		};

		const debounceTimeout = setTimeout(searchPeople, 300);
		return () => clearTimeout(debounceTimeout);
	}, [searchQuery]);

	const handlePageChange = (category: string, direction: "next" | "prev") => {
		const totalPages = Math.ceil(
			(actors[category]?.length || 0) / ACTORS_PER_PAGE,
		);
		setCurrentPage((prev) => ({
			...prev,
			[category]:
				direction === "next"
					? (prev[category] + 1) % totalPages
					: (prev[category] - 1 + totalPages) % totalPages,
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
		{
			name: "Tier Lists",
			icon: <Trophy className="h-5 w-5" />,
			to: "/tierlist",
		},
	];

	const handleSearch = (e: React.KeyboardEvent<HTMLInputElement>) => {
		if (e.key === "Enter" && searchQuery.length > 1) {
			navigate(`/search/actors?q=${encodeURIComponent(searchQuery)}`);
		}
	};

	const ActorGrid: React.FC<{
		category: string;
		actors: Crew[];
	}> = ({ category, actors }) => {
		const startIdx = currentPage[category] * ACTORS_PER_PAGE;
		const displayedActors = actors.slice(startIdx, startIdx + ACTORS_PER_PAGE);

		return (
			<div className="mb-12">
				<div className="mb-4 flex items-center justify-between">
					<h2 className="text-2xl font-bold text-white">{category}</h2>
					{actors.length > ACTORS_PER_PAGE && (
						<div className="flex space-x-2">
							<button
								type="button"
								onClick={() => handlePageChange(category, "prev")}
								className="rounded-full bg-neutral-800 p-2 text-white hover:bg-neutral-700"
							>
								←
							</button>
							<button
								type="button"
								onClick={() => handlePageChange(category, "next")}
								className="rounded-full bg-neutral-800 p-2 text-white hover:bg-neutral-700"
							>
								→
							</button>
						</div>
					)}
				</div>
				{actors.length > 0 ? (
					<div className="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
						{displayedActors.map((actor) => (
							<Link
								key={actor.id}
								to={`/actors/${actor.id}`}
								className="group relative overflow-hidden rounded-lg transition-transform duration-200 hover:scale-105"
							>
								<div className="aspect-[2/3] w-full">
									<img
										src={actor.iconUrl || "/default-actor-placeholder.png"}
										alt={actor.name || "Unknown Actor"}
										className="h-full w-full object-cover"
									/>
								</div>
								<div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-4">
									<h3 className="text-sm font-bold text-white group-hover:text-purple-700">
										{actor.name || "Unknown"}
									</h3>
									<p className="text-xs text-neutral-400">{actor.department}</p>
								</div>
							</Link>
						))}
					</div>
				) : (
					<div className="text-center text-neutral-400">No actors found</div>
				)}
			</div>
		);
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
										onChange={(e) => setSearchQuery(e.target.value)}
										onKeyDown={handleSearch}
										placeholder="Search Actors..."
										type="text"
										value={searchQuery}
									/>
								</div>
								{searchQuery.length > 0 && (
									<div className="absolute left-0 right-0 top-full mt-2 max-h-96 overflow-y-auto rounded-lg bg-neutral-800 shadow-xl">
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
												{searchResults.slice(0, 5).map((actor) => (
													<Link
														key={actor.id}
														to={`/actors/${actor.id}`}
														className="flex items-center gap-3 rounded-md p-2 hover:bg-neutral-700"
													>
														<img
															src={
																actor.iconUrl ||
																"/default-actor-placeholder.png"
															}
															alt={actor.name || "Unknown Actor"}
															className="h-12 w-12 rounded-full object-cover"
														/>
														<div className="flex-1">
															<h4 className="text-sm font-medium text-white">
																{actor.name || "Unknown"}
															</h4>
															<p className="text-xs text-neutral-400">
																{actor.department}
															</p>
														</div>
													</Link>
												))}
												{searchResults.length > 5 && (
													<Link
														to={`/search/actors?q=${encodeURIComponent(searchQuery)}`}
														className="block border-t border-neutral-700 p-3 text-center text-sm text-purple-400 hover:bg-neutral-700"
													>
														View all {searchResults.length} results
													</Link>
												)}
											</div>
										)}
									</div>
								)}
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
						<div className="text-lg text-neutral-400">Loading actors...</div>
					</div>
				) : error ? (
					<div className="flex items-center justify-center pt-20">
						<div className="text-lg text-red-400">{error}</div>
					</div>
				) : (
					Object.entries(actors)
						.filter(([_, categoryActors]) => categoryActors.length > 0)
						.map(([category, categoryActors]) => (
							<ActorGrid
								key={category}
								category={category}
								actors={categoryActors}
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

export default ActorBrowse;
