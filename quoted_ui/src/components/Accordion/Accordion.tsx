import React from "react";
import { IconCaretDown, IconCaretUp } from "@tabler/icons-react";
import clsx from "clsx";

import useToggle from "../../hooks/useToggle";

import styles from "./Accordion.module.scss";

interface AccordionProps {
  title: string;
  actionIcon?: React.JSX.Element;
  children?: React.ReactNode;
  depth?: number;
  onExpanded?(): void;
  onCollapse?(): void;
}

function Accordion({
  title,
  actionIcon,
  children,
  onExpanded,
  onCollapse,
}: AccordionProps) {
  const [open, { toggle }] = useToggle();

  function handleToggle(e: React.MouseEvent<HTMLDivElement>) {
    e.stopPropagation();
    if (open) {
      onCollapse?.();
    } else {
      onExpanded?.();
    }

    toggle();
  }

  return (
    <div
      className={clsx({
        [styles["accordion"]]: true,
        [styles["accordion--open"]]: open,
        [styles["accordion--closed"]]: !open,
      })}
    >
      <div
        className={clsx({
          [styles["accordion__header"]]: true,
          [styles["accordion__header--open"]]: open,
          [styles["accordion__header--closed"]]: !open,
        })}
        onClick={handleToggle}
      >
        <div className={styles["accordion__title-primary"]}>{title}</div>
        <div className={styles["accordion__title-secondary"]}>
          {actionIcon}
          <div className={styles["accordion__expand-toggler"]}>
            {open ? <IconCaretUp /> : <IconCaretDown />}
          </div>
        </div>
      </div>
      <div
        className={clsx({
          [styles["accordion__content"]]: true,
          [styles["accordion__content--open"]]: open,
          [styles["accordion__content--closed"]]: !open,
        })}
      >
        {open && children}
      </div>
    </div>
  );
}

export default Accordion;
