import NavList, { NavItem } from "../NavList";
import styles from "./SideNav.module.scss";

interface SideNavProps {
  navItems: Array<NavItem>;
}

function SideNav({ navItems }: SideNavProps) {
  return (
    <nav className={styles["side-nav"]}>
      <NavList navItems={navItems} />
    </nav>
  );
}

export default SideNav;
