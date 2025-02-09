import type React from "react";
import { useState } from "react";
import { Link } from "react-router-dom";
import {
	Popcorn,
	Mail,
	Film,
	User,
	Clapperboard,
	Search,
	ArrowLeft,
} from "lucide-react";
import { SearchDropdown } from "../components/SearchDropdown";
import ProfileDropdown from "../components/ProfileDropdown";
import type { Movie } from "../types";

const ForgotPasswordPage = () => {
	const [email, setEmail] = useState("");
	const [isSubmitted, setIsSubmitted] = useState(false);
	const [activeNav, setActiveNav] = useState("Films");
	const [searchQuery, setSearchQuery] = useState("");
	const [searchResults] = useState<Movie[]>([]);
	const [isSearching] = useState(false);

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault();
		setIsSubmitted(true);
		// Handle password reset logic here
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

			{/* Back to Sign In */}
			<div className="fixed top-20 left-4 z-40">
				<Link
					to="/signin"
					className="flex items-center text-neutral-400 hover:text-neutral-300 transition-colors duration-200 bg-neutral-900/90 px-4 py-2 rounded-lg backdrop-blur-sm"
				>
					<ArrowLeft className="h-4 w-4 mr-2" />
					<span>Back to Sign In</span>
				</Link>
			</div>

			{/* Forgot Password Content */}
			<div className="flex-grow flex items-center justify-center px-4 py-12 mt-16">
				<div className="max-w-md w-full space-y-8">
					{/* Header */}
					<div className="text-center">
						<h2 className="text-3xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
							Reset Your Password
						</h2>
						<p className="mt-2 text-neutral-400">
							Enter your email address and we'll send you instructions to reset
							your password.
						</p>
					</div>

					{/* Reset Password Form */}
					<div className="bg-neutral-900/50 p-8 rounded-xl shadow-xl border border-neutral-800">
						{!isSubmitted ? (
							<form onSubmit={handleSubmit} className="space-y-6">
								<div>
									<label
										htmlFor="email"
										className="block text-sm font-medium text-neutral-300 mb-2"
									>
										Email Address
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

								<button
									type="submit"
									className="w-full py-2 px-4 border border-transparent rounded-lg shadow-sm text-white bg-gradient-to-r from-blue-500 to-purple-500 hover:from-blue-600 hover:to-purple-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500 transition-colors duration-200"
								>
									Send Reset Instructions
								</button>
							</form>
						) : (
							<div className="text-center space-y-4">
								<div className="bg-green-500/10 text-green-400 p-4 rounded-lg">
									<p>
										If an account exists for {email}, you will receive password
										reset instructions.
									</p>
								</div>
								<p className="text-neutral-400">
									Didn't receive the email? Check your spam folder or{" "}
									<button type="button"
										onClick={() => setIsSubmitted(false)}
										className="text-purple-400 hover:text-purple-300"
									>
										try again
									</button>
								</p>
							</div>
						)}
					</div>

					{/* Sign Up Link */}
					<div className="text-center">
						<p className="text-neutral-400">
							Don't have an account?{" "}
							<Link
								to="/signup"
								className="text-purple-400 hover:text-purple-300 font-medium"
							>
								Sign up
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

export default ForgotPasswordPage;
