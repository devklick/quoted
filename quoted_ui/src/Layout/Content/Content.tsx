import React from "react";
import clsx from "clsx";

import styles from "./Content.module.scss";

interface ContentProps {
  children: React.ReactNode;
  /**
   * Whether or not the first nav item within the SideNav component is currently active.
   */
  disableBorderRadius: boolean;
}

function Content({ children, disableBorderRadius }: ContentProps) {
  return (
    <div
      className={clsx({
        [styles["content"]]: true,
        [styles["content--no-radius"]]: disableBorderRadius,
      })}
    >
      {children}
    </div>
  );
}

export default Content;
