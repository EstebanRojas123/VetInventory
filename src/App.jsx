import { useState } from "react";
import {
  BrowserRouter as Router,
  Route,
  Routes,
  Navigate,
} from "react-router-dom";
import LoginSignup from "./components/Login/LoginSignup/LoginSignup";
import Panel from "./components/Panel/Panel";
import Loading from "./components/Login/Loading/Loading";
import LogoutLoading from "./components/Login/LogoutLoading/LogoutLoading"; // Importar el nuevo componente

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [emailUserCurrent, setEmailUserCurrent] = useState("");

  return (
    <Router>
      <Routes>
        <Route
          path="/"
          element={
            <LoginSignup
              setAuthenticated={setIsAuthenticated}
              setEmailUserCurrent={setEmailUserCurrent}
            />
          }
        />
        <Route path="/loading" element={<Loading />} />
        <Route path="/logout-loading" element={<LogoutLoading />} />{" "}
        {/* Nueva ruta */}
        <Route
          path="/panel"
          element={
            isAuthenticated ? (
              <Panel
                setAuthenticated={setIsAuthenticated}
                emailUserCurrent={emailUserCurrent}
              />
            ) : (
              <Navigate to="/" />
            )
          }
        />
      </Routes>
    </Router>
  );
}

export default App;
