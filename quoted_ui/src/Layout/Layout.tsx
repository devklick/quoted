import { Outlet, useLocation } from "react-router-dom";

import SideNav from "./Navigation/SideNav/SideNav";
import Header from "./Header/Header";
import Content from "./Content/Content";
import BurgerMenu from "./Navigation/BurgerMenu";
import useToggle from "../hooks/useToggle";
import { NavItem } from "./Navigation/NavList";
import PageOverlay from "../components/PageOverlay";

import styles from "./Layout.module.scss";

interface LayoutProps {
  navItems: Array<NavItem>;
}

function Layout({ navItems }: LayoutProps) {
  const location = useLocation();
  const [burgerOpen, { toggle: toggleBurger }] = useToggle();
  const firstNavItemActive = location.pathname === navItems[0].path;
  return (
    <div className={styles["layout"]}>
      <BurgerMenu
        open={burgerOpen}
        navItems={navItems}
        onNavItemClicked={() => toggleBurger()}
      />
      <Header burgerOpen={burgerOpen} toggleBurger={toggleBurger} />
      <div className={styles["main"]}>
        {<PageOverlay enabled={burgerOpen} />}
        <SideNav navItems={navItems} />
        <Content disableBorderRadius={firstNavItemActive}>
          <Outlet />
        </Content>
      </div>
    </div>
  );
}

export default Layout;