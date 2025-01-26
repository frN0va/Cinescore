import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { Film, User, Clapperboard, Star, Popcorn } from 'lucide-react';
import MoviePoster from '../components/MoviePoster';

const movieCategories = {
  'Trending Now': [
    {
      id: 1,
      title: 'Inception',
      poster: '/api/placeholder/200/300',
      description: 'A mind-bending sci-fi thriller about dream infiltration.',
      rank: 0,
      overallScore: 4.7,
    },
    {
      id: 2,
      title: 'The Matrix',
      poster: '/api/placeholder/200/300',
      description: 'A computer programmer discovers the world is a simulation.',
      rank: 0,
      overallScore: 4.5,
    },
    {
      id: 3,
      title: 'Interstellar',
      poster: '/api/placeholder/200/300',
      description: 'A journey through space and time to save humanity.',
      rank: 0,
      overallScore: 4.8,
    },
  ],
  'Top Rated': [
    {
      id: 4,
      title: 'The Shawshank Redemption',
      poster: '/api/placeholder/200/300',
      description: 'A story of hope and friendship in prison.',
      rank: 0,
      overallScore: 4.9,
    },
    {
      id: 5,
      title: 'Pulp Fiction',
      poster: '/api/placeholder/200/300',
      description: 'Interconnected stories of Los Angeles criminals.',
      rank: 0,
      overallScore: 4.6,
    },
  ],
};

const HomePage: React.FC = () => {
  const [movies, setMovies] = useState(movieCategories);
  const [activeNav, setActiveNav] = useState('Films');

  const handleRankMovie = (movieId: number, rank: number) => {
    const updatedMovies = { ...movies };
    Object.keys(updatedMovies).forEach((category) => {
      updatedMovies[category] = updatedMovies[category].map((movie) =>
        movie.id === movieId ? { ...movie, rank } : movie
      );
    });
    setMovies(updatedMovies);
  };

  const navItems = [
    { name: 'Films', icon: <Film className="w-5 h-5" /> },
    { name: 'Actors', icon: <User className="w-5 h-5" /> },
    { name: 'Directors', icon: <Clapperboard className="w-5 h-5" /> },
  ];

  return (
    <div className="w-full min-h-screen bg-neutral-950 text-white">
      <nav className="fixed top-0 left-0 right-0 z-50 bg-neutral-900/90 backdrop-blur-sm shadow-lg">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center">
              <Link to="/" className="flex items-center">
                <Popcorn className="w-6 h-6 mr-2 text-purple-400" />
                <span className="text-lg font-bold text-white tracking-wider">Cinescore</span>
              </Link>
            </div>
            <div className="flex space-x-6">
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
            <div>
              <Link 
                to="/profile" 
                className="hover:bg-neutral-800 p-2 rounded-full transition duration-200"
              >
                <User className="w-6 h-6 text-purple-400 hover:text-purple-300 transition" />
              </Link>
            </div>
          </div>
        </div>
      </nav>

      <main className="pt-20 px-6 pb-6 max-w-7xl mx-auto">
        {Object.entries(movies).map(([category, categoryMovies]) => (
          <section key={category} className="mb-12">
            <h2 className="text-2xl font-semibold mb-6 text-blue-300 border-b border-neutral-800 pb-2">
              {category}
            </h2>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
              {categoryMovies.map((movie) => (
                <MoviePoster 
                  key={movie.id} 
                  movie={movie} 
                  onRank={handleRankMovie} 
                />
              ))}
            </div>
          </section>
        ))}
      </main>
    </div>
  );
};

export default HomePage;