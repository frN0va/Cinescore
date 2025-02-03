import { BrowserRouter, Routes, Route } from "react-router-dom";
import HomePage from "./pages/HomePage";
import ProfilePage from "./pages/ProfilePage";
import MoviePage from "./pages/MoviePage";
import ActorsPage from "./pages/ActorsPage";
import ActorBrowse from "./pages/ActorBrowse";
import SearchResults from "./pages/SearchResults";
import ActorSearchResults from "./pages/ActorSearchResults";

function App() {
	return (
		<BrowserRouter>
			<Routes>
				<Route path="/" element={<HomePage />} />
				<Route path="/films" element={<HomePage />} />
				<Route path="/profile" element={<ProfilePage />} />
				<Route path="/movie/:id" element={<MoviePage />} />
				<Route path="/actors/:id" element={<ActorsPage />} />
				<Route path="/actors" element={<ActorBrowse />} />
				<Route path="/search" element={<SearchResults />} />
				<Route path="/search/actors" element={<ActorSearchResults />} />
			</Routes>
		</BrowserRouter>
	);
}

export default App;
