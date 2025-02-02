import type React from "react";

interface Actor {
	id: number;
	name: string;
	image: string;
	department: string;
	popularity: number;
	imdb?: string;
}

interface ActorsCardProps {
	actor: Actor;
	onClick?: (actorId: number) => void;
}

export const ActorsCard: React.FC<ActorsCardProps> = ({ actor, onClick }) => {
	return (
		<div
			className="group cursor-pointer overflow-hidden rounded-lg bg-neutral-900 transition-transform hover:scale-105"
			onClick={() => onClick?.(actor.id)}
		>
			<div className="relative aspect-[2/3]">
				<img
					src={actor.image}
					alt={actor.name}
					className="h-full w-full object-cover"
				/>
				<div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />
				<div className="absolute bottom-0 p-4">
					<h3 className="text-lg font-semibold text-white">{actor.name}</h3>
					<p className="text-sm text-neutral-400">
						{actor.department} • {actor.popularity.toFixed(1)} popularity
					</p>
				</div>
			</div>
		</div>
	);
};

interface ModalProps {
	isOpen: boolean;
	onClose: () => void;
	children: React.ReactNode;
}

export const Modal: React.FC<ModalProps> = ({ isOpen, onClose, children }) => {
	if (!isOpen) return null;

	return (
		<>
			<div
				className="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm"
				onClick={onClose}
			/>
			<div className="fixed left-1/2 top-1/2 z-50 w-full max-w-4xl -translate-x-1/2 -translate-y-1/2">
				{children}
			</div>
		</>
	);
};
