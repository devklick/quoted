import clsx from "clsx";

import styles from "./PageOverlay.module.scss";

interface PageOverlayProps {
  enabled: boolean;
}

function PageOverlay({ enabled }: PageOverlayProps) {
  return (
    <div
      className={clsx({
        [styles["page-overlay"]]: true,
        [styles["page-overlay--active"]]: enabled,
        [styles["page-overlay--inactive"]]: !enabled,
      })}
    ></div>
  );
}

export default PageOverlay;
