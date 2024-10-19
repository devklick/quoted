import { NavLink } from "react-router-dom";
import clsx from "clsx";
import styles from "./NavList.module.scss";

export interface NavItem {
  title: string;
  path: string;
}

interface NavListProps {
  navItems: Array<NavItem>;
  className?: string;
  onNavItemClicked?(navItem: NavItem): void;
}

function NavList({ navItems, className, onNavItemClicked }: NavListProps) {
  return (
    <ul
      className={clsx({
        [styles["nav-list"]]: true,
        [className ?? ""]: !!className,
      })}
    >
      {navItems.map(({ title, path }) => (
        <li key={`nav-item-${title}`} className={styles["nav-item"]}>
          <NavLink
            to={path}
            onClick={() => onNavItemClicked?.({ path, title })}
            className={({ isActive }) =>
              clsx({
                [styles["nav-item__link"]]: true,
                [styles["nav-item__link--active"]]: isActive,
              })
            }
          >
            {title}
          </NavLink>
        </li>
      ))}
    </ul>
  );
}

export default NavList;
