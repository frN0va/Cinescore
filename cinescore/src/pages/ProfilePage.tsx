import type React from "react";
import { useState } from "react";
import { Link } from "react-router-dom";
import {
	User,
	Film,
	Clapperboard,
	Star,
	Popcorn,
	Search,
	ChevronLeft,
	ChevronRight,
	Trophy,
} from "lucide-react";

interface RankedMovie {
	rank: number;
	title: string;
	poster: string;
	overallScore?: number;
	isLiked?: boolean;
	inWatchlist?: boolean;
	id: number;
}

const MOVIES_PER_PAGE = 5;

const ProfilePage: React.FC = () => {
	const [activeNav, setActiveNav] = useState("Films");
	const [searchQuery, setSearchQuery] = useState("");
	const [currentPage, setCurrentPage] = useState<Record<string, number>>({
		"Top 5 Movies": 0,
		"My Liked Movies": 0,
		"My Watchlist": 0,
	});

	const [rankedMovies, setRankedMovies] = useState<RankedMovie[]>([
		{
			id: 1,
			rank: 1,
			title: "Interstellar - 2014",
			poster:
				"https://www.themoviedb.org/t/p/w1280/gEU2QniE6E77NI6lCU6MxlNBvIx.jpg",
			overallScore: 5.0,
		},
		{
			id: 2,
			rank: 2,
			title: "La La Land - 2016",
			poster:
				"https://www.themoviedb.org/t/p/w1280/uDO8zWDhfWwoFdKS4fzkUJt0Rf0.jpg",
			overallScore: 4.7,
		},
		{
			id: 3,
			rank: 3,
			title: "Back to the Future - 1985",
			poster:
				"https://www.themoviedb.org/t/p/w1280/rej4R5DIdlx29I2soNePfInICG3.jpg",
			overallScore: 4.8,
		},
		{
			id: 4,
			rank: 4,
			title: "Inception - 2010",
			poster:
				"https://www.themoviedb.org/t/p/w1280/9gk7adHYeDvHkCSEqAvQNLV5Uge.jpg",
			overallScore: 4.6,
		},
		{
			id: 5,
			rank: 5,
			title: "Knives Out - 2019",
			poster:
				"https://www.themoviedb.org/t/p/w1280/pThyQovXQrw2m0s9x82twj48Jq4.jpg",
			overallScore: 4.5,
		},
	]);

	const [likedMovies, setLikedMovies] = useState<RankedMovie[]>([
		{
			id: 6,
			rank: 0,
			title: "The Dark Knight - 2008",
			poster:
				"https://www.themoviedb.org/t/p/w1280/qJ2tW6WMUDux911r6m7haRef0WH.jpg",
			overallScore: 4.9,
			isLiked: true,
		},
		{
			id: 7,
			rank: 0,
			title: "Goodfellas - 1990",
			poster:
				"https://www.themoviedb.org/t/p/w1280/aKuFiU82s5ISJpGZp7YkIr3kCUd.jpg",
			overallScore: 4.8,
			isLiked: true,
		},
		{
			id: 8,
			rank: 0,
			title: "Pulp Fiction - 1994",
			poster:
				"https://www.themoviedb.org/t/p/w1280/d5iIlFn5s0ImszYzBPb8JPIfbXD.jpg",
			overallScore: 4.7,
			isLiked: true,
		},
		{
			id: 9,
			rank: 0,
			title: "The Shawshank Redemption - 1994",
			poster:
				"https://www.themoviedb.org/t/p/w1280/q6y0Go1tsGEsmtFryDOJo3dEmqu.jpg",
			overallScore: 4.9,
			isLiked: true,
		},
		{
			id: 10,
			rank: 0,
			title: "Forrest Gump - 1994",
			poster:
				"https://www.themoviedb.org/t/p/w1280/saHP97rTPS5eLmrLQEcANmKrsFl.jpg",
			overallScore: 4.8,
			isLiked: true,
		},
	]);

	const [watchlist, setWatchlist] = useState<RankedMovie[]>([
		{
			id: 11,
			rank: 0,
			title: "The Godfather - 1972",
			poster:
				"https://www.themoviedb.org/t/p/w1280/3bhkrj58Vtu7enYsRolD1fZdja1.jpg",
			overallScore: 4.9,
			inWatchlist: true,
		},
		{
			id: 12,
			rank: 0,
			title: "Fight Club - 1999",
			poster:
				"https://www.themoviedb.org/t/p/w1280/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg",
			overallScore: 4.7,
			inWatchlist: true,
		},
		{
			id: 13,
			rank: 0,
			title: "Schindler's List - 1993",
			poster:
				"https://www.themoviedb.org/t/p/w1280/sF1U4EUQS8YHUYjNl3pMGNIQyr0.jpg",
			overallScore: 4.9,
			inWatchlist: true,
		},
		{
			id: 14,
			rank: 0,
			title: "12 Angry Men - 1957",
			poster:
				"https://www.themoviedb.org/t/p/w1280/ppd84D2i9W8jXmsyInGyihiSyqz.jpg",
			overallScore: 4.8,
			inWatchlist: true,
		},
		{
			id: 15,
			rank: 0,
			title: "The Silence of the Lambs - 1991",
			poster:
				"https://www.themoviedb.org/t/p/w1280/uS9m8OBk1A8eM9I042bx8XXpqAq.jpg",
			overallScore: 4.7,
			inWatchlist: true,
		},
	]);

	const navItems = [
		{ name: "Films", icon: <Film className="w-5 h-5" />, to: "/" },
		{ name: "Actors", icon: <User className="w-5 h-5" />, to: "/actors" },
		{
			name: "Directors",
			icon: <Clapperboard className="w-5 h-5" />,
			to: "/directors",
		},
	];

	const handlePrevPage = (category: string) => {
		setCurrentPage((prev) => ({
			...prev,
			[category]: Math.max(0, prev[category] - 1),
		}));
	};

	const handleNextPage = (category: string) => {
		const maxPage =
			Math.ceil(
				(category === "Top 5 Movies"
					? rankedMovies
					: category === "My Liked Movies"
						? likedMovies
						: watchlist
				).length / MOVIES_PER_PAGE,
			) - 1;

		setCurrentPage((prev) => ({
			...prev,
			[category]: Math.min(maxPage, prev[category] + 1),
		}));
	};

	const TopFiveSection: React.FC<{ movies: RankedMovie[] }> = ({ movies }) => (
		<section className="mb-16 bg-neutral-900/50 p-8 rounded-xl shadow-xl">
			<div className="mb-8 flex items-center justify-between border-b border-neutral-800 pb-3">
				<div className="flex items-center gap-3">
					<Trophy className="w-7 h-7 text-purple-400" />
					<h2 className="text-3xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
						Top 5 Movies
					</h2>
				</div>
			</div>
			<div className="grid grid-cols-1 md:grid-cols-5 gap-6">
				{movies.map((movie) => (
					<div
						key={movie.id}
						className="group bg-neutral-900 rounded-lg overflow-hidden shadow-xl transform transition-all duration-300 hover:scale-105 hover:shadow-2xl border border-neutral-800 hover:border-purple-500/30"
					>
						<div className="relative">
							<img
								src={movie.poster}
								alt={movie.title}
								className="w-full h-[320px] object-cover transition-transform duration-300 group-hover:scale-105"
							/>
							<div className="absolute top-4 right-4 bg-black/70 text-yellow-400 px-3 py-1 rounded-full flex items-center">
								<Star className="w-5 h-5 mr-1" />
								<span className="font-bold">
									{movie.overallScore?.toFixed(1)}
								</span>
							</div>
							<div className="absolute top-4 left-4 bg-purple-500 text-white rounded-full w-12 h-12 flex items-center justify-center text-2xl font-bold shadow-lg border border-purple-400/30">
								#{movie.rank}
							</div>
						</div>
						<div className="p-4">
							<h2 className="text-lg font-semibold text-blue-300 line-clamp-2 group-hover:text-blue-200">
								{movie.title}
							</h2>
						</div>
					</div>
				))}
			</div>
		</section>
	);

	const CategoryCarousel: React.FC<{
		category: string;
		movies: RankedMovie[];
	}> = ({ category, movies }) => (
		<section key={category} className="mb-12">
			<div className="mb-6 flex items-center justify-between border-b border-neutral-800 pb-2">
				<h2 className="text-2xl font-bold text-blue-300">{category}</h2>
				<div className="flex space-x-2">
					<button
						type="button"
						className="rounded-full p-1 text-neutral-400 transition-colors hover:bg-neutral-800 hover:text-white"
						onClick={() => handlePrevPage(category)}
					>
						<ChevronLeft className="h-6 w-6" />
					</button>
					<button
						type="button"
						className="rounded-full p-1 text-neutral-400 transition-colors hover:bg-neutral-800 hover:text-white"
						onClick={() => handleNextPage(category)}
					>
						<ChevronRight className="h-6 w-6" />
					</button>
				</div>
			</div>
			<div className="grid grid-cols-1 md:grid-cols-5 gap-6">
				{movies
					.slice(
						currentPage[category] * MOVIES_PER_PAGE,
						(currentPage[category] + 1) * MOVIES_PER_PAGE,
					)
					.map((movie, index) => (
						<div
							key={movie.id}
							className="bg-neutral-900 rounded-lg overflow-hidden shadow-xl transform transition hover:scale-105 hover:shadow-2xl"
						>
							<div className="relative">
								<img
									src={movie.poster}
									alt={movie.title}
									className="w-full h-[300px] object-cover"
								/>
								{movie.overallScore && (
									<div className="absolute top-4 right-4 bg-black/70 text-yellow-400 px-3 py-1 rounded-full flex items-center">
										<Star className="w-5 h-5 mr-1" />
										<span className="font-bold">
											{movie.overallScore.toFixed(1)}
										</span>
									</div>
								)}
								{movie.rank > 0 && (
									<div className="absolute top-4 left-4 bg-purple-600 text-white rounded-full w-12 h-12 flex items-center justify-center text-2xl font-bold">
										#{movie.rank}
									</div>
								)}
							</div>
							<div className="p-4">
								<h2 className="text-lg font-semibold text-blue-300 line-clamp-2">
									{movie.title}
								</h2>
							</div>
						</div>
					))}
			</div>
		</section>
	);

	return (
		<div className="w-full min-h-screen bg-neutral-950 text-white">
			<nav className="fixed top-0 left-0 right-0 z-50 bg-neutral-900/90 backdrop-blur-sm shadow-lg">
				<div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
					<div className="flex items-center h-16">
						<div className="flex items-center space-x-8">
							<Link to="/" className="flex items-center">
								<Popcorn className="w-6 h-6 mr-2 text-purple-400" />
								<span className="text-lg font-bold text-white tracking-wider">
									Cinescore
								</span>
							</Link>
							<div className="flex space-x-4">
								{navItems.map((item) => (
									<Link
										key={item.name}
										to={`/${item.name.toLowerCase()}`}
										className={`flex items-center space-x-2 px-3 py-2 rounded-md transition-colors duration-300 ${
											activeNav === item.name
												? "bg-purple-400 text-white"
												: "text-neutral-300 hover:bg-neutral-800 hover:text-white"
										}`}
										onClick={() => setActiveNav(item.name)}
									>
										{item.icon}
										<span className="text-sm font-medium">{item.name}</span>
									</Link>
								))}
							</div>
						</div>
						<div className="flex items-center ml-auto space-x-4">
							<div className="relative">
								<div className="flex items-center bg-neutral-800/50 rounded-full pr-4">
									<div className="flex items-center pl-4 pr-2">
										<Search className="w-5 h-5 text-neutral-400" />
									</div>
									<input
										type="text"
										placeholder="Search Cinescore..."
										className="bg-transparent border-none focus:outline-none text-white py-2 w-64 placeholder-neutral-400"
										value={searchQuery}
										onChange={(e) => setSearchQuery(e.target.value)}
									/>
								</div>
							</div>
							<Link
								to="/"
								className="hover:bg-neutral-800 p-2 rounded-full transition duration-200"
							>
								<Film className="w-6 h-6 text-blue-400 hover:text-blue-300 transition" />
							</Link>
						</div>
					</div>
				</div>
			</nav>

			<main className="pt-20 px-6 pb-6 max-w-7xl mx-auto">
				{/* Profile Section */}
				<div className="mb-12 bg-neutral-900 rounded-lg p-8 flex flex-col md:flex-row gap-8">
					<div className="flex-shrink-0">
						<div className="w-48 h-48 rounded-full overflow-hidden bg-neutral-800">
							<img
								src="https://wallpapers.com/images/hd/cool-profile-picture-minion-13pu7815v42uvrsg.jpg"
								alt="Profile"
								className="w-full h-full object-cover"
							/>
						</div>
					</div>
					<div className="flex-grow">
						<h1 className="text-3xl font-bold text-blue-300 mb-2">N0va</h1>
						<div className="space-y-4">
							<p className="text-neutral-400">
								1/2 of the developer team. Movie & TV Lover.
							</p>
							<div className="flex gap-4">
								<div className="bg-neutral-800 px-4 py-2 rounded-lg">
									<span className="block text-sm text-neutral-400">
										Movies Rated
									</span>
									<span className="text-xl font-bold text-blue-300">247</span>
								</div>
								<div className="bg-neutral-800 px-4 py-2 rounded-lg">
									<span className="block text-sm text-neutral-400">
										Reviews Written
									</span>
									<span className="text-xl font-bold text-blue-300">125</span>
								</div>
								<div className="bg-neutral-800 px-4 py-2 rounded-lg">
									<span className="block text-sm text-neutral-400">
										Following
									</span>
									<span className="text-xl font-bold text-blue-300">7</span>
								</div>
								<div className="bg-neutral-800 px-4 py-2 rounded-lg">
									<span className="block text-sm text-neutral-400">
										Followers
									</span>
									<span className="text-xl font-bold text-blue-300">2094</span>
								</div>
							</div>
						</div>
					</div>
				</div>

				{/* Movie Categories */}
				<div className="space-y-12">
					<TopFiveSection movies={rankedMovies} />
					<CategoryCarousel category="My Liked Movies" movies={likedMovies} />
					<CategoryCarousel category="My Watchlist" movies={watchlist} />
				</div>
			</main>

			<footer className="bg-neutral-900 text-neutral-400 py-6 mt-auto text-center">
				<p>
					Created and Copyrighted by Owen Perry and Connor Sample. All Rights
					Reserved © 2025.
				</p>
			</footer>
		</div>
	);
};

export default ProfilePage;
