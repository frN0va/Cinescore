import type React from "react";
import { useState, useEffect, useRef } from "react";
import { Link } from "react-router-dom";
import { User, LogOut, UserCircle } from "lucide-react";

interface ProfileDropdownProps {
	isAuthenticated?: boolean;
	onLogout?: () => void;
}

const ProfileDropdown: React.FC<ProfileDropdownProps> = ({
	isAuthenticated = false,
	onLogout = () => {},
}) => {
	const [isOpen, setIsOpen] = useState(false);
	const dropdownRef = useRef<HTMLDivElement>(null);

	useEffect(() => {
		const handleClickOutside = (event: MouseEvent) => {
			if (
				dropdownRef.current &&
				!dropdownRef.current.contains(event.target as Node)
			) {
				setIsOpen(false);
			}
		};

		document.addEventListener("mousedown", handleClickOutside);
		return () => document.removeEventListener("mousedown", handleClickOutside);
	}, []);

	return (
		<div className="relative" ref={dropdownRef}>
			<button type="button"
				onClick={() => setIsOpen(!isOpen)}
				className="rounded-full p-2 transition duration-200 hover:bg-neutral-800"
			>
				<User className="h-6 w-6 text-purple-400 transition hover:text-purple-300" />
			</button>

			{isOpen && (
				<div className="absolute right-0 mt-2 w-48 rounded-lg border border-neutral-800 bg-neutral-900 py-2 shadow-lg">
					<Link
						to="/profile"
						className="flex items-center px-4 py-2 text-neutral-300 hover:bg-neutral-800"
						onClick={() => setIsOpen(false)}
					>
						<UserCircle className="mr-2 h-4 w-4" />
						Profile
					</Link>

					{isAuthenticated ? (
						<button type="button"
							onClick={() => {
								onLogout();
								setIsOpen(false);
							}}
							className="flex w-full items-center px-4 py-2 text-red-400 hover:bg-neutral-800"
						>
							<LogOut className="mr-2 h-4 w-4" />
							Log Out
						</button>
					) : (
						<Link
							to="/signin"
							className="flex items-center px-4 py-2 text-neutral-300 hover:bg-neutral-800"
							onClick={() => setIsOpen(false)}
						>
							<User className="mr-2 h-4 w-4" />
							Sign In
						</Link>
					)}
				</div>
			)}
		</div>
	);
};

export default ProfileDropdown;
