import React, { FC } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Securedpage from "./pages/Securedpage";
import { Layout } from "./components/Layout";
import { Home } from "./components/Home";
import Privated from "./pages/Privatedpage";
import { OidcSecure } from "@axa-fr/react-oidc";

const App: FC = () => {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route
            path="/private"
            element={
              <OidcSecure>
                <Privated />
              </OidcSecure>
            }
          />
          <Route
            path="/fetch-data"
            element={
              <OidcSecure>
                <Securedpage />
              </OidcSecure>
            }
          />
        </Routes>
      </Layout>
    </BrowserRouter>
  );
};

export default App;