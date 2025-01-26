import React, { useState } from 'react';
import { Star } from 'lucide-react';

interface MoviePosterProps {
  movie: {
    id: number;
    title: string;
    poster: string;
    description: string;
    rank: number;
    overallScore?: number;
  };
  onRank: (id: number, rank: number) => void;
}

const MoviePoster: React.FC<MoviePosterProps> = ({ movie, onRank }) => {
  const [showDetails, setShowDetails] = useState(false);
  const [hoveredRank, setHoveredRank] = useState(0);

  return (
    <div
      className="relative group overflow-hidden rounded-lg shadow-xl transition-all duration-300 ease-in-out"
      onMouseEnter={() => setShowDetails(true)}
      onMouseLeave={() => setShowDetails(false)}
    >
      <div className="relative">
        <img
          src={movie.poster}
          alt={movie.title}
          className="w-full h-[300px] object-cover transition-transform duration-300 group-hover:scale-105"
        />
        {movie.overallScore !== undefined && (
          <div className="absolute top-3 right-3 bg-black/70 text-yellow-400 px-3 py-1 rounded-full flex items-center">
            <Star className="w-5 h-5 mr-1" />
            <span className="font-bold">{movie.overallScore.toFixed(1)}</span>
          </div>
        )}
      </div>
      {showDetails && (
        <div className="absolute inset-0 bg-black/80 text-white p-4 flex flex-col justify-between opacity-0 group-hover:opacity-100 transition-opacity duration-300">
          <div>
            <h3 className="text-2xl font-bold mb-2">{movie.title}</h3>
            <p className="text-sm">{movie.description}</p>
          </div>
          <div className="flex justify-center space-x-2 mt-4">
            {[1, 2, 3, 4, 5].map((rank) => (
              <Star
                key={rank}
                className={`w-8 h-8 cursor-pointer transition-colors ${
                  rank <= hoveredRank ? 'text-yellow-400' : 'text-gray-500'
                } hover:text-yellow-300`}
                onMouseEnter={() => setHoveredRank(rank)}
                onMouseLeave={() => setHoveredRank(movie.rank)}
                onClick={() => onRank(movie.id, rank)}
              />
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default MoviePoster;