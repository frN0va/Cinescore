import { BrowserRouter, Routes, Route } from "react-router-dom";
import HomePage from "./pages/HomePage";
import ProfilePage from "./pages/ProfilePage";
import MoviePage from "./pages/MoviePage";
import ActorsPage from "./pages/ActorsPage";

function App() {
	return (
		<BrowserRouter>
			<Routes>
				<Route path="/" element={<HomePage />} />
				<Route path="/films" element={<HomePage />} />
				<Route path="/profile" element={<ProfilePage />} />
				<Route path="/movie/:id" element={<MoviePage />} />
				<Route path="/actors" element={<ActorsPage />} />
			</Routes>
		</BrowserRouter>
	);
}

export default App;
