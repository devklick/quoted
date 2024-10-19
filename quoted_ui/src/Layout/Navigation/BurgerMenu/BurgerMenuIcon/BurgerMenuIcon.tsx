import { IconMenu2, IconX } from "@tabler/icons-react";

import styles from "./BurgerMenuIcon.module.scss";

interface BurgerMenuIconProps {
  open: boolean;
  onClick(): void;
}

function BurgerMenuIcon({ open, onClick }: BurgerMenuIconProps) {
  return (
    <div className={styles["burger-menu"]} onClick={onClick}>
      {!open && <IconMenu2 className={styles["burger-menu__icon"]} />}
      {open && <IconX className={styles["burger-menu__icon"]} />}
    </div>
  );
}

export default BurgerMenuIcon;
