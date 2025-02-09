import type React from "react";
import { useState } from "react";
import { Link } from "react-router-dom";
import {
	Popcorn,
	Mail,
	Lock,
	Film,
	User,
	Clapperboard,
	Search,
	UserPlus,
} from "lucide-react";
import { FaGoogle, FaGithub } from "react-icons/fa";
import { SearchDropdown } from "../components/SearchDropdown";
import ProfileDropdown from "../components/ProfileDropdown";
import type { Movie } from "../types";

const SignUpPage = () => {
	const [email, setEmail] = useState("");
	const [password, setPassword] = useState("");
	const [confirmPassword, setConfirmPassword] = useState("");
	const [username, setUsername] = useState("");
	const [activeNav, setActiveNav] = useState("Films");
	const [searchQuery, setSearchQuery] = useState("");
	const [searchResults] = useState<Movie[]>([]);
	const [isSearching] = useState(false);

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault();
		// Handle sign up logic here
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
		<div className="min-h-screen bg-neutral-950 flex flex-col">
			{/* Navbar */}
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
							<ProfileDropdown isAuthenticated={false} />
						</div>
					</div>
				</div>
			</nav>

			{/* Sign Up Content */}
			<div className="flex-grow flex items-center justify-center px-4 py-12 mt-16">
				<div className="max-w-md w-full space-y-8">
					{/* Sign Up Header */}
					<div className="text-center">
						<h2 className="text-3xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
							Join Cinescore
						</h2>
						<p className="mt-2 text-neutral-400">
							Create an account to start rating and reviewing films!
						</p>
					</div>

					{/* Sign Up Form */}
					<div className="bg-neutral-900/50 p-8 rounded-xl shadow-xl border border-neutral-800">
						<form onSubmit={handleSubmit} className="space-y-6">
							<div>
								<label
									htmlFor="username"
									className="block text-sm font-medium text-neutral-300 mb-2"
								>
									Username
								</label>
								<div className="relative">
									<div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
										<UserPlus className="h-5 w-5 text-neutral-500" />
									</div>
									<input
										id="username"
										type="text"
										required
										value={username}
										onChange={(e) => setUsername(e.target.value)}
										className="block w-full pl-10 pr-3 py-2 border border-neutral-800 rounded-lg bg-neutral-900 text-white placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
										placeholder="Choose a username"
									/>
								</div>
							</div>

							<div>
								<label
									htmlFor="email"
									className="block text-sm font-medium text-neutral-300 mb-2"
								>
									Email
								</label>
								<div className="relative">
									<div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
										<Mail className="h-5 w-5 text-neutral-500" />
									</div>
									<input
										id="email"
										type="email"
										required
										value={email}
										onChange={(e) => setEmail(e.target.value)}
										className="block w-full pl-10 pr-3 py-2 border border-neutral-800 rounded-lg bg-neutral-900 text-white placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
										placeholder="you@example.com"
									/>
								</div>
							</div>

							<div>
								<label
									htmlFor="password"
									className="block text-sm font-medium text-neutral-300 mb-2"
								>
									Password
								</label>
								<div className="relative">
									<div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
										<Lock className="h-5 w-5 text-neutral-500" />
									</div>
									<input
										id="password"
										type="password"
										required
										value={password}
										onChange={(e) => setPassword(e.target.value)}
										className="block w-full pl-10 pr-3 py-2 border border-neutral-800 rounded-lg bg-neutral-900 text-white placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
										placeholder="••••••••"
									/>
								</div>
							</div>

							<div>
								<label
									htmlFor="confirm-password"
									className="block text-sm font-medium text-neutral-300 mb-2"
								>
									Confirm Password
								</label>
								<div className="relative">
									<div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
										<Lock className="h-5 w-5 text-neutral-500" />
									</div>
									<input
										id="confirm-password"
										type="password"
										required
										value={confirmPassword}
										onChange={(e) => setConfirmPassword(e.target.value)}
										className="block w-full pl-10 pr-3 py-2 border border-neutral-800 rounded-lg bg-neutral-900 text-white placeholder-neutral-500 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
										placeholder="••••••••"
									/>
								</div>
							</div>

							<div className="flex items-center">
								<input
									id="terms"
									type="checkbox"
									required
									className="h-4 w-4 rounded border-neutral-700 bg-neutral-900 text-purple-500 focus:ring-purple-500"
								/>
								<label
									htmlFor="terms"
									className="ml-2 block text-sm text-neutral-400"
								>
									I agree to the{" "}
									<Link
										to="/terms"
										className="text-purple-400 hover:text-purple-300"
									>
										Terms of Service
									</Link>{" "}
									and{" "}
									<Link
										to="/privacy"
										className="text-purple-400 hover:text-purple-300"
									>
										Privacy Policy
									</Link>
								</label>
							</div>

							<button
								type="submit"
								className="w-full py-2 px-4 border border-transparent rounded-lg shadow-sm text-white bg-gradient-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500 transition-colors duration-200"
							>
								Create Account
							</button>
						</form>

						<div className="mt-6">
							<div className="relative">
								<div className="absolute inset-0 flex items-center">
									<div className="w-full border-t border-neutral-800" />
								</div>
								<div className="relative flex justify-center text-sm">
									<span className="px-2 bg-neutral-900 text-neutral-400">
										Or continue with
									</span>
								</div>
							</div>

							<div className="mt-6 grid grid-cols-2 gap-3">
              <button
									type="button"
									className="w-full inline-flex justify-center py-2 px-4 border border-neutral-800 rounded-lg shadow-sm bg-neutral-900 text-neutral-300 hover:bg-neutral-800 transition-colors duration-200"
								>
									<FaGoogle className="h-5 w-5" />
									<span className="ml-2">Google</span>
								</button>
								<button
									type="button"
									className="w-full inline-flex justify-center py-2 px-4 border border-neutral-800 rounded-lg shadow-sm bg-neutral-900 text-neutral-300 hover:bg-neutral-800 transition-colors duration-200"
								>
									<FaGithub className="h-5 w-5" />
									<span className="ml-2">GitHub</span>
								</button>
							</div>
						</div>
					</div>

					{/* Sign In Link */}
					<div className="text-center">
						<p className="text-neutral-400">
							Already have an account?{" "}
							<Link
								to="/signin"
								className="text-purple-400 hover:text-purple-300 font-medium"
							>
								Sign in
							</Link>
						</p>
					</div>
				</div>
			</div>

			{/* Footer */}
			<footer className="bg-neutral-900 py-6 px-4 text-center text-neutral-400 text-sm">
				<p>
					Created and Copyrighted by Owen Perry and Connor Sample. All Rights
					Reserved © 2025.
				</p>
			</footer>
		</div>
	);
};

export default SignUpPage;
