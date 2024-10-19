import { useNavigate } from "react-router-dom";

import Button from "../../components/Button";
import Github from "../../assets/github.svg";

import { BurgerMenuIcon } from "../Navigation/BurgerMenu";

import styles from "./Header.module.scss";

interface HeaderProps {
  burgerOpen: boolean;
  toggleBurger(): void;
}

function Header({ burgerOpen, toggleBurger }: HeaderProps) {
  const nav = useNavigate();
  return (
    <header className={styles["header"]}>
      <h1 onClick={() => nav("/")} className={styles["header__text"]}>
        Quoted
      </h1>
      <div className={styles["header__right"]}>
        <Button
          type="ghost"
          onClick={() => open("https://github.com/devklick/quoted")}
        >
          <img src={Github} className={styles["icon-github"]} />
        </Button>
        <BurgerMenuIcon open={burgerOpen} onClick={toggleBurger} />
      </div>
    </header>
  );
}

export default Header;
