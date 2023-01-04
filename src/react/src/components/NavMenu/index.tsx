import React, { FC, useState } from "react";
import {
  Button,
  Col,
  Collapse,
  Container,
  Navbar,
  NavbarToggler,
  NavItem,
  NavLink,
  Row,
} from "reactstrap";
import { Link } from "react-router-dom";
import "./styles.ts";
import { useAuth } from "react-oidc-context";
import { NavbarBrand } from "./styles";

export const NavMenu: FC = () => {
  const [collapsed, setCollapsed] = useState(true);

  const toggleNavbar = () => setCollapsed((current) => !current);

  const auth = useAuth();
  const given_name = auth?.user?.profile.given_name;

  return (
    <header>
      <Navbar
        className="navbar-expand-sm navbar-toggleable-sm ng-white border-bottom box-shadow mb-3"
        light
      >
        <Container>
          <Row >
              <Col>
                <NavbarBrand tag={Link} to="/">
                  React Client
                </NavbarBrand>
              </Col>
              <Col Align="right">
                <NavbarToggler onClick={toggleNavbar} className="mr-2" />
                <Collapse
                  className="d-sm-inline-flex flex-sm-row-reverse"
                  isOpen={!collapsed}
                  navbar
                >
                  <ul className="navbar-nav flex-grow">
                    <NavItem>
                      <NavLink tag={Link} className="text-dark" to="/">
                        Home
                      </NavLink>
                    </NavItem>
                    <NavItem>
                      <NavLink tag={Link} className="text-dark" to="/private">
                        Private
                      </NavLink>
                    </NavItem>
                    <NavItem>
                      <NavLink tag={Link} className="text-dark" to="/fetch-data">
                        Fetch data
                      </NavLink>
                    </NavItem>

                    {given_name ? (
                      <>
                        <h5 style={{ margin: "10px 10px" }}>Hello, {given_name}</h5>
                        <NavItem>
                          <NavLink
                            tag={Button}
                            color="warning"
                            onClick={() => auth.removeUser()}
                          >
                            Logout
                          </NavLink>
                        </NavItem>
                      </>
                    ) : (
                      <NavItem>
                        <NavLink
                          tag={Button}
                          color="warning"
                          onClick={() => auth.signinRedirect()}>
                          Login
                        </NavLink>
                      </NavItem>
                    )}
                  </ul>
                </Collapse>
              </Col>
          </Row>
          
        </Container>
      </Navbar>
    </header>
  );
};
