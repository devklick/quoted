import { useNavigate } from "react-router-dom";

import Button from "../../components/Button";
import Github from "../../assets/github.svg";
import Quoted from "../../assets/quoted-no-bg.svg";

import { BurgerMenuIcon } from "../Navigation/BurgerMenu";

import styles from "./Header.module.scss";
import routeDefinitions from "../../route-definitions";

interface HeaderProps {
  burgerOpen: boolean;
  toggleBurger(): void;
}

function Header({ burgerOpen, toggleBurger }: HeaderProps) {
  const nav = useNavigate();
  return (
    <header className={styles["header"]}>
      <div
        className={styles["header__title"]}
        onClick={() => nav(routeDefinitions.home.path)}
      >
        <h1 className={styles["header__text"]}>Quoted</h1>
        <img src={Quoted} className={styles["icon-quoted"]} />
      </div>

      <div className={styles["header__right"]}>
        <a href="https://github.com/devklick/quoted" target="_blank">
          <Button type="ghost">
            <img src={Github} className={styles["icon-github"]} />
          </Button>
        </a>
        <Button type="ghost">
          <BurgerMenuIcon open={burgerOpen} onClick={toggleBurger} />
        </Button>
      </div>
    </header>
  );
}

export default Header;
