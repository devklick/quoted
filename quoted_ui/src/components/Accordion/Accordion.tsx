import React, { useRef } from "react";
import { IconCaretDown, IconCaretUp } from "@tabler/icons-react";
import clsx from "clsx";

import useToggle from "../../hooks/useToggle";
import { useScrollContentTo } from "../../Layout/Content";

import styles from "./Accordion.module.scss";

interface AccordionProps {
  title: string;
  actionIcon?: React.JSX.Element;
  children?: React.ReactNode;
  scrollToViewEnabled?: boolean;
  onExpanded?(): void;
  onCollapse?(): void;
}

function Accordion({
  title,
  actionIcon,
  children,
  scrollToViewEnabled,
  onExpanded,
  onCollapse,
}: AccordionProps) {
  const [open, { toggle }] = useToggle();
  const ref = useRef<HTMLDivElement>(null);
  const scrollContentTo = useScrollContentTo({
    elementRef: ref,
    enabled: scrollToViewEnabled,
  });

  // TODO: Some undesired behavior to fix:
  // Collapsing a nested accordion currently causes us to scroll so that the
  // parent accordion is at the top of the screen. Instead, collapsing a child
  // accordion should not cause any auto scrolling at all.

  function handleToggle(e: React.MouseEvent<HTMLDivElement>) {
    e.stopPropagation();
    if (open) {
      scrollContentTo.disable();
      onCollapse?.();
    } else {
      scrollContentTo.enable();
      onExpanded?.();
    }

    toggle();
  }

  return (
    <div
      ref={ref}
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
