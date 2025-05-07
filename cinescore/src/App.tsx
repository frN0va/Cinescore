import { BrowserRouter, Routes, Route } from "react-router-dom";
import HomePage from "./pages/HomePage";
import ProfilePage from "./pages/ProfilePage";
import MoviePage from "./pages/MoviePage";
import ActorsPage from "./pages/ActorsPage";
import ActorBrowse from "./pages/ActorBrowse";
import SearchResults from "./pages/SearchResults";
import ActorSearchResults from "./pages/ActorSearchResults";
import SignInPage from "./pages/SignInPage";
import SignUpPage from "./pages/SignUpPage";
import ForgotPasswordPage from "./pages/ForgotPasswordPage";
import TierListPage from "./pages/TierListPage";

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
				<Route path="/signin" element={<SignInPage />} />
				<Route path="/signup" element={<SignUpPage />} />
				<Route path="/forgot-password" element={<ForgotPasswordPage />} />
				<Route path="/tierlist" element={<TierListPage />} />
			</Routes>
		</BrowserRouter>
	);
}

export default App;
