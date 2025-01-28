import React, { useState } from "react";
import { Link } from "react-router-dom";
import { Film, User, Clapperboard, Search, Popcorn } from "lucide-react";
import CategoryCarosel, {
	MOVIES_PER_PAGE,
} from "../components/CategoryCarousel";

const movieCategories = {
	"Trending Now": [
		{
			id: 1,
			title: "Carry-On",
			poster: "https://image.tmdb.org/t/p/w780/sjMN7DRi4sGiledsmllEw5HJjPy.jpg",
			description:
				"A young airline security guard is blackmailed by a mysterious passenger who threatens to smuggle a dangerous package onto a plane on Christmas Eve.",
			rank: 0,
			overallScore: 4.7,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 2,
			title: "The Matrix",
			poster:
				"https://www.themoviedb.org/t/p/w1280/dXNAPwY7VrqMAo51EKhhCJfaGb5.jpg",
			description: "A computer programmer discovers the world is a simulation.",
			rank: 0,
			overallScore: 4.5,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 3,
			title: "Interstellar",
			poster:
				"https://www.themoviedb.org/t/p/w1280/gEU2QniE6E77NI6lCU6MxlNBvIx.jpg",
			description: "A journey through space and time to save humanity.",
			rank: 0,
			overallScore: 5.0,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 4,
			title: "Inception",
			poster:
				"https://www.themoviedb.org/t/p/w1280/oYuLEt3zVCKq57qu2F8dT7NIa6f.jpg",
			description:
				"A thief who steals corporate secrets through dream-sharing technology.",
			rank: 0,
			overallScore: 4.8,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 5,
			title: "Dune",
			poster:
				"https://www.themoviedb.org/t/p/w1280/d5NXSklXo0qyIYkgV94XAgMIckC.jpg",
			description:
				"A noble family becomes embroiled in a war for control over the galaxy's most valuable asset.",
			rank: 0,
			overallScore: 4.6,
			isLiked: false,
			inWatchlist: false,
		},
	],
	"Top Rated": [
		{
			id: 6,
			title: "The Shawshank Redemption",
			poster:
				"https://www.themoviedb.org/t/p/w1280/9cqNxx0GxF0bflZmeSMuL5tnGzr.jpg",
			description: "A story of hope and friendship in prison.",
			rank: 0,
			overallScore: 4.9,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 7,
			title: "Knives Out",
			poster:
				"https://www.themoviedb.org/t/p/w1280/pThyQovXQrw2m0s9x82twj48Jq4.jpg",
			description: "A famous author is murdered. Who did it?",
			rank: 0,
			overallScore: 4.9,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 8,
			title: "The Godfather",
			poster:
				"https://www.themoviedb.org/t/p/w1280/3bhkrj58Vtu7enYsRolD1fZdja1.jpg",
			description:
				"The aging patriarch of an organized crime dynasty transfers control to his son.",
			rank: 0,
			overallScore: 5.0,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 9,
			title: "Pulp Fiction",
			poster:
				"https://www.themoviedb.org/t/p/w1280/d5iIlFn5s0ImszYzBPb8JPIfbXD.jpg",
			description:
				"Various interconnected stories of criminals in Los Angeles.",
			rank: 0,
			overallScore: 4.8,
			isLiked: false,
			inWatchlist: false,
		},
	],
	"New Releases": [
		{
			id: 10,
			title: "Oppenheimer",
			poster:
				"https://www.themoviedb.org/t/p/w1280/8Gxv8gSFCU0XGDykEGv7zR1n2ua.jpg",
			description:
				"The story of J. Robert Oppenheimer and the creation of the atomic bomb.",
			rank: 0,
			overallScore: 4.7,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 11,
			title: "Barbie",
			poster:
				"https://www.themoviedb.org/t/p/w1280/iuFNMS8U5cb6xfzi51Dbkovj7vM.jpg",
			description:
				"Barbie and Ken go on a journey of self-discovery in the real world.",
			rank: 0,
			overallScore: 4.5,
			isLiked: false,
			inWatchlist: false,
		},
		{
			id: 12,
			title: "Poor Things",
			poster:
				"https://www.themoviedb.org/t/p/w1280/kCGlIMHnOm8JPXq3rXM6c5wMxcT.jpg",
			description:
				"The story of Bella Baxter, a young woman brought back to life by an unorthodox scientist.",
			rank: 0,
			overallScore: 4.6,
			isLiked: false,
			inWatchlist: false,
		},
	],
};

const HomePage: React.FC = () => {
	const [movies, setMovies] = useState(movieCategories);
	const [activeNav, setActiveNav] = useState("Films");
	const [searchQuery, setSearchQuery] = useState("");
	const [currentPage, setCurrentPage] = useState<Record<string, number>>(() =>
		Object.keys(movieCategories).reduce(
			(acc, category) => ({ ...acc, [category]: 0 }),
			{},
		),
	);

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
				{Object.entries(movies).map(([category, categoryMovies]) => (
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
				))}
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
