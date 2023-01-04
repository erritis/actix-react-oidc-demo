import React, { FC } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import PrivateRoute from "./helpers/PrivateRoute";
import Securedpage from "./pages/Securedpage";
import { Layout } from "./components/Layout";
import { Home } from "./components/Home";
import Privated from "./pages/Privatedpage";

const App: FC = () => {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route
            path="/private"
            element={
              <PrivateRoute>
                <Privated />
              </PrivateRoute>
            }
          />
          <Route
            path="/fetch-data"
            element={
              <PrivateRoute>
                <Securedpage />
              </PrivateRoute>
            }
          />
        </Routes>
      </Layout>
    </BrowserRouter>
  );
};

export default App;
