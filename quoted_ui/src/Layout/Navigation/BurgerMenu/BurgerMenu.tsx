import { useRef } from "react";

import clsx from "clsx";
import NavList, { NavItem } from "../NavList";

import styles from "./BurgerMenu.module.scss";

interface BurgerMenuProps {
  open: boolean;
  navItems: Array<NavItem>;
  onNavItemClicked?(navItem: NavItem): void;
}

function BurgerMenu({ open, navItems, onNavItemClicked }: BurgerMenuProps) {
  // A bit of a hack to suppress the inactive class being applied on load,
  // which causes the burger menu to slide out of view when it wasn't even open.
  const hasBeenOpen = useRef(false);

  if (open) {
    hasBeenOpen.current = true;
  }

  return (
    <>
      <div
        className={clsx({
          [styles["burger-menu"]]: true,
          [styles["burger-menu--active"]]: open,
          [styles["burger-menu--inactive"]]: hasBeenOpen.current && !open,
        })}
      >
        <NavList
          navItems={navItems}
          className={styles["burger-menu__nav-list"]}
          onNavItemClicked={onNavItemClicked}
        />
      </div>
    </>
  );
}

export default BurgerMenu;
