import type React from "react";
import { useState, useEffect } from "react";
import { User, Search, Film, Clapperboard, Popcorn, X } from "lucide-react";
import { Link } from "react-router-dom";
import { ActorsCard, Modal } from "../components/ActorComponents";

interface Actor {
	id: number;
	name: string;
	image: string;
	birthYear: number;
	nationality: string;
	knownFor: string;
}

interface Movie {
	id: number;
	title: string;
	year: number;
	poster: string;
	role: string;
}

interface ActorDetails extends Actor {
	biography: string;
	filmography: Movie[];
}

const ActorsPage: React.FC = () => {
	const [actors, setActors] = useState<Actor[]>([]);
	const [selectedActor, setSelectedActor] = useState<ActorDetails | null>(null);
	const [isLoading, setIsLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);
	const [searchQuery, setSearchQuery] = useState("");
	const [activeNav, setActiveNav] = useState("Actors");

	useEffect(() => {
		const fetchActors = async () => {
			try {
				const response = await fetch("/api/v1/actors/popular");
				if (!response.ok) throw new Error("Failed to fetch actors");
				const data = await response.json();
				setActors(data.actors);
			} catch (err) {
				setError(err instanceof Error ? err.message : "Failed to fetch actors");
			} finally {
				setIsLoading(false);
			}
		};

		fetchActors();
	}, []);

	const handleActorClick = async (actorId: number) => {
		try {
			const response = await fetch(`/api/v1/actors/${actorId}`);
			if (!response.ok) throw new Error("Failed to fetch actor details");
			const data = await response.json();
			setSelectedActor(data);
		} catch (err) {
			setError(
				err instanceof Error ? err.message : "Failed to fetch actor details",
			);
		}
	};

	const navItems = [
		{ name: "Films", icon: <Film className="h-5 w-5" /> },
		{ name: "Actors", icon: <User className="h-5 w-5" /> },
		{ name: "Directors", icon: <Clapperboard className="h-5 w-5" /> },
	];

	const filteredActors = actors.filter((actor) =>
		actor.name.toLowerCase().includes(searchQuery.toLowerCase()),
	);

	return (
		<div className="flex min-h-screen w-full flex-col bg-neutral-950 text-white">
			{/* Navigation Bar */}
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
										placeholder="Search actors..."
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

			{/* Main Content */}
			<main className="mx-auto flex-grow max-w-7xl px-6 pb-6 pt-24">
				<h1 className="mb-8 text-3xl font-bold">Popular Actors</h1>

				{isLoading ? (
					<div className="flex items-center justify-center pt-20">
						<div className="text-lg text-neutral-400">Loading actors...</div>
					</div>
				) : error ? (
					<div className="flex items-center justify-center pt-20">
						<div className="text-lg text-red-400">{error}</div>
					</div>
				) : (
					<div className="grid grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
						{filteredActors.map((actor) => (
							<ActorsCard
								key={actor.id}
								actor={actor}
								onClick={handleActorClick}
							/>
						))}
					</div>
				)}
			</main>

			{/* Actor Details Modal */}
			<Modal isOpen={!!selectedActor} onClose={() => setSelectedActor(null)}>
				{selectedActor && (
					<div className="relative rounded-lg bg-neutral-900 shadow-xl">
						{/* biome-ignore lint/a11y/useButtonType: <explanation> */}
						<button
							onClick={() => setSelectedActor(null)}
							className="absolute right-4 top-4 rounded-full bg-neutral-800 p-2 text-white hover:bg-neutral-700"
						>
							<X className="h-5 w-5" />
						</button>
						<div className="flex flex-col md:flex-row">
							<div className="md:w-1/3">
								<img
									src={selectedActor.image}
									alt={selectedActor.name}
									className="h-full w-full rounded-t-lg object-cover md:rounded-l-lg md:rounded-t-none"
								/>
							</div>
							<div className="flex-1 overflow-y-auto p-6">
								<h2 className="text-2xl font-bold">{selectedActor.name}</h2>
								<p className="mt-2 text-neutral-400">
									Born: {selectedActor.birthYear} • {selectedActor.nationality}
								</p>
								<p className="mt-4">{selectedActor.biography}</p>

								<h3 className="mt-6 text-xl font-semibold">Filmography</h3>
								<div className="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
									{selectedActor.filmography.map((movie) => (
										<Link
											key={movie.id}
											to={`/films/${movie.id}`}
											className="group relative overflow-hidden rounded-lg bg-neutral-800"
										>
											<img
												src={movie.poster}
												alt={movie.title}
												className="aspect-[2/3] w-full object-cover transition-transform group-hover:scale-105"
											/>
											<div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />
											<div className="absolute bottom-0 p-3">
												<h4 className="font-semibold text-white">
													{movie.title}
												</h4>
												<p className="text-sm text-neutral-400">
													{movie.year} • {movie.role}
												</p>
											</div>
										</Link>
									))}
								</div>
							</div>
						</div>
					</div>
				)}
			</Modal>

			{/* Footer */}
			<footer className="mt-auto bg-neutral-900 py-6 text-center text-neutral-400">
				<p>
					Created and Copyrighted by Owen Perry and Connor Sample. All Rights
					Reserved &copy; 2025.
				</p>
			</footer>
		</div>
	);
};

export default ActorsPage;
