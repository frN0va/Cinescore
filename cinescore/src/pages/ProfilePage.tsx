import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { User, Film, Clapperboard, Star, Popcorn, Search, Heart, Plus } from 'lucide-react';

interface RankedMovie {
  rank: number;
  title: string;
  poster: string;
  overallScore?: number;
  isLiked?: boolean;
  inWatchlist?: boolean;
}

const ProfilePage: React.FC = () => {
  const [activeNav, setActiveNav] = useState('Films');
  const [searchQuery, setSearchQuery] = useState('');
  
  const [rankedMovies, setRankedMovies] = useState<RankedMovie[]>([
    {
      rank: 1,
      title: 'Inception',
      poster: '/api/placeholder/200/300',
      overallScore: 4.7,
    },
    {
      rank: 2,
      title: 'The Matrix',
      poster: '/api/placeholder/200/300',
      overallScore: 4.5,
    },
    {
      rank: 3,
      title: 'Interstellar',
      poster: '/api/placeholder/200/300',
      overallScore: 4.8,
    },
  ]);

  const [likedMovies, setLikedMovies] = useState<RankedMovie[]>([
    {
      rank: 0,
      title: 'The Dark Knight',
      poster: '/api/placeholder/200/300',
      overallScore: 4.9,
      isLiked: true,
    },
    {
      rank: 0,
      title: 'Goodfellas',
      poster: '/api/placeholder/200/300',
      overallScore: 4.8,
      isLiked: true,
    },
  ]);

  const [watchlist, setWatchlist] = useState<RankedMovie[]>([
    {
      rank: 0,
      title: 'The Godfather',
      poster: '/api/placeholder/200/300',
      overallScore: 4.9,
      inWatchlist: true,
    },
    {
      rank: 0,
      title: 'Fight Club',
      poster: '/api/placeholder/200/300',
      overallScore: 4.7,
      inWatchlist: true,
    },
  ]);

  const navItems = [
    { name: 'Films', icon: <Film className="w-5 h-5" /> },
    { name: 'Actors', icon: <User className="w-5 h-5" /> },
    { name: 'Directors', icon: <Clapperboard className="w-5 h-5" /> },
  ];

  const MovieGrid: React.FC<{ movies: RankedMovie[], title: string }> = ({ movies, title }) => (
    <section className="mb-12">
      <h2 className="text-2xl font-bold mb-6 text-blue-300">{title}</h2>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {movies.map((movie, index) => (
          <div
            key={index}
            className="bg-neutral-900 rounded-lg overflow-hidden shadow-xl transform transition hover:scale-105 hover:shadow-2xl"
          >
            <div className="relative">
              <img
                src={movie.poster}
                alt={movie.title}
                className="w-full h-[450px] object-cover"
              />
              {movie.overallScore && (
                <div className="absolute top-4 right-4 bg-black/70 text-yellow-400 px-3 py-1 rounded-full flex items-center">
                  <Star className="w-5 h-5 mr-1" />
                  <span className="font-bold">{movie.overallScore.toFixed(1)}</span>
                </div>
              )}
              {movie.rank > 0 && (
                <div className="absolute top-4 left-4 bg-purple-600 text-white rounded-full w-12 h-12 flex items-center justify-center text-2xl font-bold">
                  #{movie.rank}
                </div>
              )}
            </div>
            <div className="p-4">
              <h2 className="text-xl font-semibold text-blue-300">{movie.title}</h2>
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
                <span className="text-lg font-bold text-white tracking-wider">Cinescore</span>
              </Link>
              <div className="flex space-x-4">
                {navItems.map((item) => (
                  <Link
                    key={item.name}
                    to={`/${item.name.toLowerCase()}`}
                    className={`flex items-center space-x-2 px-3 py-2 rounded-md transition-colors duration-300 ${
                      activeNav === item.name
                        ? 'bg-purple-400 text-white'
                        : 'text-neutral-300 hover:bg-neutral-800 hover:text-white'
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
        <div className="space-y-12">
          <MovieGrid title="My Ranked Movies" movies={rankedMovies} />
          <MovieGrid title="My Liked Movies" movies={likedMovies} />
          <MovieGrid title="My Watchlist" movies={watchlist} />
        </div>
      </main>
    </div>
  );
};

export default ProfilePage;